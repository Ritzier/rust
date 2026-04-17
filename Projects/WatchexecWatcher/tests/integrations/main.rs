mod workflow;

use std::path::{Path, PathBuf};
use std::time::Duration;

use futures::future::join_all;
use tempfile::TempDir;
use tokio::fs;
use tokio::sync::mpsc::Receiver;
use tokio::time::timeout;
use watchexec_watcher::{Error, Event, IncludeSender, Watcher};

const TIMEOUT: Duration = Duration::from_millis(800);

enum FileType {
    Config,
    File,
}

enum Action<'a> {
    Write(&'a [FileType]),
    Remove(&'a [FileType]),
}

struct Temp {
    _temp_dir: TempDir,
    config: PathBuf,
    file: PathBuf,
}

impl Temp {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let config = temp_path.join("config.toml");
        let file = temp_path.join("style.css");

        Self {
            _temp_dir: temp_dir,
            config,
            file,
        }
    }

    pub async fn action<'a>(&self, actions: &[Action<'a>]) {
        let actions_futures = actions.iter().map(|a| self.parse_action(a));

        join_all(actions_futures).await;
    }

    pub async fn parse_action<'a>(&self, action: &Action<'a>) {
        let file_types = match action {
            Action::Write(types) | Action::Remove(types) => types,
        };

        let futures = file_types.iter().map(|t| {
            let path = match t {
                FileType::Config => &self.config,
                FileType::File => &self.file,
            };

            async move {
                match action {
                    Action::Write(_) => {
                        fs::write(path, "content").await.unwrap();
                    }
                    Action::Remove(_) => {
                        fs::remove_file(path).await.unwrap();
                    }
                }
            }
        });

        join_all(futures).await;
    }

    fn file_string(&self) -> String {
        self.file.to_string_lossy().to_string()
    }
}

async fn setup_watcher(file: &Path) -> (Receiver<Event>, IncludeSender) {
    let Watcher {
        watchexec_task: _,
        event_receiver,
        startup_rx,
        include_updater_task: _,
        include_sender,
    } = Watcher::build(file.to_path_buf()).unwrap();
    startup_rx.await.unwrap();

    (event_receiver, include_sender)
}

#[macro_export]
macro_rules! assert_event {
    ($receiver:expr, $expected:expr) => {{
        let result = timeout(TIMEOUT, $receiver.recv()).await;
        assert_eq!(result, Ok(Some($expected)));
    }};
}
