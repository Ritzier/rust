use std::path::{PathBuf, absolute};
use std::sync::Arc;

use globset::{Glob, GlobSetBuilder};
use rstest::rstest;
use tempfile::TempDir;
use tokio::fs;
use tokio::sync::RwLock;
use watchexec_events::filekind::{AccessKind, CreateKind, FileEventKind, ModifyKind, RemoveKind};
use watchexec_events::{Event as WatchexecEvent, Tag};

use crate::{Error, Event};

use super::Watcher;

// ------ Helper -----

/// Construct a minimal `WatchexecEvent` with a path tag and an event-kind tag.
fn make_event(path: PathBuf, kind: FileEventKind) -> WatchexecEvent {
    WatchexecEvent {
        tags: vec![
            Tag::Path {
                path,
                file_type: None,
            },
            Tag::FileEventKind(kind),
        ],
        metadata: Default::default(),
    }
}

/// Wrap events in the `Arc<[_]>` that `handle_event` expects.
fn arc_events(events: Vec<WatchexecEvent>) -> Arc<[WatchexecEvent]> {
    events.into()
}

/// Build an empty `GlobSet` wrapped in `Arc<RwLock<_>>`.
fn empty_globset() -> Arc<RwLock<globset::GlobSet>> {
    Arc::new(RwLock::new(globset::GlobSet::empty()))
}

/// Build a `GlobSet` that matches a single pattern.
fn globset_for(pattern: &str) -> Arc<RwLock<globset::GlobSet>> {
    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new(pattern).unwrap());
    Arc::new(RwLock::new(builder.build().unwrap()))
}

// ------ Error -----

// `build()` error `Error::ConfigurationNotExists` when file not exist
#[test]
fn build_fails_when_config_does_not_exist() {
    let path = PathBuf::from("definetly_does_not_exist.toml");
    let result = Watcher::build(path.clone());

    let abs = absolute(path).unwrap();
    assert!(matches!(result, Err(Error::ConfigurationNotExists { ref path }) if path == &abs));
}

// `build()` success when the `configuration` file actually exists
#[tokio::test]
async fn build_success_with_existing_config() {
    let dir = TempDir::new().unwrap();
    let config = dir.path().join("config.toml");
    fs::write(&config, "[settings]").await.unwrap();

    let watcher = Watcher::build(config);
    assert!(watcher.is_ok())
}

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
#[case(FileEventKind::Create(CreateKind::Any), Some(Event::ConfigCreate))]
#[case(FileEventKind::Modify(ModifyKind::Any), Some(Event::ConfigModify))]
#[case(FileEventKind::Remove(RemoveKind::Any), Some(Event::ConfigRemove))]
#[case(FileEventKind::Other, None)]
#[tokio::test]
async fn handle_event_config_kinds(#[case] kind: FileEventKind, #[case] expected: Option<Event>) {
    let config = PathBuf::from("/fake/config.toml");
    let event = make_event(config.clone(), kind);
    let result = Watcher::handle_event(&arc_events(vec![event]), &config, &empty_globset()).await;

    assert_eq!(result, expected);
}

// ----- `handle_event()`: `globset` path detection -----

#[rstest]
#[case(FileEventKind::Any, None)]
#[case(FileEventKind::Access(AccessKind::Any), None)]
#[case(FileEventKind::Create(CreateKind::Any), Some(Event::FileCreate))]
#[case(FileEventKind::Modify(ModifyKind::Any), Some(Event::FileModify))]
#[case(FileEventKind::Remove(RemoveKind::Any), Some(Event::FileRemove))]
#[case(FileEventKind::Other, None)]
#[tokio::test]
async fn handle_event_file_kinds(#[case] kind: FileEventKind, #[case] expected: Option<Event>) {
    let config = PathBuf::from("/fake/config.toml");
    let watched = PathBuf::from("/src/main.rs");
    let globset = globset_for("/src/*.rs");

    let event = make_event(watched, kind);
    let result = Watcher::handle_event(&arc_events(vec![event]), &config, &globset).await;

    assert_eq!(result, expected);
}

