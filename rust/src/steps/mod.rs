use std::collections::HashMap;

use serde_json::Value;

use crate::ast::AstNode;
use crate::pipeline::Pipeline;
use crate::step::{SettingDescriptor, Step};

// ---------------------------------------------------------------------------
// ConstantArray
// ---------------------------------------------------------------------------

pub struct ConstantArray {
    pub treshold: f64,
    pub strings_only: bool,
    pub shuffle: bool,
    pub rotate: bool,
    pub local_wrapper_treshold: f64,
    pub local_wrapper_count: u64,
    pub local_wrapper_arg_count: u64,
    pub max_wrapper_offset: u64,
    pub encoding: String,
}

impl ConstantArray {
    pub fn new(settings: &HashMap<String, Value>) -> Self {
        Self {
            treshold: settings
                .get("Treshold")
                .and_then(Value::as_f64)
                .unwrap_or(1.0),
            strings_only: settings
                .get("StringsOnly")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            shuffle: settings
                .get("Shuffle")
                .and_then(Value::as_bool)
                .unwrap_or(true),
            rotate: settings
                .get("Rotate")
                .and_then(Value::as_bool)
                .unwrap_or(true),
            local_wrapper_treshold: settings
                .get("LocalWrapperTreshold")
                .and_then(Value::as_f64)
                .unwrap_or(1.0),
            local_wrapper_count: settings
                .get("LocalWrapperCount")
                .and_then(Value::as_u64)
                .unwrap_or(0),
            local_wrapper_arg_count: settings
                .get("LocalWrapperArgCount")
                .and_then(Value::as_u64)
                .unwrap_or(10),
            max_wrapper_offset: settings
                .get("MaxWrapperOffset")
                .and_then(Value::as_u64)
                .unwrap_or(65535),
            encoding: settings
                .get("Encoding")
                .and_then(Value::as_str)
                .unwrap_or("base64")
                .to_string(),
        }
    }
}

impl Step for ConstantArray {
    fn name(&self) -> &'static str {
        "Constant Array"
    }
    fn description(&self) -> &'static str {
        "This Step will Extract all Constants and put them into an Array at the beginning of the script"
    }
    fn settings_descriptor(&self) -> &'static [SettingDescriptor] {
        &CONSTANT_ARRAY_SETTINGS
    }
    fn apply(&mut self, ast: AstNode, _pipeline: &Pipeline) -> AstNode {
        ast
    }
}

const CONSTANT_ARRAY_SETTINGS: [SettingDescriptor; 9] = [
    SettingDescriptor::number(
        "Treshold",
        "The relative amount of nodes that will be affected",
        1.0,
        Some(0.0),
        Some(1.0),
    ),
    SettingDescriptor::boolean(
        "StringsOnly",
        "Whether to only Extract Strings",
        false,
    ),
    SettingDescriptor::boolean(
        "Shuffle",
        "Whether to shuffle the order of Elements in the Array",
        true,
    ),
    SettingDescriptor::boolean(
        "Rotate",
        "Whether to rotate the String Array by a specific (random) amount. This will be undone on runtime.",
        true,
    ),
    SettingDescriptor::number(
        "LocalWrapperTreshold",
        "The relative amount of functions that will get local wrappers",
        1.0,
        Some(0.0),
        Some(1.0),
    ),
    SettingDescriptor::number(
        "LocalWrapperCount",
        "The number of Local wrapper Functions per scope. This only applies if LocalWrapperTreshold is greater than 0",
        0.0,
        Some(0.0),
        Some(512.0),
    ),
    SettingDescriptor::number(
        "LocalWrapperArgCount",
        "The number of Arguments to the Local wrapper Functions",
        10.0,
        Some(1.0),
        Some(200.0),
    ),
    SettingDescriptor::number(
        "MaxWrapperOffset",
        "The Max Offset for the Wrapper Functions",
        65535.0,
        Some(0.0),
        None,
    ),
    SettingDescriptor::enumeration(
        "Encoding",
        "The Encoding to use for the Strings",
        "base64",
        &["none", "base64"],
    ),
];

// ---------------------------------------------------------------------------
// WrapInFunction
// ---------------------------------------------------------------------------

pub struct WrapInFunction {
    pub iterations: u64,
}

impl WrapInFunction {
    pub fn new(settings: &HashMap<String, Value>) -> Self {
        Self {
            iterations: settings
                .get("Iterations")
                .and_then(Value::as_u64)
                .unwrap_or(1),
        }
    }
}

