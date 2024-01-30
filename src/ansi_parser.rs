use anstyle_parse::{DefaultCharAccumulator, Params, Parser, Perform};

use crate::ansi_styles::{AnsiColor, ANSI_RESET, DEFAULT_BG, DEFAULT_FG};

/// Represents a character with ansi color information.
#[derive(Debug)]
pub struct AnsiChar {
    pub fg: AnsiColor,
    pub bg: AnsiColor,
    pub c: char,
}

/// Represents a parsed ANSI document.
pub struct AnsiDocument {
    pub chars: Vec<AnsiChar>,
}

/// Converts an ANSI file to a series of blocks.
///
/// The basic idea here is we convert the ANSI file into a series of `Blocks`,
/// where each block has some color information and a string of character.
/// We then serialize our blocks out into a new ANSI file.
///
/// We do this intermediate step so we can do some optimizations, like converting
/// colored spaces into colored foreground blocks, and removing redundant color information.
struct AnsiParser {
    current_fg: AnsiColor,
    current_bg: AnsiColor,
    output: Vec<AnsiChar>,
}

impl AnsiDocument {
    /// Converts an ANSI string to 16 colors.
    pub fn from_string(input: &str) -> Self {
        let mut performer = AnsiParser::new();
        let mut statemachine = Parser::<DefaultCharAccumulator>::new();
        for byte in input.as_bytes() {
            statemachine.advance(&mut performer, *byte);
        }

        AnsiDocument {
            chars: performer.output,
        }
    }
}

impl ToString for AnsiDocument {
    fn to_string(&self) -> String {
        let mut output = String::with_capacity(self.chars.len());

        let mut current_bg = DEFAULT_BG;
        let mut current_fg = DEFAULT_FG;

        for c in &self.chars {
            // Change the color if needed.
            if (c.bg == DEFAULT_BG && c.fg == DEFAULT_FG)
                && (current_bg != c.bg || current_fg != c.fg)
            {
                output.push_str(ANSI_RESET);
            } else if current_bg != c.bg && current_fg != c.fg {
                output.push_str(&format!("\x1b[{};{}m", c.fg.fg_code(), c.bg.bg_code()));
            } else if current_fg != c.fg {
                output.push_str(&format!("\x1b[{}m", c.fg.fg_code()));
            } else if current_bg != c.bg {
                output.push_str(&format!("\x1b[{}m", c.bg.bg_code()));
            };

            // Write the next character.Ø
            output.push(c.c);

            current_bg = c.bg;
            current_fg = c.fg;
        }

        output
    }
}

impl AnsiParser {
    pub fn new() -> Self {
        Self {
            current_bg: DEFAULT_BG,
            current_fg: DEFAULT_FG,
            output: Vec::new(),
        }
    }
}

impl Perform for AnsiParser {
    fn print(&mut self, c: char) {
        self.output.push(AnsiChar {
            fg: self.current_fg,
            bg: self.current_bg,
            c,
        });
    }

    fn execute(&mut self, byte: u8) {
        if byte == b'\n' {
            self.output.push(AnsiChar {
                fg: self.current_fg,
                bg: self.current_bg,
                c: '\n',
            });
        }
    }

    fn csi_dispatch(&mut self, params: &Params, _intermediates: &[u8], _ignore: bool, _c: u8) {
        let mut iter = params.iter();
        while let Some(value) = iter.next() {
            match value[0] {
                38 => match iter.next().unwrap()[0] {
                    5 => {
                        let c256 = iter.next().unwrap()[0];
                        self.current_fg = AnsiColor::Ansi256(c256 as u8);
                    }
                    2 => {
                        let r = iter.next().unwrap()[0] as u8;
                        let g = iter.next().unwrap()[0] as u8;
                        let b = iter.next().unwrap()[0] as u8;
                        self.current_fg = AnsiColor::Rgb(r, g, b);
                    }
                    _ => {
                        panic!("Unknown color mode");
                    }
                },
                48 => match iter.next().unwrap()[0] {
                    5 => {
                        let c256 = iter.next().unwrap()[0];
                        self.current_bg = AnsiColor::Ansi256(c256 as u8);
                    }
                    2 => {
                        let r = iter.next().unwrap()[0] as u8;
                        let g = iter.next().unwrap()[0] as u8;
                        let b = iter.next().unwrap()[0] as u8;
                        self.current_bg = AnsiColor::Rgb(r, g, b);
                    }
                    _ => {
                        panic!("Unknown color mode");
                    }
                },
                30..=37 | 90..=97 => self.current_fg = AnsiColor::Ansi16(value[0] as u8),
                40..=47 | 100..=107 => self.current_bg = AnsiColor::Ansi16(value[0] as u8),
                0 => {
                    self.current_fg = DEFAULT_FG;
                    self.current_bg = DEFAULT_BG;
                }
                v => {
                    panic!("Unhandled color mode {v}");
                }
            }
        }
    }
}
