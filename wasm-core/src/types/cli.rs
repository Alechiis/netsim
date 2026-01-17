use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CliView {
    UserView,
    SystemView,
    InterfaceView,
    BgpView,
    PoolView,
    ZoneView,
    AaaView,
    AclView,
    SecurityPolicyView,
    SecurityRuleView,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CliState {
    pub view: CliView,
    pub current_interface_id: Option<String>,
    pub current_pool_name: Option<String>,
    pub bgp_view: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommandResult {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub output: String,
    #[serde(default)]
    pub new_view: Option<CliView>,
    /// If hostname was changed, this contains the new hostname to sync to UI
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub new_hostname: Option<String>,
}

impl CommandResult {
    /// Create a successful result with output
    pub fn success(output: String) -> Self {
        Self {
            success: true,
            output,
            new_view: None,
            new_hostname: None,
        }
    }

    /// Create a failed result with error message
    pub fn error(output: String) -> Self {
        Self {
            success: false,
            output,
            new_view: None,
            new_hostname: None,
        }
    }
}

