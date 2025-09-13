use serde::Deserialize;
use std::collections::HashMap;
use crate::lua::LuaVersion;

/// Prometheus global configuration constants.
pub const NAME: &str = "Prometheus";
pub const REVISION: &str = "Alpha";
pub const VERSION: &str = "v0.2";
pub const BY: &str = "levno-710";
pub const NAME_UPPER: &str = "PROMETHEUS";
pub const NAME_AND_VERSION: &str = "Prometheus v0.2";
pub const IDENT_PREFIX: &str = "__prometheus_";
pub const SPACE: &str = " ";
pub const TAB: &str = "\t";

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "LuaVersion", default)]
    pub lua_version: LuaVersion,
    #[serde(rename = "VarNamePrefix", default)]
    pub var_name_prefix: String,
    #[serde(rename = "NameGenerator", default)]
    pub name_generator: String,
    #[serde(rename = "PrettyPrint", default)]
    pub pretty_print: bool,
    #[serde(rename = "Seed", default)]
    pub seed: u64,
    #[serde(rename = "Steps", default)]
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Step {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Settings", default)]
    pub settings: HashMap<String, serde_json::Value>,
}


impl Default for Config {
    fn default() -> Self {
        Config {
            lua_version: LuaVersion::Lua51,
            var_name_prefix: String::new(),
            name_generator: "MangledShuffled".to_string(),
            pretty_print: false,
            seed: 0,
            steps: vec![],
        }
    }
}

/// Load a built-in preset by name.
pub fn load_preset(name: &str) -> Option<Config> {
    match name {
        "Minify" => Some(Config::default()),
        _ => None,
    }
}
