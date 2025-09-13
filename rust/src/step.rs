use std::collections::HashMap;

use serde_json::Value;

use crate::ast::AstNode;
use crate::pipeline::Pipeline;

/// Supported types for a [`SettingDescriptor`].
#[derive(Clone, Copy, Debug)]
pub enum SettingKind {
    Boolean,
    Number,
    String,
    Enum,
}

/// Default value of a [`SettingDescriptor`].
#[derive(Clone, Copy, Debug)]
pub enum DefaultValue {
    Bool(bool),
    Number(f64),
    Str(&'static str),
}

/// Describes a configurable setting for a [`Step`].
#[derive(Clone, Debug)]
pub struct SettingDescriptor {
    pub name: &'static str,
    pub description: &'static str,
    pub kind: SettingKind,
    pub default: DefaultValue,
    pub min: Option<f64>,
    pub max: Option<f64>,
    /// Possible values for enum settings.
    pub values: &'static [&'static str],
}

impl SettingDescriptor {
    /// Convenience constructor for boolean settings.
    pub const fn boolean(
        name: &'static str,
        description: &'static str,
        default: bool,
    ) -> Self {
        Self {
            name,
            description,
            kind: SettingKind::Boolean,
            default: DefaultValue::Bool(default),
            min: None,
            max: None,
            values: &[],
        }
    }

    /// Convenience constructor for numeric settings with optional bounds.
    pub const fn number(
        name: &'static str,
        description: &'static str,
        default: f64,
        min: Option<f64>,
        max: Option<f64>,
    ) -> Self {
        Self {
            name,
            description,
            kind: SettingKind::Number,
            default: DefaultValue::Number(default),
            min,
            max,
            values: &[],
        }
    }

    /// Convenience constructor for string settings.
    pub const fn string(
        name: &'static str,
        description: &'static str,
        default: &'static str,
    ) -> Self {
        Self {
            name,
            description,
            kind: SettingKind::String,
            default: DefaultValue::Str(default),
            min: None,
            max: None,
            values: &[],
        }
    }

    /// Convenience constructor for enum settings.
    pub const fn enumeration(
        name: &'static str,
        description: &'static str,
        default: &'static str,
        values: &'static [&'static str],
    ) -> Self {
        Self {
            name,
            description,
            kind: SettingKind::Enum,
            default: DefaultValue::Str(default),
            min: None,
            max: None,
            values,
        }
    }
}

/// Trait for transformation steps in the obfuscation pipeline.
pub trait Step {
    /// Human readable name of the step.
    fn name(&self) -> &'static str;
    /// Description of what the step does.
    fn description(&self) -> &'static str;
    /// Descriptor of supported settings.
    fn settings_descriptor(&self) -> &'static [SettingDescriptor];
    /// Apply the transformation to the AST.
    fn apply(&mut self, ast: AstNode, pipeline: &Pipeline) -> AstNode;
}

/// Factory type used for constructing steps from configuration.
pub type StepConstructor = fn(&HashMap<String, Value>) -> Box<dyn Step>;