impl Step for WrapInFunction {
    fn name(&self) -> &'static str {
        "Wrap in Function"
    }
    fn description(&self) -> &'static str {
        "This Step Wraps the Entire Script into a Function"
    }
    fn settings_descriptor(&self) -> &'static [SettingDescriptor] {
        &WRAP_IN_FUNCTION_SETTINGS
    }
    fn apply(&mut self, ast: AstNode, _pipeline: &Pipeline) -> AstNode {
        ast
    }
}

const WRAP_IN_FUNCTION_SETTINGS: [SettingDescriptor; 1] = [SettingDescriptor::number(
    "Iterations",
    "The Number Of Iterations",
    1.0,
    Some(1.0),
    None,
)];

// ---------------------------------------------------------------------------
// AntiTamper
// ---------------------------------------------------------------------------

pub struct AntiTamper {
    pub use_debug: bool,
}

impl AntiTamper {
    pub fn new(settings: &HashMap<String, Value>) -> Self {
        Self {
            use_debug: settings
                .get("UseDebug")
                .and_then(Value::as_bool)
                .unwrap_or(true),
        }
    }
}

impl Step for AntiTamper {
    fn name(&self) -> &'static str {
        "Anti Tamper"
    }
    fn description(&self) -> &'static str {
        "This Step Breaks your Script when it is modified. This is only effective when using the new VM."
    }
    fn settings_descriptor(&self) -> &'static [SettingDescriptor] {
        &ANTI_TAMPER_SETTINGS
    }
    fn apply(&mut self, ast: AstNode, _pipeline: &Pipeline) -> AstNode {
        ast
    }
}

const ANTI_TAMPER_SETTINGS: [SettingDescriptor; 1] = [SettingDescriptor::boolean(
    "UseDebug",
    "Use debug library. (Recommended, however scripts will not work without debug library.)",
    true,
)];

// ---------------------------------------------------------------------------
// AddVararg
// ---------------------------------------------------------------------------

pub struct AddVararg;

impl AddVararg {
    pub fn new(_settings: &HashMap<String, Value>) -> Self {
        AddVararg
    }
}

impl Step for AddVararg {
    fn name(&self) -> &'static str {
        "Add Vararg"
    }
    fn description(&self) -> &'static str {
        "This Step Adds Vararg to all Functions"
    }
    fn settings_descriptor(&self) -> &'static [SettingDescriptor] {
        &[]
    }
    fn apply(&mut self, ast: AstNode, _pipeline: &Pipeline) -> AstNode {
        ast
    }
}

// ---------------------------------------------------------------------------
// NumbersToExpressions
// ---------------------------------------------------------------------------

pub struct NumbersToExpressions {
    pub treshold: f64,
    pub internal_treshold: f64,
}

impl NumbersToExpressions {
    pub fn new(settings: &HashMap<String, Value>) -> Self {
        Self {
            treshold: settings
                .get("Treshold")
                .and_then(Value::as_f64)
                .unwrap_or(1.0),
            internal_treshold: settings
                .get("InternalTreshold")
                .and_then(Value::as_f64)
                .unwrap_or(0.2),
        }
    }
}

impl Step for NumbersToExpressions {
    fn name(&self) -> &'static str {
        "Numbers To Expressions"
    }
    fn description(&self) -> &'static str {
        "This Step Converts number Literals to Expressions"
    }
    fn settings_descriptor(&self) -> &'static [SettingDescriptor] {
        &NUMBERS_TO_EXPRESSIONS_SETTINGS
    }
    fn apply(&mut self, ast: AstNode, _pipeline: &Pipeline) -> AstNode {
        ast
    }
}

const NUMBERS_TO_EXPRESSIONS_SETTINGS: [SettingDescriptor; 2] = [
    SettingDescriptor::number(
        "Treshold",
        "The relative amount of nodes that will be affected",
        1.0,
        Some(0.0),
        Some(1.0),
    ),
    SettingDescriptor::number(
        "InternalTreshold",
        "Internal recursion treshold",
        0.2,
        Some(0.0),
        Some(0.8),
    ),
];

// ---------------------------------------------------------------------------
// SplitStrings
// ---------------------------------------------------------------------------

pub struct SplitStrings {
    pub treshold: f64,
    pub min_length: u64,
    pub max_length: u64,
    pub concatenation_type: String,
    pub custom_function_type: String,
    pub custom_local_functions_count: u64,
}

