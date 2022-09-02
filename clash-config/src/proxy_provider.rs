use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProxyProviderType {
    Http,
    File,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HealthCheck {
    pub enable: Option<bool>,
    pub interval: Option<u32>,
    pub lazy: Option<bool>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProxyProvider {
    #[serde(rename = "type")]
    pub ty: ProxyProviderType,
    pub url: Option<String>,
    pub interval: Option<u32>,
    pub path: String,
    pub health_check: Option<HealthCheck>,
    pub filter: Option<String>,
}
