use inquire::Confirm;

fn main() {
    let answ = Confirm::new("Are you binary?")
        .with_default(false)
        .with_help_message("Please answer it")
        .prompt();

    match answ {
        Ok(true) => println!("Ha! Gotcha"),
        Ok(false) => println!("Hmmmm, pass it"),
        Err(_) => println!("Error with questionnaire, try again later"),
    }
}
