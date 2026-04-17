use super::*;

// ------ `handle_event()`: returns None for empty / irrelevant events -----

#[tokio::test]
async fn handle_event_empty_slice_returns_none() {
    let config = PathBuf::from("/fake/config.toml");
    let result = Watcher::handle_event(&arc_events(vec![]), &config, &empty_globset()).await;
    assert!(result.is_none());
}

#[rstest]
#[case(FileEventKind::Any)]
#[case(FileEventKind::Other)]
#[case(FileEventKind::Access(AccessKind::Any))]
#[tokio::test]
async fn handle_event_ignores_kinds(#[case] kind: FileEventKind) {
    let config = PathBuf::from("/fake/config.toml");

    let event = make_event(config.clone(), kind);
    let result = Watcher::handle_event(&arc_events(vec![event]), &config, &empty_globset()).await;

    assert!(result.is_none());
}

/// A path that is neither the config nor in the globset must be ignored.
#[tokio::test]
async fn handle_event_ignores_unrelated_path() {
    let config = PathBuf::from("/fake/config.toml");
    let unrelated = PathBuf::from("/other/file.rs");
    let event = make_event(unrelated, FileEventKind::Modify(ModifyKind::Any));
    let result = Watcher::handle_event(&arc_events(vec![event]), &config, &empty_globset()).await;
    assert!(result.is_none());
}

// ----- handle_event: `configuration` path detection -----

#[rstest]
#[case(FileEventKind::Any, None)]
#[case(FileEventKind::Access(AccessKind::Any), None)]
#[case(FileEventKind::Create(CreateKind::Any), Some(FileEvent::Create))]
#[case(FileEventKind::Modify(ModifyKind::Any), Some(FileEvent::Modify))]
#[case(FileEventKind::Remove(RemoveKind::Any), Some(FileEvent::Remove))]
#[case(FileEventKind::Other, None)]
#[tokio::test]
async fn handle_event_config_kinds(
    #[case] kind: FileEventKind,
    #[case] expected: Option<FileEvent>,
) {
    let config = PathBuf::from("/fake/config.toml");
    let event = make_event(config.clone(), kind);
    let result = Watcher::handle_event(&arc_events(vec![event]), &config, &empty_globset()).await;

    let expected_hashmap = expected
        .map(|file_event| HashMap::<FileType, FileEvent>::from([(FileType::Config, file_event)]));

    assert_eq!(result, expected_hashmap);
}

// ----- `handle_event()`: `globset` path detection -----

#[rstest]
#[case(FileEventKind::Any, None)]
#[case(FileEventKind::Access(AccessKind::Any), None)]
#[case(FileEventKind::Create(CreateKind::Any), Some(FileEvent::Create))]
#[case(FileEventKind::Modify(ModifyKind::Any), Some(FileEvent::Modify))]
#[case(FileEventKind::Remove(RemoveKind::Any), Some(FileEvent::Remove))]
#[case(FileEventKind::Other, None)]
#[tokio::test]
async fn handle_event_file_kinds(#[case] kind: FileEventKind, #[case] expected: Option<FileEvent>) {
    let config = PathBuf::from("/fake/config.toml");
    let watched = PathBuf::from("/src/main.rs");
    let globset = globset_for("/src/*.rs");

    let event = make_event(watched, kind);
    let result = Watcher::handle_event(&arc_events(vec![event]), &config, &globset).await;

    let expected_hashmap = expected
        .map(|file_event| HashMap::<FileType, FileEvent>::from([(FileType::File, file_event)]));

    assert_eq!(result, expected_hashmap);
}

// ----- `handle_event()` both `configuration` and `file` path detection -----

fn map_kind(kind: &FileEventKind) -> Option<FileEvent> {
    match kind {
        FileEventKind::Create(_) => Some(FileEvent::Create),
        FileEventKind::Modify(_) => Some(FileEvent::Modify),
        FileEventKind::Remove(_) => Some(FileEvent::Remove),
        _ => None,
    }
}

#[tokio::test]
async fn handle_event_all_combinations() {
    let kinds = [
        FileEventKind::Any,
        FileEventKind::Access(AccessKind::Any),
        FileEventKind::Create(CreateKind::Any),
        FileEventKind::Modify(ModifyKind::Any),
        FileEventKind::Remove(RemoveKind::Any),
        FileEventKind::Other,
    ];

    let config = PathBuf::from("/fake/config.toml");
    let watched = PathBuf::from("/src/main.rs");
    let globset = globset_for("/src/*.rs");

    for config_kind in &kinds {
        for file_kind in &kinds {
            let events = vec![
                make_event(config.clone(), *config_kind),
                make_event(watched.clone(), *file_kind),
            ];

            let result = Watcher::handle_event(&arc_events(events), &config, &globset).await;

            let mut expected = HashMap::new();

            if let Some(ev) = map_kind(config_kind) {
                expected.insert(FileType::Config, ev);
            }

            if let Some(ev) = map_kind(file_kind) {
                expected.insert(FileType::File, ev);
            }

            let expected = (!expected.is_empty()).then_some(expected);

            assert_eq!(
                result, expected,
                "failed for config_kind={:?}, file_kind={:?}",
                config_kind, file_kind
            );
        }
    }
}

// ----- `handle_event()` tags without a matching path are skipped -----

/// An event that only has a FileEventKind tag (no path tag) must not panic and
/// must produce no output
#[tokio::test]
async fn handle_event_kind_tag_without_path_tag_is_ignored() {
    let config = PathBuf::from("/fake/config.toml");
    let event = WatchexecEvent {
        tags: vec![Tag::FileEventKind(FileEventKind::Modify(ModifyKind::Any))],
        metadata: Default::default(),
    };
    let result = Watcher::handle_event(&arc_events(vec![event]), &config, &empty_globset()).await;
    assert!(result.is_none());
}

/// An event that only has a path tag (no event-kind tag) must produce no output.
#[tokio::test]
async fn handle_event_path_tag_without_kind_tag_is_ignored() {
    let config = PathBuf::from("/fake/config.toml");
    let event = WatchexecEvent {
        tags: vec![Tag::Path {
            path: config.clone(),
            file_type: None,
        }],
        metadata: Default::default(),
    };
    let result = Watcher::handle_event(&arc_events(vec![event]), &config, &empty_globset()).await;
    assert!(result.is_none());
}

// ----- `handle_event()`: many events, only the relevant ones are collected -----

#[tokio::test]
async fn handle_event_mixed_relevant_and_irrelevant_events() {
    let config = PathBuf::from("/fake/config.toml");
    let watched = PathBuf::from("/src/lib.rs");
    let unrelated = PathBuf::from("/tmp/junk.log");
    let globset = globset_for("/src/*.rs");

    let events = vec![
        make_event(unrelated.clone(), FileEventKind::Modify(ModifyKind::Any)),
        make_event(config.clone(), FileEventKind::Modify(ModifyKind::Any)),
        make_event(watched.clone(), FileEventKind::Create(CreateKind::File)),
        make_event(unrelated, FileEventKind::Remove(RemoveKind::File)),
    ];

    let result = Watcher::handle_event(&arc_events(events), &config, &globset).await;

    let map = result.expect("expected Some");
    assert_eq!(map.len(), 2, "only Config and File entries expected");
    assert_eq!(map.get(&FileType::Config), Some(&FileEvent::Modify));
    assert_eq!(map.get(&FileType::File), Some(&FileEvent::Create));
}
