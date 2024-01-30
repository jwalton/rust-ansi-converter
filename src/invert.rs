/// Given a unicode block character, returns the inverted block character.  For
/// example, given a half-block with the bottom half filled in, will return a block
/// with the top half filled in.  Given a full block, will return a space.
///
pub fn invert_char(c: char) -> char {
    match c {
        ' ' => '█',
        '█' => ' ',
        '░' => '▓',
        '▒' => '▒',
        '▓' => '░',

        // Vertical eighths blocks
        '▁' => '🮆',
        '▂' => '🮅',
        '▃' => '🮄',
        '▄' => '▀',
        '▅' => '🮃',
        '▆' => '🮂',
        '▇' => '▔',
        '🮆' => '▁',
        '🮅' => '▂',
        '🮄' => '▃',
        '▀' => '▄',
        '🮃' => '▅',
        '🮂' => '▆',
        '▔' => '▇',

        // Horizontal eighths blocks
        '▏' => '🮋',
        '▎' => '🮊',
        '▍' => '🮉',
        '▌' => '▐',
        '▋' => '🮈',
        '▊' => '🮇',
        '▉' => '▕',
        '🮋' => '▏',
        '🮊' => '▎',
        '🮉' => '▍',
        '▐' => '▌',
        '🮈' => '▋',
        '🮇' => '▊',
        '▕' => '▉',

        // Quarter blocks
        '▘' => '▟',
        '▟' => '▘',
        '▝' => '▙',
        '▙' => '▝',
        '▖' => '▜',
        '▜' => '▖',
        '▗' => '▛',
        '▛' => '▗',
        '▚' => '▞',
        '▞' => '▚',

        c => c,
    }
}

/// Returns true if we can invert the character.
pub fn can_invert_char(c: char) -> bool {
    c == '▒' || invert_char(c) != c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_invert_blocks() {
        assert_eq!(invert_char('█'), ' ');
        assert_eq!(invert_char(' '), '█');
        assert_eq!(invert_char('▌'), '▐');
        assert_eq!(invert_char('▐'), '▌');
    }
}
