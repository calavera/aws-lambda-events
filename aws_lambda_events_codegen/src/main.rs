use go_to_rust;
#[macro_use]
extern crate quicli;
use codegen;

use quicli::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
struct ExampleEvent {
    name: String,
    content: String,
    event_type: String,
}

#[derive(Debug)]
struct ParsedEventFile {
    service_name: String,
    path: PathBuf,
    go: go_to_rust::GoCode,
    rust: go_to_rust::RustCode,
    example_events: Vec<ExampleEvent>,
}

/// Generate rust code for AWS lambda events sourced from `aws-go-sdk`
#[derive(Debug, StructOpt)]
#[structopt(author = "")]
struct Cli {
    /// Path to `aws-go-sdk` checkout
    #[structopt(long = "input", name = "AWS_GO_SDK_DIRECTORY", parse(from_os_str))]
    sdk_location: PathBuf,
    /// Output directory
    #[structopt(long = "output", short = "o", name = "DIRECTORY", parse(from_os_str))]
    output_location: PathBuf,
    /// Overwrite existing files
    #[structopt(long = "overwrite")]
    overwrite: bool,
    /// Verbose output. Pass many times for more log output
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

fn get_blacklist() -> HashSet<String> {
    let mut blacklist = HashSet::new();
    // https://github.com/aws/aws-lambda-go/blob/master/events/attributevalue.go
    blacklist.insert("attributevalue".to_string());
    // https://github.com/aws/aws-lambda-go/blob/master/events/duration.go
    blacklist.insert("duration".to_string());
    // https://github.com/aws/aws-lambda-go/blob/master/events/dynamodb.go
    blacklist.insert("dynamodb".to_string());
    // https://github.com/aws/aws-lambda-go/blob/master/events/epoch_time.go
    blacklist.insert("epoch_time".to_string());
    blacklist
}

fn overwrite_warning(path: &PathBuf, overwrite: bool) -> Option<()> {
    if path.exists() && !overwrite {
        error!(
            "File already exists and `--overwrite` not specified. Skipping: {}",
            path.to_string_lossy()
        );
        return Some(());
    }
    None
}

fn write_mod_index(
    mod_path: &PathBuf,
    parsed_files: &[ParsedEventFile],
    overwrite: bool,
) -> Result<()> {
    if overwrite_warning(&mod_path, overwrite).is_none() {
        let mut mod_content: Vec<String> = Vec::new();
        for parsed in parsed_files {
            mod_content.push(format!(
                "/// AWS Lambda event definitions for {}.",
                parsed.service_name
            ));
            mod_content.push(format!(
                "pub mod {};",
                parsed
                    .path
                    .file_stem()
                    .expect("file stem")
                    .to_string_lossy()
            ));
        }
        let mut f = File::create(mod_path)?;
        f.write_all(mod_content.join("\n").as_bytes())?;
        f.write_all(b"\n")?;
    }
    Ok(())
}

fn write_readme(readme_path: &PathBuf, git_hash: &str, overwrite: bool) -> Result<()> {
    if overwrite_warning(&readme_path, overwrite).is_none() {
        let version_text = format!(
            "Generated from commit [{}](https://github.com/aws/aws-lambda-go/commit/{}).",
            git_hash, git_hash,
        );
        let mut content: Vec<&str> = Vec::new();
        content.push("# AWS lambda event types.");
        content.push("");
        content.push("These types are automatically generated from the");
        content.push("[official Go SDK](https://github.com/aws/aws-lambda-go/tree/master/events).");
        content.push("");
        content.push(&version_text);
        let mut f = File::create(readme_path)?;
        f.write_all(content.join("\n").as_bytes())?;
        f.write_all(b"\n")?;
    }
    Ok(())
}

fn fuzz(string: &mut String) {
    string.retain(|c| c != '_' && c != '-')
}

fn get_fuzzy_file_listing(dir_path: &Path) -> Result<HashMap<String, PathBuf>> {
    let mut listing = HashMap::new();
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let original_path = entry.path().strip_prefix(dir_path)?.to_owned();
        let mut fuzzy_path = original_path.to_string_lossy().to_string();
        fuzz(&mut fuzzy_path);
        listing.insert(fuzzy_path, original_path);
    }
    Ok(listing)
}

