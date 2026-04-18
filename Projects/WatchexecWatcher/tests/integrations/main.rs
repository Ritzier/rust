mod errors;
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
    Folder, // folder.a create dir
    Main,   // folder.main
    Lib,    // folder.lib
}

enum Action<'a> {
    Write(&'a [FileType]),
    Remove(&'a [FileType]),
}

struct Folder {
    a: PathBuf,
    main: PathBuf,
    lib: PathBuf,
}

impl Folder {
    pub fn new(temp_dir: &TempDir) -> Self {
        let a = temp_dir.path().join("src");
        let main = a.join("main.rs");
        let lib = a.join("lib.rs");

        Self { a, main, lib }
    }
}

struct Temp {
    _temp_dir: TempDir,
    config: PathBuf,
    file: PathBuf,
    folder: Folder,
}

impl Temp {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let config = temp_path.join("config.toml");
        let file = temp_path.join("style.css");
        let folder = Folder::new(&temp_dir);

        Self {
            _temp_dir: temp_dir,
            config,
            file,
            folder,
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

        let futures = file_types.iter().map(|t| async move {
            match action {
                Action::Write(_) => self.write(t).await,
                Action::Remove(_) => self.remove(t).await,
            }
        });

        join_all(futures).await;
    }

    async fn write(&self, t: &FileType) {
        match t {
            FileType::Folder => {
                fs::create_dir_all(self.resolve_path(t)).await.unwrap();
            }
            _ => {
                fs::write(self.resolve_path(t), "content").await.unwrap();
            }
        }
    }

    async fn remove(&self, t: &FileType) {
        match t {
            FileType::Folder => {
                fs::remove_dir_all(self.resolve_path(t)).await.unwrap();
            }
            _ => {
                fs::remove_file(self.resolve_path(t)).await.unwrap();
            }
        }
    }

    fn resolve_path(&self, t: &FileType) -> &Path {
        match t {
            FileType::Config => &self.config,
            FileType::File => &self.file,
            FileType::Folder => &self.folder.a,
            FileType::Main => &self.folder.main,
            FileType::Lib => &self.folder.lib,
        }
    }

    // `to_string`

    fn file_string(&self) -> String {
        self.file.to_string_lossy().to_string()
    }

    fn folder_string(&self) -> String {
        self.folder.a.to_string_lossy().to_string()
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
    // --- Expect timeout (no event received) ---
    ($receiver:expr, timeout) => {{
        let result = timeout(TIMEOUT, $receiver.recv()).await;
        assert!(result.is_err(), "expected timeout, but got: {:?}", result);
    }};

    // --- Expect a specific event ---
    ($receiver:expr, $expected:expr) => {{
        let result = timeout(TIMEOUT, $receiver.recv()).await;
        assert_eq!(result, Ok(Some($expected)));
    }};
}
