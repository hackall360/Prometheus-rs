use std::collections::HashMap;

use crate::ast::AstNode;
use crate::config::Config;
use crate::lexer::tokenize;
use crate::lua::LuaVersion;
use crate::name_generators::{
    ConfuseGenerator, IlGenerator, MangledGenerator, MangledShuffledGenerator, NumberGenerator,
};
use crate::parser::parse;

/// Trait for transformation steps in the obfuscation pipeline.
pub trait Step {
    /// Apply the transformation to the AST.
    fn apply(&mut self, ast: AstNode, pipeline: &Pipeline) -> AstNode;
}

/// Trait for variable name generators.
pub trait NameGenerator {
    /// Generate the next identifier.
    fn generate(&mut self) -> String;
}

/// Factory type used for constructing steps from configuration.
pub type StepConstructor = fn(&HashMap<String, serde_json::Value>) -> Box<dyn Step>;

/// Orchestrates parsing, running transformation steps and emitting code.
pub struct Pipeline {
    pub lua_version: LuaVersion,
    pub pretty_print: bool,
    pub var_name_prefix: String,
    pub seed: u64,
    pub name_generator: Box<dyn NameGenerator>,
    steps: Vec<Box<dyn Step>>,
    step_constructors: HashMap<String, StepConstructor>,
}

impl Pipeline {
    /// Create an empty pipeline with the given settings.
    pub fn new(
        lua_version: LuaVersion,
        pretty_print: bool,
        var_name_prefix: String,
        seed: u64,
    ) -> Self {
        Self {
            lua_version,
            pretty_print,
            var_name_prefix,
            seed,
            name_generator: Box::new(MangledShuffledGenerator::new(seed)),
            steps: Vec::new(),
            step_constructors: HashMap::new(),
        }
    }

    /// Register a step constructor that can later be referenced by name in [`Config`].
    pub fn register_step(&mut self, name: &str, constructor: StepConstructor) {
        self.step_constructors.insert(name.to_string(), constructor);
    }

    /// Set the name generator by predefined name.
    pub fn set_name_generator(&mut self, name: &str) -> Result<(), String> {
        let seed = self.seed;
        self.name_generator = match name {
            "Mangled" => Box::new(MangledGenerator::new()),
            "MangledShuffled" => Box::new(MangledShuffledGenerator::new(seed)),
            "Il" => Box::new(IlGenerator::new(seed)),
            "Confuse" => Box::new(ConfuseGenerator::new(seed)),
            "Number" => Box::new(NumberGenerator::new()),
            _ => return Err(format!("unknown name generator {name}")),
        };
        Ok(())
    }

    /// Construct a pipeline from a [`Config`].
    pub fn from_config(config: Config) -> Result<Self, String> {
        let mut pipeline = Pipeline::new(
            config.lua_version,
            config.pretty_print,
            config.var_name_prefix,
            config.seed,
        );
        pipeline.set_name_generator(&config.name_generator)?;

        for step_cfg in config.steps {
            let constructor = pipeline
                .step_constructors
                .get(&step_cfg.name)
                .ok_or_else(|| format!("step {} not registered", step_cfg.name))?;
            let step = constructor(&step_cfg.settings);
            pipeline.steps.push(step);
        }

        Ok(pipeline)
    }

    /// Manually add a step instance to the pipeline.
    pub fn add_step(&mut self, step: Box<dyn Step>) {
        self.steps.push(step);
    }

    /// Apply the pipeline to the given Lua source code.
    ///
    /// Currently this parses the code and applies steps, but returns the input
    /// unchanged until an unparser is available.
    pub fn apply(&mut self, code: &str) -> Result<String, String> {
        let tokens = tokenize(code, self.lua_version);
        let parse_result = parse(&tokens, self.lua_version).map_err(|e| format!("{:?}", e))?;
        let mut ast = parse_result.ast;

        let mut steps = std::mem::take(&mut self.steps);
        for step in steps.iter_mut() {
            ast = step.apply(ast, self);
        }
        self.steps = steps;

        // TODO: integrate variable renaming and unparsing when implemented.
        let _ = ast; // silence unused variable warning until unparser exists
        Ok(code.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_config_sets_defaults() {
        let config = Config::default();
        let pipeline = Pipeline::from_config(config).unwrap();
        assert_eq!(pipeline.var_name_prefix, "");
    }

    #[test]
    fn apply_roundtrip_without_steps() {
        let config = Config::default();
        let mut pipeline = Pipeline::from_config(config).unwrap();
        let src = "return 1";
        let out = pipeline.apply(src).unwrap();
        assert_eq!(out, src);
    }
}
