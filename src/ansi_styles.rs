macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

const ANSI_16_FG_TO_BG: u8 = 10;

/// The default foreground color.
pub const DEFAULT_FG: AnsiColor = AnsiColor::Ansi16(37);
/// The default background color.
pub const DEFAULT_BG: AnsiColor = AnsiColor::Ansi16(40);

pub const ANSI_RESET: &str = "\x1b[0m";

/// Represents an ANSI color.
#[derive(Debug, Clone, Copy)]
pub enum AnsiColor {
    Ansi16(u8),
    Ansi256(u8),
    Rgb(u8, u8, u8),
}

impl PartialEq for AnsiColor {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AnsiColor::Ansi16(a), AnsiColor::Ansi16(b)) => ansi_16_to_fg(*a) == ansi_16_to_fg(*b),
            (AnsiColor::Ansi256(a), AnsiColor::Ansi256(b)) => a == b,
            (AnsiColor::Rgb(ar, ag, ab), AnsiColor::Rgb(br, bg, bb)) => {
                ar == br && ag == bg && ab == bb
            }
            _ => false,
        }
    }
}

impl AnsiColor {
    /// Convert this color to a 16-color ANSI color.
    pub fn to_ansi_16(self) -> AnsiColor {
        match self {
            AnsiColor::Ansi16(c) => AnsiColor::Ansi16(ansi_16_to_fg(c)),
            AnsiColor::Ansi256(c) => AnsiColor::Ansi16(ansi_256_to_ansi_16(c)),
            AnsiColor::Rgb(r, g, b) => AnsiColor::Ansi16(rgb_to_ansi_16(r, g, b)),
        }
    }

    /// Convert this color to a 256-color ANSI color.
    pub fn to_ansi_256(self) -> AnsiColor {
        match self {
            AnsiColor::Ansi16(c) => {
                let c = match c {
                    (30..=37) => c - 30,
                    (40..=47) => c - 40,
                    (90..=97) => c - 82,
                    (100..=107) => c - 92,
                    _ => 0,
                };
                AnsiColor::Ansi256(c)
            }
            AnsiColor::Ansi256(_) => self,
            AnsiColor::Rgb(r, g, b) => AnsiColor::Ansi256(rgb_to_ansi_256(r, g, b)),
        }
    }

    /// Convert this color to a RGB color.
    pub fn to_rgb(self) -> AnsiColor {
        match self {
            AnsiColor::Ansi16(c) => {
                let (r, g, b) = ansi_16_to_rgb(c);
                AnsiColor::Rgb(r, g, b)
            }
            AnsiColor::Ansi256(c) => {
                let (r, g, b) = ansi_256_to_rgb(c);
                AnsiColor::Rgb(r, g, b)
            }
            AnsiColor::Rgb(_, _, _) => self,
        }
    }

    /// Return the a partial SGR to set the foreground to this color. The returned
    /// string will be missing the leading escape character and the trailing 'm'.
    pub fn fg_code(self) -> String {
        match self {
            AnsiColor::Ansi16(c) => format!("{}", ansi_16_to_fg(c)),
            AnsiColor::Ansi256(c) => format!("38;5;{}", c),
            AnsiColor::Rgb(r, g, b) => format!("38;2;{};{};{}", r, g, b),
        }
    }

    /// Return the a partial SGR to set the background to this color. The returned
    /// string will be missing the leading escape character and the trailing 'm'.
    pub fn bg_code(self) -> String {
        match self {
            AnsiColor::Ansi16(c) => format!("{}", ansi_16_to_bg(c)),
            AnsiColor::Ansi256(c) => format!("48;5;{}", c),
            AnsiColor::Rgb(r, g, b) => format!("48;2;{};{};{}", r, g, b),
        }
    }
}