impl SplitStrings {
    pub fn new(settings: &HashMap<String, Value>) -> Self {
        Self {
            treshold: settings
                .get("Treshold")
                .and_then(Value::as_f64)
                .unwrap_or(1.0),
            min_length: settings
                .get("MinLength")
                .and_then(Value::as_u64)
                .unwrap_or(5),
            max_length: settings
                .get("MaxLength")
                .and_then(Value::as_u64)
                .unwrap_or(5),
            concatenation_type: settings
                .get("ConcatenationType")
                .and_then(Value::as_str)
                .unwrap_or("custom")
                .to_string(),
            custom_function_type: settings
                .get("CustomFunctionType")
                .and_then(Value::as_str)
                .unwrap_or("global")
                .to_string(),
            custom_local_functions_count: settings
                .get("CustomLocalFunctionsCount")
                .and_then(Value::as_u64)
                .unwrap_or(2),
        }
    }
}

impl Step for SplitStrings {
    fn name(&self) -> &'static str {
        "Split Strings"
    }
    fn description(&self) -> &'static str {
        "This Step splits Strings to a specific or random length"
    }
    fn settings_descriptor(&self) -> &'static [SettingDescriptor] {
        &SPLIT_STRINGS_SETTINGS
    }
    fn apply(&mut self, ast: AstNode, _pipeline: &Pipeline) -> AstNode {
        ast
    }
}

const SPLIT_STRINGS_SETTINGS: [SettingDescriptor; 6] = [
    SettingDescriptor::number(
        "Treshold",
        "The relative amount of nodes that will be affected",
        1.0,
        Some(0.0),
        Some(1.0),
    ),
    SettingDescriptor::number(
        "MinLength",
        "The minimal length for the chunks in that the Strings are splitted",
        5.0,
        Some(1.0),
        None,
    ),
    SettingDescriptor::number(
        "MaxLength",
        "The maximal length for the chunks in that the Strings are splitted",
        5.0,
        Some(1.0),
        None,
    ),
    SettingDescriptor::enumeration(
        "ConcatenationType",
        "The Functions used for Concatenation. Note that when using custom, the String Array will also be Shuffled",
        "custom",
        &["strcat", "table", "custom"],
    ),
    SettingDescriptor::enumeration(
        "CustomFunctionType",
        "The Type of Function code injection. This option only applies when custom Concatenation is selected.",
        "global",
        &["global", "local", "inline"],
    ),
    SettingDescriptor::number(
        "CustomLocalFunctionsCount",
        "The number of local functions per scope. This option only applies when CustomFunctionType = local",
        2.0,
        Some(1.0),
        None,
    ),
];

// ---------------------------------------------------------------------------
// Watermark
// ---------------------------------------------------------------------------

pub struct Watermark {
    pub content: String,
    pub custom_variable: String,
}

impl Watermark {
    pub fn new(settings: &HashMap<String, Value>) -> Self {
        Self {
            content: settings
                .get("Content")
                .and_then(Value::as_str)
                .unwrap_or("This Script is Part of the Prometheus Obfuscator by Levno_710")
                .to_string(),
            custom_variable: settings
                .get("CustomVariable")
                .and_then(Value::as_str)
                .unwrap_or("_WATERMARK")
                .to_string(),
        }
    }
}

impl Step for Watermark {
    fn name(&self) -> &'static str {
        "Watermark"
    }
    fn description(&self) -> &'static str {
        "This Step will add a watermark to the script"
    }
    fn settings_descriptor(&self) -> &'static [SettingDescriptor] {
        &WATERMARK_SETTINGS
    }
    fn apply(&mut self, ast: AstNode, _pipeline: &Pipeline) -> AstNode {
        ast
    }
}

const WATERMARK_SETTINGS: [SettingDescriptor; 2] = [
    SettingDescriptor::string(
        "Content",
        "The Content of the Watermark",
        "This Script is Part of the Prometheus Obfuscator by Levno_710",
    ),
    SettingDescriptor::string(
        "CustomVariable",
        "The Variable that will be used for the Watermark",
        "_WATERMARK",
    ),
];

// ---------------------------------------------------------------------------
// EncryptStrings
// ---------------------------------------------------------------------------

pub struct EncryptStrings;

impl EncryptStrings {
    pub fn new(_settings: &HashMap<String, Value>) -> Self {
        EncryptStrings
    }
}

impl Step for EncryptStrings {
    fn name(&self) -> &'static str {
        "Encrypt Strings"
    }
    fn description(&self) -> &'static str {
        "This Step will encrypt strings within your Program."
    }
    fn settings_descriptor(&self) -> &'static [SettingDescriptor] {
        &[]
    }
    fn apply(&mut self, ast: AstNode, _pipeline: &Pipeline) -> AstNode {
        ast
    }
}

