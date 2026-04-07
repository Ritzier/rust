use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc::{self, Receiver};
use tokio::task::{JoinError, JoinHandle};
use watchexec::error::CriticalError;
use watchexec::{WatchedPath, Watchexec};
use watchexec_events::filekind::FileEventKind;
use watchexec_events::{Event as WatchexecEvent, Tag};
use watchexec_signals::Signal;

#[derive(Debug)]
pub enum FileEvent {
    Create,
    Remove,
    Modify,
}

pub struct Watcher {
    pub watchexec_task: JoinHandle<Result<Result<(), CriticalError>, JoinError>>,
    pub event_receiver: Receiver<HashMap<PathBuf, FileEvent>>,
}

impl Watcher {
    pub fn build(path: &[&str]) -> Result<Self> {
        let (event_sender, event_receiver) = mpsc::channel(32);

        let wx = Watchexec::new(move |mut action| {
            if action.signals().any(|sig| sig == Signal::Interrupt) {
                action.quit()
            }

            if let Some(event) = Self::handle_event(&action.events)
                && let Err(e) = event_sender.try_send(event)
            {
                eprintln!("{e}")
            }

            action
        })?;

        wx.config
            .pathset(path.iter().map(|&p| WatchedPath::from(p)));

        let watchexec_task = tokio::spawn(async move { wx.main().await });

        Ok(Self {
            watchexec_task,
            event_receiver,
        })
    }

    pub fn handle_event(events: &Arc<[WatchexecEvent]>) -> Option<HashMap<PathBuf, FileEvent>> {
        let mut seen: HashMap<PathBuf, FileEvent> = HashMap::new();

        for action_event in events.iter() {
            let mut path = Option::<PathBuf>::None;
            let mut event = Option::<FileEvent>::None;

            for tag in &action_event.tags {
                match tag {
                    Tag::Path { path: tag_path, .. } => {
                        path = Some(tag_path.clone());
                    }

                    Tag::FileEventKind(tag_event_kind) => match tag_event_kind {
                        FileEventKind::Any | FileEventKind::Access(_) | FileEventKind::Other => {
                            continue;
                        }

                        FileEventKind::Create(_) => {
                            event = Some(FileEvent::Create);
                        }

                        FileEventKind::Remove(_) => {
                            event = Some(FileEvent::Remove);
                        }
                        FileEventKind::Modify(_) => {
                            event = Some(FileEvent::Modify);
                        }
                    },

                    _ => {}
                }
            }

            if let Some(path) = path
                && let Some(event_kind) = event
            {
                match seen.get_mut(&path) {
                    Some(seen_event_kind) => {
                        fn to_num(event_kind: &FileEvent) -> u8 {
                            match event_kind {
                                FileEvent::Remove => 3,
                                FileEvent::Create => 2,
                                FileEvent::Modify => 1,
                            }
                        }

                        let seen_event_kind_num = to_num(seen_event_kind);
                        let event_kind_num = to_num(&event_kind);

                        if event_kind_num > seen_event_kind_num {
                            seen.insert(path, event_kind);
                        }
                    }

                    None => {
                        seen.insert(path, event_kind);
                    }
                }
            }
        }

        match seen.is_empty() {
            true => None,
            false => Some(seen),
        }
    }
}
