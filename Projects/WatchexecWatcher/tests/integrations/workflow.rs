use super::*;

// File not exists
#[tokio::test]
async fn detects_nonexistent_path_error() {
    let temp = Temp::new();

    let result = Watcher::build(temp.config.clone()).unwrap_err();
    assert!(matches!(
        result,
        Error::ConfigurationNotExists { ref path } if path == &temp.config
    ));
}

// ----- `config` and `file` both -----

#[tokio::test]
async fn modify_remove_cycle() {
    let temp = Temp::new();
    temp.action(&[Action::Write(&[FileType::Config, FileType::File])])
        .await;

    // Build `Watcher`
    let (mut event_receiver, include_sender) = setup_watcher(&temp.config).await;
    include_sender
        .send(vec![temp.file_string()])
        .await
        .unwrap()
        .unwrap();

    // Modify in the same time
    temp.action(&[Action::Write(&[FileType::Config, FileType::File])])
        .await;

    assert_event!(event_receiver, Event::ConfigFileModify);

    // Delete
    temp.action(&[Action::Remove(&[FileType::Config, FileType::File])])
        .await;

    assert_event!(event_receiver, Event::ConfigRemove);
    assert!(event_receiver.is_closed());
}

// ------ Only `config` -----

#[tokio::test]
async fn only_config_modify_remove_cycle() {
    let temp = Temp::new();
    temp.action(&[Action::Write(&[FileType::Config, FileType::File])])
        .await;

    // Build `Watcher`
    let (mut event_receiver, include_sender) = setup_watcher(&temp.config).await;
    include_sender
        .send(vec![temp.file_string()])
        .await
        .unwrap()
        .unwrap();

    // Modify `config`
    temp.action(&[Action::Write(&[FileType::Config])]).await;

    assert_event!(event_receiver, Event::ConfigModify);

    // Delete
    temp.action(&[Action::Remove(&[FileType::Config])]).await;

    assert_event!(event_receiver, Event::ConfigRemove);
}

// ------ Only `file` -----

#[tokio::test]
async fn only_files_modify_remove_cycle() {
    let temp = Temp::new();
    temp.action(&[Action::Write(&[FileType::Config, FileType::File])])
        .await;

    // Build `Watcher`
    let (mut event_receiver, include_sender) = setup_watcher(&temp.config).await;
    include_sender
        .send(vec![temp.file_string()])
        .await
        .unwrap()
        .unwrap();

    // Modify in the same time
    temp.action(&[Action::Write(&[FileType::File])]).await;

    assert_event!(event_receiver, Event::FileModify);

    // Delete
    temp.action(&[Action::Remove(&[FileType::File])]).await;

    assert_event!(event_receiver, Event::FileRemove);
}
