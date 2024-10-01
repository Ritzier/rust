use inquire::{ui::RenderConfig, Confirm};

fn main() {
    let ans = Confirm {
        // default: when user didn't give answer, `default` will be given
        // default_value_formatter: when default given, default_value_formatter will show between `message` and `user_inptu`
        // starting_input: preinput
        // placeholdler: showing when user input empty
        // parser: parse user input to the backend and formatter
        // formatter would receive from parser
        message: "Are you a robot?",
        starting_input: None,
        default: Some(false),
        placeholder: Some("10|no"),
        help_message: Some("Answer it!!!"),
        formatter: &|ans| match ans {
            true => "Robot".to_owned(),
            false => "Human".to_owned(),
        },
        parser: &|ans| match ans {
            "10" => Ok(true),
            "no" => Ok(false),
            _ => Err(()),
        },
        error_message: "Reply with '10' or 'no'".into(),
        default_value_formatter: &|def| match def {
            true => String::from("Robot?"),
            false => String::from("Human?"),
        },
        render_config: RenderConfig::default(),
    }
    .prompt()
    .unwrap();

    println!("You answer: {ans}");
}
