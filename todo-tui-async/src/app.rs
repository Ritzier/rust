use std::time::Duration;

use crossterm::event::Event as CrosstermEvent;
use crossterm::event::EventStream as CrosstermEventStream;
use crossterm::event::{KeyCode, KeyEventKind};
use futures::FutureExt;
use futures::StreamExt;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use todo::TodoList;
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    time,
};

use crate::{error::Error, Result};

mod handle_key;
mod todo;
mod ui;

pub struct App {
    todo_input: String,
    description_input: String,
    appstate: AppState,
    todo_list: TodoList,
    file: String,
    quit: bool,
    currently_editing: Option<CurrentlyEditing>,
    frame_rate: f64,
    event_tx: UnboundedSender<Event>,
    event_rx: UnboundedReceiver<Event>,
    crossterm_event: CrosstermEventStream,
}

pub enum Event {
    Frame,
    Key(KeyCode),
}

#[derive(PartialEq)]
enum AppState {
    Main,
    Adding,
    Editing(usize),
    Deleting(usize),
    Exiting,
}

enum CurrentlyEditing {
    Todo,
    Description,
}

impl App {
    pub async fn new(file: String, frame_rate: f64) -> Result<Self> {
        let (event_tx, event_rx) = unbounded_channel();
        Ok(Self {
            todo_input: String::new(),
            description_input: String::new(),
            appstate: AppState::Main,
            todo_list: TodoList::new(&file).await?,
            file,
            currently_editing: None,
            quit: false,
            frame_rate,
            event_tx,
            event_rx,
            crossterm_event: CrosstermEventStream::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        startup()?;

        let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
        let frame_rate = Duration::from_secs_f64(1.0 / self.frame_rate);
        let mut frame_interval = time::interval(frame_rate);

        while !self.quit {
            tokio::select! {
                _frame = frame_interval.tick() => {
                    self.event_tx.send(Event::Frame)?
                }
                Some(event) = self.event_rx.recv() => {
                    match event {
                        Event::Frame => {
                        terminal.draw(|frame|{
                            self.render(frame.area(),frame);
                        })?;
                        }
                        Event::Key(key) => {
                            self.handle_key(key)?
                        }
                    }
                }
                event = self.crossterm_event.next().fuse() => {
                    match event.ok_or(Error::CrosstermError)?? {
                        CrosstermEvent::Key(key) => {
                            if let KeyEventKind::Press = key.kind {
                                self.event_tx.send(Event::Key(key.code))?
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        self.todo_list.to_json(&self.file).await?;

        shutdown()
    }
}

fn startup() -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