fn find_example_events(
    fuzzy_files: &HashMap<String, PathBuf>,
    service_name: &str,
    example_event_path: &Path,
    scope: &codegen::Scope,
) -> Vec<ExampleEvent> {
    let mut name_with_quirks = match service_name {
        "codepipeline_job" => "codepipeline-job-event.json".to_string(),
        "firehose" => "kinesis-firehose-event.json".to_string(),
        service_name => format!("{}-event.json", service_name),
    };
    fuzz(&mut name_with_quirks);
    trace!(
        "Looking for example event: {} - {}",
        service_name,
        name_with_quirks
    );

    let mut examples = vec![];
    if let Some(file) = fuzzy_files.get(&name_with_quirks) {
        info!(
            "Found example event for service {} at: {}",
            service_name,
            file.to_string_lossy()
        );
        let content = read_example_event(&example_event_path.join(&file));
        let mut event_type = None;

        for item in scope.items() {
            match item {
                codegen::Item::Struct(s) if s.ty().name().ends_with("Event") => {
                    event_type = Some(s.ty().name());
                    break;
                }
                codegen::Item::Struct(s)
                    if s.ty().name().as_str() == "ApiGatewayProxyRequest"
                        && service_name == "apigw" =>
                {
                    event_type = Some(s.ty().name());
                    break;
                }
                _ => {}
            }
        }

        if let Some(event_type) = event_type {
            examples.push(ExampleEvent {
                name: format!("example-{}-event.json", &service_name),
                content,
                event_type: event_type.clone(),
            });
        }
    };

    if let Some(extra_examples) =
        find_custom_examples(service_name, fuzzy_files, example_event_path)
    {
        examples.extend(extra_examples);
    }

    examples
}

fn find_custom_examples(
    service_name: &str,
    fuzzy_files: &HashMap<String, PathBuf>,
    example_event_path: &Path,
) -> Option<Vec<ExampleEvent>> {
    let files = match service_name {
        "apigw" => &[
            (
                "apigw-custom-auth-request-type-request.json",
                "ApiGatewayCustomAuthorizerRequestTypeRequest",
            ),
            (
                "apigw-custom-auth-request.json",
                "ApiGatewayCustomAuthorizerRequest",
            ),
            (
                "apigw-custom-auth-response.json",
                "ApiGatewayCustomAuthorizerResponse",
            ),
            ("apigw-request.json", "ApiGatewayProxyRequest"),
            ("apigw-response.json", "ApiGatewayProxyResponse"),
            (
                "apigw-restapi-openapi-request.json",
                "ApiGatewayProxyRequest",
            ),
            ("apigw-v2-request-iam.json", "ApiGatewayV2httpRequest"),
            (
                "apigw-v2-request-jwt-authorizer.json",
                "ApiGatewayV2httpRequest",
            ),
            (
                "apigw-v2-request-lambda-authorizer.json",
                "ApiGatewayV2httpRequest",
            ),
            (
                "apigw-v2-request-no-authorizer.json",
                "ApiGatewayV2httpRequest",
            ),
            (
                "apigw-websocket-request.json",
                "ApiGatewayWebsocketProxyRequest",
            ),
        ],
        _ => return None,
    };

    let mut examples = vec![];
    for (name, event_type) in files {
        let mut filename = name.to_string();
        fuzz(&mut filename);

        if let Some(file) = fuzzy_files.get(&filename) {
            info!(
                "Found example event for service {} at: {}",
                service_name,
                file.to_string_lossy()
            );
            let content = read_example_event(&example_event_path.join(&file));
            examples.push(ExampleEvent {
                name: name.to_string(),
                content,
                event_type: event_type.to_string(),
            });
        }
    }

    if !examples.is_empty() {
        Some(examples)
    } else {
        None
    }
}

fn read_example_event(test_fixture: &PathBuf) -> String {
    let mut f = File::open(test_fixture).expect("fixture not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the fixture");
    debug!("Example event content: {}", contents);
    contents
}

fn write_fixture(example_event: &ExampleEvent, out_dir: &PathBuf, overwrite: bool) -> Result<()> {
    // Write the example event to the output location.
    let full = out_dir.join("fixtures").join(&example_event.name);
    {
        let parent = full.parent().expect("parent directory");
        if !parent.exists() {
            trace!("Creating fixture directory: {:?}", parent);
            create_dir(&parent)?;
        }
    }
    if overwrite_warning(&full, overwrite).is_none() {
        let mut f = File::create(full)?;
        f.write_all(example_event.content.as_bytes())?;
        f.write_all(b"\n")?;
    }
    Ok(())
}

