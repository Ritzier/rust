use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

fn main() {
    // `Ctrl` + `z`
    match_event(Event::Key(KeyEvent::new(
        KeyCode::Char('z'),
        KeyModifiers::CONTROL,
    )));

    // `Shift` + `Left`
    match_event(Event::Key(KeyEvent::new(
        KeyCode::Left,
        KeyModifiers::SHIFT,
    )));

    // `Alt` + `Delete`
    match_event(Event::Key(KeyEvent::new(
        KeyCode::Delete,
        KeyModifiers::ALT,
    )));

    // `Ctrl` + `Alt` + `Right`
    match_event(Event::Key(KeyEvent::new(
        KeyCode::Right,
        KeyModifiers::ALT | KeyModifiers::CONTROL,
    )));

    // `Win` + `Alt` + `Home`
    match_event(Event::Key(KeyEvent::new(
        KeyCode::Home,
        KeyModifiers::SUPER | KeyModifiers::ALT,
    )))
}

fn match_event(read_event: Event) {
    match read_event {
        // Match one modifiers:
        Event::Key(KeyEvent {
            modifiers: KeyModifiers::CONTROL,
            code,
            ..
        }) => {
            println!("Control + {:?}", code);
        }
        Event::Key(KeyEvent {
            modifiers: KeyModifiers::SHIFT,
            code,
            ..
        }) => {
            println!("Shift + {:?}", code);
        }
        Event::Key(KeyEvent {
            modifiers: KeyModifiers::ALT,
            code,
            ..
        }) => {
            println!("Alt + {:?}", code);
        }

        // Match on multiple modifiers:
        Event::Key(KeyEvent {
            code, modifiers, ..
        }) => {
            if modifiers == (KeyModifiers::ALT | KeyModifiers::SHIFT) {
                println!("Alt + Shift {:?}", code);
            } else {
                println!("({:?}) with key: {:?}", modifiers, code)
            }
        }

        _ => {}
    }
}