/// Convert a color from the ANSI 256 color palette to the 16 color palette.
///
/// This returns a color code which can be used in an SGR block.
/// For example:
///
/// ```ignore
/// let code = ansi_256_to_ansi_16(78);
/// println!("\x1b[{code}mHello, world!\x1b[0m");
/// ```
fn ansi_256_to_ansi_16(c: u8) -> u8 {
    if c < 8 {
        return c + 30;
    }

    if c < 16 {
        return c + 92;
    }

    let (r, g, b) = ansi_256_to_rgb(c);
    rgb_to_ansi_16(r, g, b)
}

fn ansi_16_to_bg(c: u8) -> u8 {
    match c {
        (30..=37) => c + ANSI_16_FG_TO_BG,
        (40..=47) => c,
        (90..=97) => c + ANSI_16_FG_TO_BG,
        (100..=107) => c,
        _ => 0,
    }
}

fn ansi_16_to_fg(c: u8) -> u8 {
    match c {
        (30..=37) => c,
        (40..=47) => c - ANSI_16_FG_TO_BG,
        (90..=97) => c,
        (100..=107) => c - ANSI_16_FG_TO_BG,
        _ => 0,
    }
}

/// Convert an RGB color to the ANSI 256 color palette.
///
/// This returns a color code which can be used in an SGR block.
/// For example:
///
/// ```ignore
/// let code = rgb_to_ansi_256(255, 0, 0);
/// println!("\x1b[38;5;{code}mHello, world!\x1b[0m");
/// ```
fn rgb_to_ansi_256(r: u8, g: u8, b: u8) -> u8 {
    // Adapted from https://github.com/Qix-/color-convert/blob/6b7dee5a168f76bf42c084fefa7bbe1a0941ad7e/conversions.js.

    // We use the extended greyscale palette here, with the exception of
    // black and white. normal palette only has 4 greyscale shades.
    if r >> 4 == g >> 4 && g >> 4 == b >> 4 {
        if r < 8 {
            return 16;
        }

        if r > 248 {
            return 231;
        }

        (((r as f64 - 8.0) / 247.0) * 24.0).round() as u8 + 232
    } else {
        16 + (36 * (r as f64 / 255.0 * 5.0).round() as u8)
            + (6 * (g as f64 / 255.0 * 5.0).round() as u8)
            + (b as f64 / 255.0 * 5.0).round() as u8
    }
}

fn ansi_16_to_rgb(c: u8) -> (u8, u8, u8) {
    match c {
        30 | 40 => (0, 0, 0),
        31 | 41 => (128, 0, 0),
        32 | 42 => (0, 128, 0),
        33 | 43 => (128, 128, 0),
        34 | 44 => (0, 0, 128),
        35 | 45 => (128, 0, 128),
        36 | 46 => (0, 128, 128),
        37 | 47 => (192, 192, 192),
        90 | 100 => (0, 0, 0),
        91 | 101 => (255, 0, 0),
        92 | 102 => (0, 255, 0),
        93 | 103 => (255, 255, 0),
        94 | 104 => (0, 0, 255),
        95 | 105 => (255, 0, 255),
        96 | 106 => (0, 255, 255),
        97 | 107 => (255, 255, 255),
        _ => (0, 0, 0),
    }
}

fn ansi_256_to_rgb(c: u8) -> (u8, u8, u8) {
    // Adapted from https://github.com/Qix-/color-convert/blob/6b7dee5a168f76bf42c084fefa7bbe1a0941ad7e/conversions.js.

    // Handle greyscale
    if c >= 232 {
        let c = (c - 232) * 10 + 8;
        return (c, c, c);
    }

    if c < 8 {
        return ansi_16_to_rgb(c + 30);
    }

    if c < 16 {
        return ansi_16_to_rgb(c + 92);
    }

    let c = c - 16;

    let r = ((c as f64 / 36.0) / 5.0 * 255.0) as u8;
    let rem = c % 36;
    let g = ((rem as f64 / 6.0) / 5.0 * 255.0) as u8;
    let b = ((rem % 6) as f64 / 5.0 * 255.) as u8;

    (r, g, b)
}

