use std::collections::HashMap;
use std::time::Duration;

use tempfile::TempDir;
use tokio::fs;
use tokio::time::error::Elapsed;
use tokio::time::timeout;
use use_watchexec::{FileEvent, Watcher};

const TIMEOUT: Duration = Duration::from_millis(800);

// Detects create after startup (times out: path watched but file missing initially)
#[tokio::test]
pub async fn ignores_create_on_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("config.toml");

    let Watcher {
        watchexec_task: _,
        mut event_receiver,
        startup_rx,
    } = Watcher::build([file.clone()]).unwrap();
    startup_rx.await.unwrap();

    fs::write(&file, "initial content").await.unwrap();
    let result = timeout(TIMEOUT, event_receiver.recv()).await;

    assert!(matches!(result, Err(Elapsed { .. })));
}

// Detects modifications on existing file
#[tokio::test]
pub async fn detects_file_modification() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("config.toml");

    fs::write(&file, "initial content").await.unwrap();

    let Watcher {
        watchexec_task,
        mut event_receiver,
        startup_rx,
    } = Watcher::build([file.clone()]).unwrap();
    startup_rx.await.unwrap();

    tokio::spawn(async move { watchexec_task.await.unwrap().unwrap() });

    fs::write(&file, "modified content").await.unwrap();
    let result = timeout(TIMEOUT, event_receiver.recv()).await;
    let expected = HashMap::from([(file, FileEvent::Modify)]);

    assert_eq!(result, Ok(Some(expected)));
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
    } = Watcher::build([file.clone()]).unwrap();
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
    } = Watcher::build([file.clone()]).unwrap();
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
