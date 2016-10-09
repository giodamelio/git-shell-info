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
    TrueColor(u8, u8, u8),
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
            TerminalColor::Reset => "0".to_string(),
            TerminalColor::Black => "0;30".to_string(),
            TerminalColor::Red => "0;31".to_string(),
            TerminalColor::Green => "0;32".to_string(),
            TerminalColor::Yellow => "0;33".to_string(),
            TerminalColor::Blue => "0;34".to_string(),
            TerminalColor::Magenta => "0;35".to_string(),
            TerminalColor::Cyan => "0;36".to_string(),
            TerminalColor::White => "0;37".to_string(),
            TerminalColor::BoldBlack => "1;30".to_string(),
            TerminalColor::BoldRed => "1;31".to_string(),
            TerminalColor::BoldGreen => "1;32".to_string(),
            TerminalColor::BoldYellow => "1;33".to_string(),
            TerminalColor::BoldBlue => "1;34".to_string(),
            TerminalColor::BoldMagenta => "1;35".to_string(),
            TerminalColor::BoldCyan => "1;36".to_string(),
            TerminalColor::BoldWhite => "1;37".to_string(),
            TerminalColor::TrueColor(r, g, b) => {
                format!("38;2;{};{};{}", r, g, b)
            }
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

        assert_eq!(
            format!("{}", TerminalColor::TrueColor(255, 0, 255)),
            "\x1b[38;2;255;0;255m"
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
