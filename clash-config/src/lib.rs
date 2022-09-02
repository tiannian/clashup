use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

mod proxy_group;
pub use proxy_group::*;

mod proxy_provider;
pub use proxy_provider::*;

mod rule_provider;
pub use rule_provider::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Rule,
    Global,
    Direct,
    Script,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
    Silent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    pub store_selected: Option<bool>,
    pub store_fake_ip: Option<bool>,
    pub tracing: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FallbackFilter {
    pub geoip: Option<bool>,
    pub geoip_code: Option<String>,
    pub ipcidr: Option<Vec<String>>,
    pub domain: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EnhancedMode {
    RedirHost,
    FakeIp,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Dns {
    pub enable: Option<bool>,
    pub listen: String,
    pub ipv6: Option<bool>,
    pub default_nameserver: Option<Vec<String>>,
    pub enhanced_mode: Option<EnhancedMode>,
    pub fake_ip_range: Option<String>,
    pub use_hosts: Option<String>,
    pub fake_ip_filter: Option<Vec<String>>,
    pub nameserver: Vec<String>,
    pub fallback: Option<Vec<String>>,
    pub fallback_filter: Option<FallbackFilter>,
    pub nameserver_policy: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Epbf {
    redirect_to_tun: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Experimental {
    pub ignore_resolve_fail: Option<bool>,
    pub sniff_tls_sni: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Stack {
    System,
    Gvisor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Tun {
    pub enable: Option<bool>,
    pub stack: Option<Stack>,
    pub dns_hijack: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Script {
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub port: Option<u32>,
    pub socks_port: Option<u32>,
    pub redir_port: Option<u32>,
    pub tproxy_port: Option<u32>,
    pub mixed_port: Option<u32>,
    pub authentication: Vec<String>,
    pub allow_lan: bool,
    pub bind_address: Option<String>,
    pub mode: Mode,
    pub log_level: LogLevel,
    pub ipv6: bool,
    pub external_controller: Option<String>,
    pub external_ui: Option<String>,
    pub interface_name: Option<String>,
    pub experimental: Option<Experimental>,
    pub routing_mark: Option<String>,
    pub epbf: Epbf,
    pub hosts: BTreeMap<String, String>,
    pub profile: Option<Profile>,
    pub dns: Option<Dns>,
    pub proxies: Vec<serde_yaml::Value>,
    pub proxy_groups: Option<Vec<ProxyGroup>>,
    pub proxy_providers: Option<BTreeMap<String, ProxyProvider>>,
    pub rule: Option<Vec<String>>,
    pub tun: Option<Tun>,
    pub rule_providers: Option<BTreeMap<String, RuleProvider>>,
    pub script: Option<Script>,
}
