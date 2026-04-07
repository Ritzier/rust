use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use tempfile::TempDir;
use tokio::fs;
use tokio::sync::mpsc::Receiver;
use tokio::time::error::Elapsed;
use tokio::time::timeout;
use watchexec_watcher::{Error, FileEvent, IncludeSender, Watcher};

const TIMEOUT: Duration = Duration::from_millis(800);

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
pub async fn detects_nonexistent_path_error() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("config.toml");

    let result = Watcher::build(file.clone()).unwrap_err();
    assert!(matches!(
        result,
        Error::ConfigurationNotExists { ref path } if path == &file
    ));
}

// Detects modifications on existing file
#[tokio::test]
pub async fn detects_file_modification() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("config.toml");

    fs::write(&file, "initial content").await.unwrap();

    let (mut event_receiver, include_sender) = setup_watcher(&file).await;

    // Modify file and get `Modify` event
    fs::write(&file, "modified content").await.unwrap();
    let expected = HashMap::from([(file, FileEvent::Modify)]);
    assert_event!(event_receiver, expected);

    // Include test
    let file2 = temp_dir.path().join("style.css");
    fs::write(&file2, "initial content").await.unwrap();
    include_sender
        .send(vec![file2.to_str().unwrap().to_string()])
        .await
        .unwrap();

    fs::write(&file2, "modified content").await.unwrap();
    let expected = HashMap::from([(file2.clone(), FileEvent::Modify)]);
    assert_event!(event_receiver, expected);
}

// Detecs removal of existing file
#[tokio::test]
pub async fn detects_file_removal() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("config.toml");
    fs::write(&file, "initial content").await.unwrap();

    let Watcher {
        watchexec_task: _,
        mut event_receiver,
        startup_rx,
        include_updater_task: _,
        include_sender,
    } = Watcher::build(file.clone()).unwrap();
    startup_rx.await.unwrap();

    fs::remove_file(&file).await.unwrap();
    let result = timeout(TIMEOUT, event_receiver.recv()).await;
    let expected = HashMap::from([(file, FileEvent::Remove)]);

    assert_eq!(result, Ok(Some(expected)));
}

// Full cycle: modify → remove → recreate (ignores final create)
#[tokio::test]
pub async fn handles_modify_remove_recreate_cycle() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("config.toml");
    fs::write(&file, "initial content").await.unwrap();

    let Watcher {
        watchexec_task: _,
        mut event_receiver,
        startup_rx,
        include_updater_task: _,
        include_sender,
    } = Watcher::build(file.clone()).unwrap();
    startup_rx.await.unwrap();

    // Modify
    fs::write(&file, "modified content").await.unwrap();
    let result = timeout(TIMEOUT, event_receiver.recv()).await;
    let expected = HashMap::from([(file.clone(), FileEvent::Modify)]);
    assert_eq!(result, Ok(Some(expected)));

    // Remove
    fs::remove_file(&file).await.unwrap();
    let result = timeout(TIMEOUT, event_receiver.recv()).await;
    let expected = HashMap::from([(file.clone(), FileEvent::Remove)]);
    assert_eq!(result, Ok(Some(expected)));

    // Recreate and expected timetout
    fs::write(&file, "restore content").await.unwrap();
    let result = timeout(TIMEOUT, event_receiver.recv()).await;
    assert!(matches!(result, Err(Elapsed { .. })))
}

#[tokio::test]
async fn a() {
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.toml");
    fs::write(&config_file, "initial content").await.unwrap();

    let include_file = temp_dir.path().join("style.css");
    fs::write(&include_file, "initial content").await.unwrap();

    let Watcher {
        watchexec_task: _,
        mut event_receiver,
        startup_rx,
        include_updater_task: _,
        include_sender,
    } = Watcher::build(config_file.clone()).unwrap();
    startup_rx.await.unwrap();

    // include_sender.send(vec![include_file.to_string_lossy().to_string()]);

    // Modify
    fs::write(&config_file, "modified content").await.unwrap();
    let result = timeout(TIMEOUT, event_receiver.recv()).await;
    let expected = HashMap::from([(config_file.clone(), FileEvent::Modify)]);
    assert_eq!(result, Ok(Some(expected)));

    // Remove
    fs::remove_file(&config_file).await.unwrap();
    let result = timeout(TIMEOUT, event_receiver.recv()).await;
    let expected = HashMap::from([(config_file.clone(), FileEvent::Remove)]);
    assert_eq!(result, Ok(Some(expected)));

    // Recreate and expected timetout
    fs::write(&config_file, "restore content").await.unwrap();
    let result = timeout(TIMEOUT, event_receiver.recv()).await;
    assert!(matches!(result, Err(Elapsed { .. })))
}
