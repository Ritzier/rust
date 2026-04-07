use std::collections::HashMap;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::time::Duration;

use tempfile::TempDir;
use tokio::fs;
use tokio::sync::mpsc::Receiver;
use tokio::time::timeout;
use watchexec_watcher::{Error, FileEvent, IncludeSender, Watcher};

const TIMEOUT: Duration = Duration::from_millis(800);

struct Temp {
    temp_dir: TempDir,
}

impl Temp {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();

        Self { temp_dir }
    }

    pub fn create_file(&self, file_name: &str) -> TempFile {
        let path = self.temp_dir.path().join(file_name);

        TempFile { path }
    }
}

struct TempFile {
    path: PathBuf,
}

impl TempFile {
    async fn write(&self, content: &str) {
        fs::write(&self.path, content).await.unwrap();
    }

    async fn delete(&self) {
        fs::remove_file(&self.path).await.unwrap();
    }
}

impl Display for TempFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.path.to_string_lossy().into_owned();
        write!(f, "{path}")
    }
}

async fn setup_watcher(file: &Path) -> (Receiver<HashMap<PathBuf, FileEvent>>, IncludeSender) {
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

macro_rules! assert_event {
    ($receiver:expr, $expected:expr) => {{
        let result = timeout(TIMEOUT, $receiver.recv()).await;
        assert_eq!(result, Ok(Some($expected)));
    }};
}

// File not exists
#[tokio::test]
async fn detects_nonexistent_path_error() {
    let temp = Temp::new();
    let file = temp.create_file("config.toml");

    let result = Watcher::build(file.path.clone()).unwrap_err();
    assert!(matches!(
        result,
        Error::ConfigurationNotExists { ref path } if path == &file.path
    ));
}

#[tokio::test]
async fn modify_remove_cycle() {
    let temp = Temp::new();
    let config_file = temp.create_file("config.toml");
    config_file.write("initial content").await;
    let include_file1 = temp.create_file("index.html");
    include_file1.write("initial content").await;
    let include_file2 = temp.create_file("style.css");
    include_file2.write("initial content").await;

    // Build `Watcher`
    let (mut event_receiver, include_sender) = setup_watcher(&config_file.path).await;
    include_sender
        .send(vec![include_file1.to_string(), include_file2.to_string()])
        .await
        .unwrap();

    // Modify in the same time
    let task1 = async { config_file.write("modify content").await };
    let task2 = async { include_file1.write("modify content").await };
    let task3 = async { include_file2.write("modify content").await };
    tokio::join!(task1, task2, task3);

    let expected = HashMap::from([
        (config_file.path.clone(), FileEvent::Modify),
        (include_file1.path.clone(), FileEvent::Modify),
        (include_file2.path.clone(), FileEvent::Modify),
    ]);
    assert_event!(event_receiver, expected);

    // Delete
    let task1 = async { config_file.delete().await };
    let task2 = async { include_file1.delete().await };
    let task3 = async { include_file2.delete().await };
    tokio::join!(task1, task2, task3);

    let expected = HashMap::from([
        (config_file.path.clone(), FileEvent::Remove),
        (include_file1.path.clone(), FileEvent::Remove),
        (include_file2.path.clone(), FileEvent::Remove),
    ]);
    assert_event!(event_receiver, expected);
}