// ----- `handle_event()` both `configuration` and `file` path detection -----

// fn map_kind(kind: &FileEventKind) -> Option<Event> {
//     match kind {
//         FileEventKind::Create(_) => Some(Event::Create),
//         FileEventKind::Modify(_) => Some(Event::Modify),
//         FileEventKind::Remove(_) => Some(Event::Remove),
//         _ => None,
//     }
// }
// #[tokio::test]
// async fn handle_event_all_combinations_a() {
//     let kinds = [
//         FileEventKind::Any,
//         FileEventKind::Access(AccessKind::Any),
//         FileEventKind::Create(CreateKind::Any),
//         FileEventKind::Modify(ModifyKind::Any),
//         FileEventKind::Remove(RemoveKind::Any),
//         FileEventKind::Other,
//     ];
//
//     let config = PathBuf::from("/fake/config.toml");
//     let watched = PathBuf::from("/src/main.rs");
//     let globset = globset_for("/src/*.rs");
//
//     for config_kind in &kinds {
//         for file_kind in &kinds {
//             let events = vec![
//                 make_event(config.clone(), *config_kind),
//                 make_event(watched.clone(), *file_kind),
//             ];
//
//             let result = Watcher::handle_event(&arc_events(events), &config, &globset).await;
//
//             let mut expected = HashMap::new();
//
//             if let Some(ev) = map_kind(config_kind) {
//                 expected.insert(FileType::Config, ev);
//             }
//
//             if let Some(ev) = map_kind(file_kind) {
//                 expected.insert(FileType::File, ev);
//             }
//
//             let expected = (!expected.is_empty()).then_some(expected);
//
//             assert_eq!(
//                 result, expected,
//                 "failed for config_kind={:?}, file_kind={:?}",
//                 config_kind, file_kind
//             );
//         }
//     }
// }

// ----- `handle_event()`: priority merging (higher-priority event wins) -----

// Priority order: Remove(3) > Create(2) > Modify(1) :TODO:
// #[rstest]
// #[case(
//     vec![
//         FileEventKind::Modify(ModifyKind::Any),
//         FileEventKind::Remove(RemoveKind::File),
//     ],
//     Event::Remove,
//     "Remove should supersede Modify"
// )]
// #[case(
//     vec![
//         FileEventKind::Modify(ModifyKind::Any),
//         FileEventKind::Create(CreateKind::Any),
//     ],
//     FileEvent::Create,
//     "Create should supersede Modify"
// )]
// #[case(
//     vec![
//         FileEventKind::Create(CreateKind::Any),
//         FileEventKind::Remove(RemoveKind::Any),
//     ],
//     FileEvent::Remove,
//     "Remove should supersede Create"
// )]
// #[case(
//     vec![
//         FileEventKind::Remove(RemoveKind::Any),
//         FileEventKind::Modify(ModifyKind::Any),
//     ],
//     FileEvent::Remove,
//     "Modify must not downgrade Remove"
// )]
// #[tokio::test]
// async fn handle_event_priority_merging(
//     #[case] kinds: Vec<FileEventKind>,
//     #[case] expected: FileEvent,
//     #[case] msg: &str,
// ) {
//     let config = PathBuf::from("/fake/config.toml");
//
//     let events = kinds
//         .into_iter()
//         .map(|k| make_event(config.clone(), k))
//         .collect();
//
//     let result = Watcher::handle_event(&arc_events(events), &config, &empty_globset()).await;
//
//     let map = result.expect("expected Some");
//
//     // assert_eq!(map.get(&FileType::Config), Some(&expected), "{msg}");
//     assert_eq!(map, expected);
// }

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

    assert_eq!(result, Some(Event::ConfigFileModify));
}