/// Convert an RGB color to the ANSI 16 color palette.
///
/// This returns a color code which can be used in an SGR block.
/// For example:
///
/// ```ignore
/// let code = rgb_to_ansi_16(78);
/// println!("\x1b[{code}mHello, world!\x1b[0m");
/// ```
fn rgb_to_ansi_16(r: u8, g: u8, b: u8) -> u8 {
    // Adapted from https://github.com/Qix-/color-convert/blob/6b7dee5a168f76bf42c084fefa7bbe1a0941ad7e/conversions.js.
    let value = rbg_to_hsv(r, g, b).2;
    let value = (value / 50.0).round() as u8;

    if value == 0 {
        return 30;
    }

    let mut ansi = 30
        + (((b as f64 / 255.0).round() as u8 * 4)
            | ((g as f64 / 255.0).round() as u8 * 2)
            | (r as f64 / 255.0).round() as u8);

    if value == 2 {
        ansi += 60;
    }

    ansi
}

fn rbg_to_hsv(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    // Adapted from https://github.com/Qix-/color-convert/blob/6b7dee5a168f76bf42c084fefa7bbe1a0941ad7e/conversions.js.
    let mut h;
    let s;

    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;
    let v = max!(r, b, g);
    let diff = v - min!(r, b, g);
    let diffc = |c: f64| (v - c) / 6.0 / diff + 1.0 / 2.0;

    if diff == 0.0 {
        h = 0.0;
        s = 0.0;
    } else {
        s = diff / v;
        let rdif = diffc(r);
        let gdif = diffc(g);
        let bdif = diffc(b);

        if r == v {
            h = bdif - gdif;
        } else if g == v {
            h = (1.0 / 3.0) + rdif - bdif;
        } else { // b == v
            h = (2.0 / 3.0) + gdif - rdif;
        }

        if h < 0.0 {
            h += 1.0;
        } else if h > 1.0 {
            h -= 1.0;
        }
    }

    (h * 360.0, s * 100.0, v * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_white() {
        let white = AnsiColor::Rgb(255, 255, 255);
        assert_eq!(white.to_ansi_16(), AnsiColor::Ansi16(97));
        assert_eq!(white.to_ansi_256(), AnsiColor::Ansi256(231));

        let white = AnsiColor::Ansi16(97);
        assert_eq!(white.to_rgb(), AnsiColor::Rgb(255, 255, 255));
        assert_eq!(white.to_ansi_256(), AnsiColor::Ansi256(15));

        let white = AnsiColor::Ansi256(231);
        assert_eq!(white.to_rgb(), AnsiColor::Rgb(255, 255, 255));
        assert_eq!(white.to_ansi_16(), AnsiColor::Ansi16(97));

        let white = AnsiColor::Ansi256(15);
        assert_eq!(white.to_rgb(), AnsiColor::Rgb(255, 255, 255));
        assert_eq!(white.to_ansi_16(), AnsiColor::Ansi16(97));
    }

    #[test]
    fn should_convert_high_and_low_intensity() {
        let bright_blue = AnsiColor::Rgb(0, 0, 255);
        assert_eq!(bright_blue.to_ansi_16(), AnsiColor::Ansi16(94));
        assert_eq!(bright_blue.to_ansi_256(), AnsiColor::Ansi256(21));

        let dim_blue = AnsiColor::Rgb(0, 0, 128);
        assert_eq!(dim_blue.to_ansi_16(), AnsiColor::Ansi16(34));
        assert_eq!(dim_blue.to_ansi_256(), AnsiColor::Ansi256(19));
    }

    #[test]
    fn foreground_and_background_should_be_equal() {
        let black_bg = AnsiColor::Ansi16(40);
        let black_fg = AnsiColor::Ansi16(30);
        assert_eq!(black_bg, black_fg);
    }
}
