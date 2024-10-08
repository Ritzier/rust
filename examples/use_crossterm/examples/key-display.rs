use std::io;

use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

const HELP: &str = r#"Key display
- Press any key to see its display format
- Use Esc to quit
"#;

fn main() -> io::Result<()> {
    println!("{}", HELP);
    enable_raw_mode()?;
    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e)
    }
    disable_raw_mode()?;
    Ok(())
}

fn print_events() -> io::Result<()> {
    loop {
        let event = read()?;
        match event {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                println!("Key pressed: ");
                if event.modifiers != KeyModifiers::NONE {
                    println!("{}+", event.modifiers);
                }
                println!("{}\r", event.code);
                if event.code == KeyCode::Esc {
                    break;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
