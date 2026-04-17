use watchexec_watcher::include::IncludeError;

use super::*;

// `Watcher::build` config path not exists
#[tokio::test]
async fn detects_nonexistent_path_error() {
    let temp = Temp::new();

    let result = Watcher::build(temp.config.clone()).unwrap_err();
    assert!(matches!(
        result,
        Error::ConfigurationNotExists { ref path } if path == &temp.config
    ));
}

// `IncludeSender` get empty path
#[tokio::test]
async fn returns_error_when_path_does_not_exist() {
    let temp = Temp::new();
    temp.action(&[Action::Write(&[FileType::Config])]).await;

    let (_event_receiver, include_sender) = setup_watcher(&temp.config).await;
    let result = include_sender.send(vec![temp.file_string()]).await.unwrap();

    assert!(matches!(
        result,
        Err(IncludeError::PathNotExists { pathbuf }) if pathbuf == temp.file
    ))
}
