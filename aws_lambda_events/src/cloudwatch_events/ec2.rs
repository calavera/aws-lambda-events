use serde_derive::Deserialize;
use serde_derive::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceStateChange {
    #[serde(rename = "instance-id")]
    pub instance_id: String,
    pub state: String,
}