fn generate_test_module(example_events: &[ExampleEvent]) -> Result<codegen::Module> {
    let mut test_module = codegen::Module::new("test");
    test_module.annotation(vec!["cfg(test)"]);
    test_module.import("super", "*");
    test_module.scope().raw("extern crate serde_json;");

    for e in example_events {
        let name = e.name.trim_end_matches(".json").replace("-", "_");
        let path = PathBuf::from("fixtures").join(&e.name);
        let test_function = generate_test_function(&name, &e.event_type, path);

        test_module.scope().push_fn(test_function);
    }

    Ok(test_module)
}

fn generate_test_function(
    fn_name: &str,
    toplevel_type: &str,
    relative: PathBuf,
) -> codegen::Function {
    let mut test_function = codegen::Function::new(fn_name);
    test_function.annotation(vec!["test"]);
    // Include the fixture content.
    test_function.line(format!(
        r#"let data = include_bytes!("{}");"#,
        relative.to_string_lossy(),
    ));
    // Deserialize.
    test_function.line(format!(
        r#"let parsed: {} = serde_json::from_slice(data).unwrap();"#,
        toplevel_type,
    ));
    // Serialize.
    test_function.line(String::from(
        r#"let output: String = serde_json::to_string(&parsed).unwrap();"#,
    ));
    // Deserialize.
    test_function.line(format!(
        r#"let reparsed: {} = serde_json::from_slice(output.as_bytes()).unwrap();"#,
        toplevel_type,
    ));
    // Compare.
    test_function.line(String::from(r#"assert_eq!(parsed, reparsed);"#));
    test_function
}

main!(|args: Cli, log_level: verbosity| {
    let mut parsed_files: Vec<ParsedEventFile> = Vec::new();

    // The glob pattern we are going to use to find the go files with event defs.
    let pattern = format!("{}/events/*.go", args.sdk_location.to_string_lossy());

    // Some files we don't properly handle yet.
    let blacklist = get_blacklist();

    let example_event_path = args.sdk_location.clone().join("events/testdata");
    let fuzzy_example_events = get_fuzzy_file_listing(&example_event_path)?;

    // Loop over matched files.
    for path in glob(&pattern)? {
        let x = path.clone();
        let file_name = x.file_stem().expect("file stem").to_string_lossy();

        // Filter out tests and blacklisted files.
        if !file_name.contains("_test") && !blacklist.contains(&*file_name) {
            // Parse the code.
            info!("Parsing: {}", x.to_string_lossy());
            let (go, rust) = go_to_rust::parse_go_file(&path)?;
            debug!("Go ------v\n{}", go);
            debug!("Rust-----v\n{}", rust);

            // Check for an example event in their test data.
            let example_events = find_example_events(
                &fuzzy_example_events,
                &file_name,
                &example_event_path,
                &rust.scope(),
            );

            parsed_files.push(ParsedEventFile {
                service_name: file_name.into_owned(),
                path,
                go,
                rust,
                example_events,
            });
        }
    }

    // Create the output location if needed.
    if !args.output_location.exists() {
        trace!("Creating directory: {:?}", args.output_location);
        create_dir(&args.output_location)?;
    }

    // Write the files.
    for parsed in &mut parsed_files {
        let out_dir = args.output_location.clone();
        let output_path = out_dir.join(
            parsed
                .path
                .with_extension("rs")
                .file_name()
                .expect("a file name exists"),
        );

        if !parsed.example_events.is_empty() {
            for example_event in &parsed.example_events {
                // Write the example event to a test fixture.
                trace!("Writing fixure for: {:?}", parsed.service_name);
                let _ = write_fixture(&example_event, &out_dir, args.overwrite)?;
            }

            trace!("Generating test module for: {:?}", parsed.service_name);
            let test_module = generate_test_module(&parsed.example_events)?;
            parsed.rust.push_module(test_module);
        }

        if overwrite_warning(&output_path, args.overwrite).is_none() {
            let mut f = File::create(output_path)?;
            f.write_all(parsed.rust.to_string().as_bytes())?;
            f.write_all(b"\n")?;
        }
    }

    // Write the crate index.
    let mod_path = args.output_location.clone().join("mod.rs");
    write_mod_index(&mod_path, &parsed_files, args.overwrite)?;

    // Write the crate readme.
    let output = Command::new("git")
        .arg(format!(
            "--git-dir={}",
            args.sdk_location.join(".git").to_string_lossy()
        ))
        .arg("rev-parse")
        .arg("--verify")
        .arg("HEAD")
        .output()
        .expect("failed to execute git")
        .stdout;
    let git_hash = String::from_utf8_lossy(&output);
    let readme_path = args.output_location.clone().join("README.md");
    write_readme(&readme_path, git_hash.trim(), args.overwrite)?;
});
