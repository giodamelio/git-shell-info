use git2::Repository;

pub fn color_handler(args: Vec<&str>, _: Option<&Repository>) -> Result<String, String> {
    if args.len() == 1 {
        let color_code = match args[0] {
            "reset" => Ok("0"),
            "black" => Ok("0;30"),
            "red" => Ok("0;31"),
            "green" => Ok("0;32"),
            "yellow" => Ok("0;33"),
            "blue" => Ok("0;34"),
            "magenta" => Ok("0;35"),
            "cyan" => Ok("0;36"),
            "white" => Ok("0;37"),
            "bold_black" => Ok("1;30"),
            "bold_red" => Ok("1;31"),
            "bold_green" => Ok("1;32"),
            "bold_yellow" => Ok("1;33"),
            "bold_blue" => Ok("1;34"),
            "bold_magenta" => Ok("1;35"),
            "bold_cyan" => Ok("1;36"),
            "bold_white" => Ok("1;37"),
            _ => Err("color: must be a valid color name".to_string()),
        };

        color_code.map(|c| format!("\x1b[{}m", c))
    } else {
        Err("color: must have one arguments".to_string())
    }
}

pub fn rgb_handler(args: Vec<&str>, _: Option<&Repository>) -> Result<String, String> {
    if args.len() == 3 {
        let nums: Result<Vec<u8>, _> = args.iter()
            .map(|n| n.parse::<u8>())
            .collect();

        match nums {
            Ok(_) => Ok(format!("\x1b[38;2;{};{};{}m", args[0], args[1], args[2])),
            Err(_) => Err("rbg: arguments must be numbers between 0 and 255".to_string()),
        }
    } else {
        Err("rgb: must have three arguments".to_string())
    }

}

#[cfg(test)]
mod tests {
    use git2::Repository;

    use super::{color_handler, rgb_handler};

    #[test]
    fn valid_color() {
        assert_eq!(
            color_handler(vec!["red"], None).unwrap(),
            "\x1b[0;31m"
        );

        assert_eq!(
            color_handler(vec!["bold_cyan"], None).unwrap(),
            "\x1b[1;36m"
        );
    }

    #[test]
    fn invalid_color() {
        assert_eq!(
            color_handler(vec!["not a color"], None).err(),
            Some("color: must be a valid color name".to_string())
        );
    }

    #[test]
    fn incorrect_parameters_color() {
        assert_eq!(
            color_handler(vec!["red", "blue"], None).err(),
            Some("color: must have one arguments".to_string())
        );

        assert_eq!(
            color_handler(vec![], None).err(),
            Some("color: must have one arguments".to_string())
        );
    }

    #[test]
    fn valid_rgb() {
        assert_eq!(
            rgb_handler(vec!["255", "0", "222"], None).unwrap(),
            "\x1b[38;2;255;0;222m"
        );
    }

    #[test]
    fn invalid_rgb() {
        assert_eq!(
            rgb_handler(vec!["100000", "0", "222"], None).err(),
            Some("rbg: arguments must be numbers between 0 and 255".to_string())
        );
    }

    #[test]
    fn incorrect_parameters_rgb() {
        assert_eq!(
            rgb_handler(vec!["222"], None).err(),
            Some("rgb: must have three arguments".to_string())
        );

        assert_eq!(
            rgb_handler(vec![], None).err(),
            Some("rgb: must have three arguments".to_string())
        );
    }
}
