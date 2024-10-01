use inquire::{validator::Validation, Password, PasswordDisplayMode};

fn main() {
    let validator = |input: &str| {
        if input.chars().count() < 10 {
            Ok(Validation::Invalid(
                "Keys must have at least 10 characters.".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    let passwd = Password::new("Encryption key:")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_custom_confirmation_message("Encryption key (confirm)")
        .with_validator(validator)
        .with_formatter(&|_| String::from("Key received"))
        .prompt();

    match passwd {
        Ok(psw) => println!("Ha! I got it: {psw}"),
        Err(_) => println!("An error happened when asking for your key, try again later."),
    }
}
