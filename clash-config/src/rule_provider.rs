use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProxyProviderType {
    Http,
    File,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Behavior {
    Domain,
    Ipcidr,
    Classical,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RuleProvider {
    #[serde(rename = "type")]
    pub ty: ProxyProviderType,
    pub behavior: Behavior,
    pub path: String,
    pub url: Option<String>,
    pub interval: Option<String>,
}
