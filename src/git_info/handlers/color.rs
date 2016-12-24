pub fn color_handler(args: Vec<&str>) -> String {
    if args.len() == 1 {
        let color_code = (match args[0] {
            "reset" => "0",
            "black" => "0;30",
            "red" => "0;31",
            "green" => "0;32",
            "yellow" => "0;33",
            "blue" => "0;34",
            "magenta" => "0;35",
            "cyan" => "0;36",
            "white" => "0;37",
            "bold_black" => "1;30",
            "bold_red" => "1;31",
            "bold_green" => "1;32",
            "bold_yellow" => "1;33",
            "bold_blue" => "1;34",
            "bold_magenta" => "1;35",
            "bold_cyan" => "1;36",
            "bold_white" => "1;37",
            _ => "0",
        }).to_string();

        format!("\x1b[{}m", color_code)
    } else {
        "".to_string()
    }
}

pub fn rgb_handler(args: Vec<&str>) -> String {
    if args.len() == 3 {
        let nums: Result<Vec<u8>, _> = args.iter()
            .map(|n| n.parse::<u8>())
            .collect();
        match nums {
            Ok(_) => format!("\x1b[38;2;{};{};{}m", args[0], args[1], args[2]),
            Err(_) => "".to_string(),
        }
    } else {
        "".to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::{color_handler, rgb_handler};

    #[test]
    fn valid_color() {
        assert_eq!(
            color_handler(vec!["red"]),
            "\x1b[0;31m"
        );

        assert_eq!(
            color_handler(vec!["bold_cyan"]),
            "\x1b[1;36m"
        );
    }

    #[test]
    fn invalid_color() {
        assert_eq!(
            color_handler(vec!["not a color"]),
            "\x1b[0m"
        );
    }

    #[test]
    fn incorrect_parameters_color() {
        assert_eq!(
            color_handler(vec!["red", "blue"]),
            ""
        );

        assert_eq!(
            color_handler(vec![]),
            ""
        );
    }

    #[test]
    fn valid_rgb() {
        assert_eq!(
            rgb_handler(vec!["255", "0", "222"]),
            "\x1b[38;2;255;0;222m"
        );
    }

    #[test]
    fn invalid_rgb() {
        assert_eq!(
            rgb_handler(vec!["100000", "0", "222"]),
            ""
        );
    }

    #[test]
    fn incorrect_parameters_rgb() {
        assert_eq!(
            rgb_handler(vec!["222"]),
            ""
        );

        assert_eq!(
            rgb_handler(vec![]),
            ""
        );
    }
}
