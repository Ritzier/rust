use std::fmt::{Display, Formatter};

use inquire::{error::InquireResult, Password, Select};

#[derive(Debug, Clone, Copy)]
enum State {
    Login,
    Nothing,
    Quit,
}

impl State {
    const VARIANTS: &'static [State] = &[State::Login, State::Nothing, State::Quit];
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy)]
enum Login {
    QR,
    Code,
    Password,
}

impl Login {
    const VARIANTS: &'static [Login] = &[Login::QR, Login::Code, Login::Password];
}

impl Display for Login {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

fn main() -> InquireResult<()> {
    run()
}

fn run() -> InquireResult<()> {
    loop {
        match main_menu()? {
            State::Login => login_menu()?,
            State::Nothing => continue,
            State::Quit => break,
        }
    }
    Ok(())
}

fn main_menu() -> InquireResult<State> {
    let options = State::VARIANTS.to_vec();
    let ans = Select::new("Choose an option:", options).prompt()?;

    Ok(ans)
}

fn login_menu() -> InquireResult<()> {
    let options = Login::VARIANTS.to_vec();
    let ans = Select::new("Choose an option:", options).prompt()?;

    match ans {
        Login::QR => println!("QR"),
        Login::Code => println!("Code"),
        Login::Password => println!("Password"),
    }

    Ok(())
}
