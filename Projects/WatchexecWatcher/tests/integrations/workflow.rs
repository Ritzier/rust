use super::*;

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

    // --- Modify --
    temp.action(&[Action::Write(&[FileType::Config, FileType::File])])
        .await;

    assert_event!(event_receiver, Event::ConfigFileModify);

    // --- Remove ---
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

    // --- Modify `config` ---
    temp.action(&[Action::Write(&[FileType::Config])]).await;
    assert_event!(event_receiver, Event::ConfigModify);

    // --- Remove `config` ---
    temp.action(&[Action::Remove(&[FileType::Config])]).await;
    assert_event!(event_receiver, Event::ConfigRemove);
    assert!(event_receiver.is_closed());
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

    // --- Modify only file --
    temp.action(&[Action::Write(&[FileType::File])]).await;
    assert_event!(event_receiver, Event::FileModify);

    // --- Remove file ---
    temp.action(&[Action::Remove(&[FileType::File])]).await;
    assert_event!(event_receiver, Event::FileRemove);

    // --- Recreate file (timeout) ---
    temp.action(&[Action::Write(&[FileType::Folder])]).await;
    assert_event!(event_receiver, timeout);
}

// ----- Only `folder` ------

#[tokio::test]
async fn modify_remove_cycle_folder() {
    let temp = Temp::new();

    // Initial state: config + folder exist
    temp.action(&[Action::Write(&[FileType::Config, FileType::Folder])])
        .await;

    let (mut event_receiver, include_sender) = setup_watcher(&temp.config).await;

    // Watch all `.rs` files inside folder recursively
    include_sender
        .send(vec![format!("{}/**/*.rs", temp.folder_string())])
        .await
        .unwrap()
        .unwrap();

    // --- File creation inside folder ---
    temp.action(&[Action::Write(&[FileType::Lib, FileType::Main])])
        .await;
    assert_event!(event_receiver, Event::FileCreate);

    // --- File modification ---
    temp.action(&[Action::Write(&[FileType::Lib])]).await;
    assert_event!(event_receiver, Event::FileModify);

    // --- File removal ---
    temp.action(&[Action::Remove(&[FileType::Lib])]).await;
    assert_event!(event_receiver, Event::FileRemove);

    temp.action(&[Action::Remove(&[FileType::Main])]).await;
    assert_event!(event_receiver, Event::FileRemove);

    // --- Recreate files ---
    temp.action(&[Action::Write(&[FileType::Lib, FileType::Main])])
        .await;
    assert_event!(event_receiver, Event::FileCreate);

    // --- Remove entire folder ---
    temp.action(&[Action::Remove(&[FileType::Folder])]).await;
    assert_event!(event_receiver, Event::FileRemove);

    // --- Recreate folder (timeout) ---
    temp.action(&[Action::Write(&[FileType::Folder])]).await;
    assert_event!(event_receiver, timeout);
}