// ---------------------------------------------------------------------------
// ProxifyLocals
// ---------------------------------------------------------------------------

pub struct ProxifyLocals {
    pub literal_type: String,
}

impl ProxifyLocals {
    pub fn new(settings: &HashMap<String, Value>) -> Self {
        Self {
            literal_type: settings
                .get("LiteralType")
                .and_then(Value::as_str)
                .unwrap_or("string")
                .to_string(),
        }
    }
}

impl Step for ProxifyLocals {
    fn name(&self) -> &'static str {
        "Proxify Locals"
    }
    fn description(&self) -> &'static str {
        "This Step wraps all locals into Proxy Objects"
    }
    fn settings_descriptor(&self) -> &'static [SettingDescriptor] {
        &PROXIFY_LOCALS_SETTINGS
    }
    fn apply(&mut self, ast: AstNode, _pipeline: &Pipeline) -> AstNode {
        ast
    }
}

const PROXIFY_LOCALS_SETTINGS: [SettingDescriptor; 1] = [SettingDescriptor::enumeration(
    "LiteralType",
    "The type of the randomly generated literals",
    "string",
    &["dictionary", "number", "string", "any"],
)];

// ---------------------------------------------------------------------------
// Vmify
// ---------------------------------------------------------------------------

pub struct Vmify;

impl Vmify {
    pub fn new(_settings: &HashMap<String, Value>) -> Self {
        Vmify
    }
}

impl Step for Vmify {
    fn name(&self) -> &'static str {
        "Vmify"
    }
    fn description(&self) -> &'static str {
        "This Step will Compile your script into a fully-custom Bytecode Format and emit a vm for executing it."
    }
    fn settings_descriptor(&self) -> &'static [SettingDescriptor] {
        &[]
    }
    fn apply(&mut self, ast: AstNode, _pipeline: &Pipeline) -> AstNode {
        ast
    }
}

// ---------------------------------------------------------------------------
// WatermarkCheck
// ---------------------------------------------------------------------------

pub struct WatermarkCheck {
    pub content: String,
}

impl WatermarkCheck {
    pub fn new(settings: &HashMap<String, Value>) -> Self {
        Self {
            content: settings
                .get("Content")
                .and_then(Value::as_str)
                .unwrap_or("This Script is Part of the Prometheus Obfuscator by Levno_710")
                .to_string(),
        }
    }
}

impl Step for WatermarkCheck {
    fn name(&self) -> &'static str {
        "WatermarkCheck"
    }
    fn description(&self) -> &'static str {
        "This Step will add a watermark to the script"
    }
    fn settings_descriptor(&self) -> &'static [SettingDescriptor] {
        &WATERMARK_CHECK_SETTINGS
    }
    fn apply(&mut self, ast: AstNode, _pipeline: &Pipeline) -> AstNode {
        ast
    }
}

const WATERMARK_CHECK_SETTINGS: [SettingDescriptor; 1] = [SettingDescriptor::string(
    "Content",
    "The Content of the WatermarkCheck",
    "This Script is Part of the Prometheus Obfuscator by Levno_710",
)];

// ---------------------------------------------------------------------------
// Registration helper
// ---------------------------------------------------------------------------

/// Register all built-in steps with the provided pipeline.
pub fn register_builtin_steps(pipeline: &mut Pipeline) {
    pipeline.register_step("ConstantArray", |s| Box::new(ConstantArray::new(s)));
    pipeline.register_step("WrapInFunction", |s| Box::new(WrapInFunction::new(s)));
    pipeline.register_step("AntiTamper", |s| Box::new(AntiTamper::new(s)));
    pipeline.register_step("AddVararg", |s| Box::new(AddVararg::new(s)));
    pipeline.register_step(
        "NumbersToExpressions",
        |s| Box::new(NumbersToExpressions::new(s)),
    );
    pipeline.register_step("SplitStrings", |s| Box::new(SplitStrings::new(s)));
    pipeline.register_step("Watermark", |s| Box::new(Watermark::new(s)));
    pipeline.register_step("EncryptStrings", |s| Box::new(EncryptStrings::new(s)));
    pipeline.register_step("ProxifyLocals", |s| Box::new(ProxifyLocals::new(s)));
    pipeline.register_step("Vmify", |s| Box::new(Vmify::new(s)));
    pipeline.register_step("WatermarkCheck", |s| Box::new(WatermarkCheck::new(s)));
}

