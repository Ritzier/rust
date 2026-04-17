use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::path::{Path, PathBuf, absolute};
use std::sync::Arc;

use globset::GlobSet;
use tokio::sync::mpsc::{self, Receiver};
use tokio::sync::{Mutex, RwLock, oneshot};
use tokio::task::{JoinError, JoinHandle};
use watchexec::action::ActionHandler;
use watchexec::error::CriticalError;
use watchexec::{WatchedPath, Watchexec};
use watchexec_events::filekind::FileEventKind;
use watchexec_events::{Event as WatchexecEvent, Tag};
use watchexec_signals::Signal;

use crate::include::include_updater::{IncludeUpdater, IncludeUpdaterInit};
use crate::{Error, IncludeSender};

use super::event::Event;

#[derive(Debug, PartialEq)]
pub enum FileEvent {
    Create,
    Remove,
    Modify,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FileType {
    Config,
    File,
}

#[derive(Debug)]
pub struct Watcher {
    pub watchexec_task: JoinHandle<Result<Result<(), CriticalError>, JoinError>>,
    pub event_receiver: Receiver<Event>,
    pub startup_rx: oneshot::Receiver<()>,
    pub include_updater_task: JoinHandle<Result<(), Error>>,
    pub include_sender: IncludeSender,
}

impl Watcher {
    pub fn build<P: Into<WatchedPath> + AsRef<Path>>(configuration_path: P) -> Result<Self, Error>
where {
        let configuration =
            absolute(&configuration_path).map_err(|_| Error::PathIsNotValidUTF8 {
                pathbuf: configuration_path.as_ref().to_path_buf(),
            })?;

        if !configuration.exists() {
            return Err(Error::ConfigurationNotExists {
                path: configuration,
            });
        }

        let (event_sender, event_receiver) = mpsc::channel(32);
        let (startup_tx, startup_rx) = oneshot::channel();
        let arc_globset = Arc::new(RwLock::new(GlobSet::empty()));

        let event_sender = Arc::new(Mutex::new(Some(event_sender)));
        let configuration_clone = configuration.clone();
        let arc_globset_clone = arc_globset.clone();
        let wx = Watchexec::new_async(move |action| {
            let configuration = configuration_clone.clone();
            let arc_globset = arc_globset_clone.clone();
            let event_sender = event_sender.clone();

            Box::new(Self::handle_action(
                action,
                configuration,
                arc_globset,
                event_sender,
            ))
        })
        .map_err(Box::from)?;

        wx.config.pathset([configuration.clone()]);

        let startup_tx = Some(startup_tx);

        let wx_clone = wx.clone();
        let watchexec_task = tokio::spawn(async move {
            if let Some(tx) = startup_tx {
                let _ = tx.send(());
            }

            wx_clone.main().await
        });

        // `IncludeUpdater`
        let IncludeUpdaterInit {
            include_updater_task,
            include_sender,
        } = IncludeUpdater::build(wx, arc_globset, configuration);

        Ok(Self {
            watchexec_task,
            event_receiver,
            startup_rx,
            include_updater_task,
            include_sender,
        })
    }

    async fn handle_action(
        mut action: ActionHandler,
        configuration: PathBuf,
        arc_globset: Arc<RwLock<GlobSet>>,
        event_sender: Arc<Mutex<Option<mpsc::Sender<Event>>>>,
    ) -> ActionHandler {
        if action.signals().any(|sig| sig == Signal::Interrupt) {
            action.quit();
        }

        if let Some(map) = Self::handle_event(&action.events, &configuration, &arc_globset).await
            && let Some(event) = Self::merge_events(map)
        {
            let mut sender_guard = event_sender.lock().await;

            if let Some(sender) = sender_guard.as_ref()
                && let Err(e) = sender.try_send(event)
            {
                eprintln!("{e}");
            }

            // Close `channel` when `ConfigRemove`
            if event == Event::ConfigRemove {
                sender_guard.take();
            }
        }

        action
    }

    pub async fn handle_event(
        events: &Arc<[WatchexecEvent]>,
        configuration: &PathBuf,
        arc_globset: &Arc<RwLock<GlobSet>>,
    ) -> Option<HashMap<FileType, FileEvent>> {
        let mut seen: HashMap<FileType, FileEvent> = HashMap::new();
        let globset_read = arc_globset.read().await;

        for action_event in events.iter() {
            let mut path = Option::<FileType>::None;
            let mut event = Option::<FileEvent>::None;

            for tag in &action_event.tags {
                match tag {
                    Tag::Path { path: tag_path, .. } => {
                        if tag_path == configuration {
                            path = Some(FileType::Config);
                            continue;
                        }
                        if globset_read.is_match(tag_path) {
                            path = Some(FileType::File);
                            continue;
                        }
                    }
                    Tag::FileEventKind(tag_event_kind) => match tag_event_kind {
                        FileEventKind::Any | FileEventKind::Access(_) | FileEventKind::Other => {
                            continue;
                        }
                        FileEventKind::Create(_) => event = Some(FileEvent::Create),
                        FileEventKind::Remove(_) => event = Some(FileEvent::Remove),
                        FileEventKind::Modify(_) => event = Some(FileEvent::Modify),
                    },
                    _ => {}
                }
            }

            if let (Some(path), Some(event_kind)) = (path, event) {
                fn priority(e: &FileEvent) -> u8 {
                    match e {
                        FileEvent::Remove => 3,
                        FileEvent::Create => 2,
                        FileEvent::Modify => 1,
                    }
                }
                match seen.entry(path) {
                    Entry::Occupied(mut entry) => {
                        if priority(&event_kind) > priority(entry.get()) {
                            entry.insert(event_kind);
                        }
                    }

                    Entry::Vacant(entry) => {
                        entry.insert(event_kind);
                    }
                }
            }
        }

        match seen.is_empty() {
            true => None,
            false => Some(seen),
        }
    }

    pub fn merge_events(seen: HashMap<FileType, FileEvent>) -> Option<Event> {
        match (seen.get(&FileType::Config), seen.get(&FileType::File)) {
            (None, None) => None,

            // Highest
            (Some(FileEvent::Remove), _) => Some(Event::ConfigRemove),

            // Config only
            (Some(FileEvent::Create), None) => Some(Event::ConfigCreate),
            (Some(FileEvent::Modify), None) => Some(Event::ConfigModify),

            // File only
            (None, Some(FileEvent::Create)) => Some(Event::FileCreate),
            (None, Some(FileEvent::Modify)) => Some(Event::FileModify),
            (None, Some(FileEvent::Remove)) => Some(Event::FileRemove),

            // Both changed — collapse to ConfigFileModify or decide per variant
            (Some(_), Some(_)) => Some(Event::ConfigFileModify),
        }
    }
}
