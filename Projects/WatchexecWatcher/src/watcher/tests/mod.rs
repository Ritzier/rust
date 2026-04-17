use std::collections::HashMap;
use std::path::{PathBuf, absolute};
use std::sync::Arc;

use globset::{Glob, GlobSetBuilder};
use rstest::rstest;
use tempfile::TempDir;
use tokio::fs;
use tokio::sync::RwLock;
use watchexec_events::filekind::{AccessKind, CreateKind, FileEventKind, ModifyKind, RemoveKind};
use watchexec_events::{Event as WatchexecEvent, Tag};

use crate::{Error, Event, Watcher};

use super::watcher_core::{FileEvent, FileType};

mod build;
mod handle_event;
mod merge_events;

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
