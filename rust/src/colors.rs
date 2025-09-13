use std::sync::atomic::{AtomicBool, Ordering};

static ENABLED: AtomicBool = AtomicBool::new(true);

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Bright = 1,
    Dim = 2,
    Underline = 4,
    Blink = 5,
    Reverse = 7,
    Hidden = 8,
    Black = 30,
    Pink = 91,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    Grey = 37,
    White = 97,
    BlackBg = 40,
    RedBg = 41,
    GreenBg = 42,
    YellowBg = 43,
    BlueBg = 44,
    MagentaBg = 45,
    CyanBg = 46,
    GreyBg = 47,
    WhiteBg = 107,
}

/// Alias for Grey.
pub const GRAY: Color = Color::Grey;
/// Alias for GreyBg.
pub const GRAY_BG: Color = Color::GreyBg;

fn escape(code: u32) -> String {
    format!("\x1b[{}m", code)
}

/// Enable or disable colored output globally.
pub fn set_enabled(enabled: bool) {
    ENABLED.store(enabled, Ordering::Relaxed);
}

/// Returns whether colored output is enabled.
pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// Apply ANSI colors to a string if color output is enabled.
pub fn colorize(text: impl AsRef<str>, colors: &[Color]) -> String {
    if !is_enabled() || colors.is_empty() {
        return text.as_ref().to_string();
    }
    let mut result = String::new();
    result.push_str(&escape(0));
    for c in colors {
        result.push_str(&escape(*c as u32));
    }
    result.push_str(text.as_ref());
    result.push_str(&escape(0));
    result
}
