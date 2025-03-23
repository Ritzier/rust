use std::{collections::HashMap, io::Stdout, time::Duration};

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyCode, KeyEventKind};
use futures::{FutureExt, StreamExt};
use ratatui::{prelude::CrosstermBackend, widgets::ListState, Terminal};
use reqwest::Client;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::{Error, Result};

mod downloader;
mod handle_key;
mod ui;

pub enum Event {
    Key(KeyCode),
    Tick,
    Frame,
    DownloadStatus(String, DownloadItem),
}

#[derive(PartialEq)]
pub enum AppState {
    Main,
    Input,
}

#[derive(Clone)]
pub struct DownloadItem {
    status: DownloadStatus,
    loading_frame: usize,
}

#[derive(Clone, PartialEq)]
pub enum DownloadStatus {
    Loading,
    Error(String),
    Complete(String),
}

pub struct App {
    should_quit: bool,
    crossterm_event: EventStream,
    frame_rate: f64,
    tick_rate: f64,
    event_rx: UnboundedReceiver<Event>,
    event_tx: UnboundedSender<Event>,
    appstate: AppState,
    client: Client,
    loading_frames: Vec<&'static str>,
    url_input: String,
    list_state: ListState,
    download_hashmap: HashMap<String, DownloadItem>,
}

impl App {
    pub fn new(frame_rate: f64, tick_rate: f64) -> Result<Self> {
        let (event_tx, event_rx) = unbounded_channel();
        let crossterm_event = EventStream::new();

        Ok(Self {
            should_quit: false,
            event_rx,
            event_tx,
            frame_rate,
            tick_rate,
            crossterm_event,
            appstate: AppState::Main,
            client: Client::new(),
            loading_frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            url_input: String::new(),
            list_state: ListState::default(),
            download_hashmap: HashMap::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        startup()?;

        let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

        let frame_rate = Duration::from_secs_f64(1.0 / self.frame_rate);
        let tick_rate = Duration::from_secs_f64(1.0 / self.tick_rate);
        let mut frame_interval = tokio::time::interval(frame_rate);
        let mut tick_interval = tokio::time::interval(tick_rate);

        while !self.should_quit {
            tokio::select! {
                _tick = tick_interval.tick() => {
                    self.event_tx.send(Event::Tick).unwrap();
                }
                _frame = frame_interval.tick() => {
                    self.event_tx.send(Event::Frame).unwrap();
                }
                Some(event) = self.event_rx.recv() => {
                    self.handle_event(&mut terminal,event).await?;
                }
                event = self.crossterm_event.next().fuse() => {
                    match event.ok_or(Error::Crossterm)?? {
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

        shutdown()
    }

    async fn handle_event(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        event: Event,
    ) -> Result<()> {
        match event {
            Event::Key(key) => self.handle_key_event(key).await?,
            Event::Frame => {
                terminal.draw(|frame| {
                    frame.render_widget(self, frame.area());
                })?;
            }
            Event::DownloadStatus(username, downloaditem) => {
                self.download_hashmap
                    .entry(username.to_string())
                    .and_modify(|user| *user = downloaditem.clone())
                    .or_insert(downloaditem);
            }
            Event::Tick => {
                self.download_hashmap.iter_mut().for_each(|(_i, item)| {
                    if item.status == DownloadStatus::Loading {
                        item.loading_frame = (item.loading_frame + 1) % self.loading_frames.len();
                    }
                });
            }
        }

        Ok(())
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
