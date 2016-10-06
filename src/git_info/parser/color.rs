use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TerminalColor {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BoldBlack,
    BoldRed,
    BoldGreen,
    BoldYellow,
    BoldBlue,
    BoldMagenta,
    BoldCyan,
    BoldWhite,
}

impl TerminalColor {
    pub fn convert(color_name: &[u8]) -> TerminalColor {
       match color_name {
            b"black" => TerminalColor::Black,
            b"red" => TerminalColor::Red,
            b"green" => TerminalColor::Green,
            b"yellow" => TerminalColor::Yellow,
            b"blue" => TerminalColor::Blue,
            b"magenta" => TerminalColor::Magenta,
            b"cyan" => TerminalColor::Cyan,
            b"white" => TerminalColor::White,
            b"bold_black" => TerminalColor::BoldBlack,
            b"bold_red" => TerminalColor::BoldRed,
            b"bold_green" => TerminalColor::BoldGreen,
            b"bold_yellow" => TerminalColor::BoldYellow,
            b"bold_blue" => TerminalColor::BoldBlue,
            b"bold_magenta" => TerminalColor::BoldMagenta,
            b"bold_cyan" => TerminalColor::BoldCyan,
            b"bold_white" => TerminalColor::BoldWhite,
            &_ => TerminalColor::Reset,
       }
    }
}

impl fmt::Display for TerminalColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Get the ANSI code for a specific color
        let color_code = match *self {
            TerminalColor::Reset => "0",
            TerminalColor::Black => "0;30",
            TerminalColor::Red => "0;31",
            TerminalColor::Green => "0;32",
            TerminalColor::Yellow => "0;33",
            TerminalColor::Blue => "0;34",
            TerminalColor::Magenta => "0;35",
            TerminalColor::Cyan => "0;36",
            TerminalColor::White => "0;37",
            TerminalColor::BoldBlack => "1;30",
            TerminalColor::BoldRed => "1;31",
            TerminalColor::BoldGreen => "1;32",
            TerminalColor::BoldYellow => "1;33",
            TerminalColor::BoldBlue => "1;34",
            TerminalColor::BoldMagenta => "1;35",
            TerminalColor::BoldCyan => "1;36",
            TerminalColor::BoldWhite => "1;37",
        };

        write!(f, "\x1b[{}m", color_code)
    }
}

#[cfg(test)]
mod tests {
    use super::TerminalColor;

    #[test]
    fn enum_to_output() {
        assert_eq!(
            format!("{}", TerminalColor::BoldRed),
            "\x1b[1;31m"
        );
    }

    #[test]
    fn input_to_enum() {
        assert_eq!(
            TerminalColor::convert(b"black"),
            TerminalColor::Black
        );
    }
}
