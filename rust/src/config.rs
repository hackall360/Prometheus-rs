use serde::Deserialize;
use std::collections::HashMap;

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

#[derive(Debug, Clone, Deserialize)]
pub enum LuaVersion {
    Lua51,
    LuaU,
}

impl Default for LuaVersion {
    fn default() -> Self {
        LuaVersion::Lua51
    }
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

