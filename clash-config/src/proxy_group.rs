use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProxyType {
    Fallback,
    LoadBalance,
    Select,
    UrlTest,
    Relay,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Strategy {
    ConsistentHashing,
    RoundRobin,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProxyGroup {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: ProxyType,
    pub proxies: Option<Vec<String>>,
    pub url: Option<String>,
    pub interval: Option<u32>,
    #[serde(rename = "use")]
    pub u: Option<Vec<String>>,
    pub disable_udp: Option<bool>,
    pub tolerance: Option<u32>,
    pub lazy: Option<bool>,
    pub strategy: Option<Strategy>,
}

