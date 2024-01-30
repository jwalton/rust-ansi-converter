//! Convert an ANSI document from RGB or 256 colors down to 16 colors.
//!
//!
//! ```rust
//! # use ansi_converter::string_to_ansi_16;
//! #
//! # fn main() {
//!     let input = "\x1b[38;2;0;255;255mHello, World!";
//!     let output = string_to_ansi_16(input);
//!     assert_eq!(output, "\x1b[96mHello, World!");
//! # }
//! ````
//!

use ansi_styles::DEFAULT_BG;
use ansi_parser::AnsiDocument;

mod ansi_styles;
mod ansi_parser;
mod invert;

pub use ansi_styles::AnsiColor;
use invert::{can_invert_char, invert_char};

/// Converts an ANSI string to 16 colors.
pub fn string_to_ansi_16(input: &str) -> String {
    let mut doc = AnsiDocument::from_string(input);

    let mut current_bg = DEFAULT_BG;

    for c in doc.chars.iter_mut() {
        // Convert to 16 colors.
        c.fg = c.fg.to_ansi_16();
        c.bg = c.bg.to_ansi_16();

        if c.c == ' ' && c.bg != DEFAULT_BG {
            // Prefer colored foreground blocks over colored spaces.
            c.fg = c.bg;
            c.bg = current_bg;
            c.c = '█';
        } else if c.fg == c.bg {
            // If the foreground and background are the same, replace whatever is here with a space or block.
            if c.bg == DEFAULT_BG {
                c.c = ' ';
            } else {
                c.c = '█';
            }
        } else if c.fg == DEFAULT_BG && can_invert_char(c.c) {
            // If the foreground is black, and we can "flip" this character, then do so.
            c.fg = c.bg;
            c.bg = DEFAULT_BG;
            c.c = invert_char(c.c);
        };

        current_bg = c.bg;
    }

    doc.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_rgb_to_ansi_16() {
        let input = "\x1b[38;2;0;255;255mHello, World!";
        let output = string_to_ansi_16(input);
        assert_eq!(output, "\x1b[96mHello, World!");
    }

    #[test]
    fn should_prefer_back_backgrounds() {
        let input = "\x1b[38;5;0;48;5;37m▄";
        let output = string_to_ansi_16(input);
        assert_eq!(output, "\x1b[36m▀");
    }

    #[test]
    fn should_convert_ansi_256_to_ansi_16() {
        let input = "\x1b[38;5;51mHello, World!";
        let output = string_to_ansi_16(input);
        assert_eq!(output, "\x1b[96mHello, World!");

        let input = "\x1b[48;5;0m     \x1b[38;5;15;48;5;0m▄▄▄\x1b[48;5;15m    \x1b[38;5;15;48;5;0m▄▄\x1b[48;5;0m \x1b[38;5;37;48;5;0m▄\x1b[48;5;37m   \x1b[38;5;37;48;5;0m▄\x1b[48;5;0m  \x1b[m";
        let output = string_to_ansi_16(input);
        assert_eq!(
            output,
            "     \x1b[97m▄▄▄\x1b[107m████\x1b[40m▄▄ \x1b[36m▄\x1b[46m███\x1b[40m▄  \x1b[0m"
        );
    }
}
