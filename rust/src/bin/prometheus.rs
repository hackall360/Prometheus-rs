use std::error::Error;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use prometheus_rs::{
    colors,
    logger::{Logger, LogLevel},
    obfuscate, Config, LuaVersion, load_preset,
};

#[derive(Parser, Debug)]
#[command(author, version, about = "Prometheus obfuscator CLI")]
struct Cli {
    /// Input Lua source file
    source: PathBuf,

    /// Use a built-in preset
    #[arg(short, long)]
    preset: Option<String>,

    /// Load configuration from external JSON file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Output file path
    #[arg(short, long)]
    out: Option<PathBuf>,

    /// Disable colored output
    #[arg(long)]
    nocolors: bool,

    /// Set log level (error, warn, log, debug)
    #[arg(long, value_enum, default_value_t = LogLevel::Log)]
    loglevel: LogLevel,

    /// Override Lua version to Lua 5.1
    #[arg(long = "Lua51")]
    lua51: bool,

    /// Override Lua version to Luau
    #[arg(long = "LuaU")]
    luau: bool,

    /// Enable pretty printed output
    #[arg(long)]
    pretty: bool,

    /// Save errors to a .error.txt file
    #[arg(long)]
    saveerrors: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    colors::set_enabled(!cli.nocolors);
    let logger = Logger::new(cli.loglevel);

    // Load configuration
    let mut config: Config = if let Some(preset) = cli.preset.as_deref() {
        load_preset(preset).ok_or_else(|| format!("Preset '{preset}' not found"))?
    } else if let Some(path) = cli.config.as_ref() {
        let text = fs::read_to_string(path)?;
        serde_json::from_str(&text)?
    } else {
        load_preset("Minify").expect("Default preset available")
    };

    if cli.lua51 {
        config.lua_version = LuaVersion::Lua51;
    }
    if cli.luau {
        config.lua_version = LuaVersion::LuaU;
    }
    if cli.pretty {
        config.pretty_print = true;
    }

    // Determine output file
    let out_path = cli.out.unwrap_or_else(|| {
        let mut p = cli.source.clone();
        p.set_extension("obfuscated.lua");
        p
    });

    let source = fs::read_to_string(&cli.source)?;

    // Placeholder obfuscation until Rust port is complete
    let out = match std::panic::catch_unwind(|| obfuscate(&source)) {
        Ok(code) => code,
        Err(_) => format!("-- obfuscation not yet implemented\n{source}"),
    };

    fs::write(&out_path, out)?;
    logger.log(format!("Wrote output to {}", out_path.display()));
    Ok(())
}
