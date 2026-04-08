use std::path::{PathBuf, absolute};

use globset::Glob;
use rstest::rstest;
use tempfile::TempDir;
use tokio::fs;

use crate::Error;
use crate::include::include_updater::IncludeUpdater;

// ----- Error: path does not exists -----

#[test]
fn not_exist_relative_file() {
    let config = "config.toml";
    let abs = absolute(config).unwrap();
    let result = IncludeUpdater::process_include(config);
    assert!(
        matches!(result, Err(Error::PathNotExists { ref pathbuf }) if pathbuf == &abs),
        "expected PathNotExists for {abs:?}, got {result:#?}"
    );
}

/// Absolute path to a file that does not exist.
#[tokio::test]
async fn not_exist_absolute_file() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("ghost.toml"); // never created
    let result = IncludeUpdater::process_include(path.to_str().unwrap());
    assert!(
        matches!(result, Err(Error::PathNotExists { ref pathbuf }) if pathbuf == &path),
        "expected PathNotExists for {path:?}, got {result:#?}"
    );
}

// ----- Relative glob patterns -----

/// `**.rs` — wildcard at the start; base collapses to `.`
#[test]
fn glob_double_star_no_prefix() {
    let pattern = "**.rs";
    let (pathbuf, glob) = IncludeUpdater::process_include(pattern).unwrap();

    let abs_cwd = absolute(PathBuf::from(".")).unwrap();
    let expected_pattern = format!("{}/{pattern}", abs_cwd.to_str().unwrap());

    assert_eq!(pathbuf, abs_cwd);
    assert_eq!(glob, Glob::new(&expected_pattern).unwrap());
}

/// `*/**.rs` — single non-wildcard component at the front also collapses to `.`
#[test]
fn glob_star_prefix() {
    let pattern = "*/**.rs";
    let (pathbuf, glob) = IncludeUpdater::process_include(pattern).unwrap();

    let abs_cwd = absolute(PathBuf::from(".")).unwrap();
    let expected_pattern = format!("{}/{pattern}", abs_cwd.to_str().unwrap());

    assert_eq!(pathbuf, abs_cwd);
    assert_eq!(glob, Glob::new(&expected_pattern).unwrap());
}

/// `?oo/*.rs` — `?` wildcard in first component; base collapses to `.`
#[test]
fn glob_question_mark_in_first_component() {
    let pattern = "?oo/*.rs";
    let (pathbuf, glob) = IncludeUpdater::process_include(pattern).unwrap();

    let abs_cwd = absolute(PathBuf::from(".")).unwrap();
    let expected_pattern = format!("{}/{pattern}", abs_cwd.to_str().unwrap());

    assert_eq!(pathbuf, abs_cwd);
    assert_eq!(glob, Glob::new(&expected_pattern).unwrap());
}

// ----- Glob: non-existent base directory -----

#[rstest]
#[case("should/*.rs", "should")]
#[case("should/not/exists/*.rs", "should/not/exists")]
fn glob_nonexistent_base(#[case] pattern: &str, #[case] missing_dir: &str) {
    let result = IncludeUpdater::process_include(pattern);
    let expected = absolute(PathBuf::from(missing_dir)).unwrap();
    assert!(
        matches!(result, Err(Error::PathNotExists { ref pathbuf }) if pathbuf == &expected),
        "pattern={pattern:?}: expected PathNotExists({expected:?}), got {result:#?}"
    );
}

// ----- Absolute paths -----

/// Absolute path to an existing *file* — no wildcard branch.
#[tokio::test]
async fn absolute_existing_file() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("style.css");
    fs::write(&path, "body {}").await.unwrap();

    let (pathbuf, glob) = IncludeUpdater::process_include(path.to_str().unwrap()).unwrap();

    assert_eq!(pathbuf, path);
    assert_eq!(glob, Glob::new(path.to_str().unwrap()).unwrap());
}

/// Absolute path to an existing *nested* file.
#[tokio::test]
async fn absolute_existing_nested_file() {
    let temp = TempDir::new().unwrap();
    let path = temp
        .path()
        .join("app")
        .join("src")
        .join("pages")
        .join("home.rs");
    fs::create_dir_all(path.parent().unwrap()).await.unwrap();
    fs::write(&path, "fn main() {}").await.unwrap();

    let (pathbuf, glob) = IncludeUpdater::process_include(path.to_str().unwrap()).unwrap();

    assert_eq!(pathbuf, path);
    assert_eq!(glob, Glob::new(path.to_str().unwrap()).unwrap());
}

/// Absolute path to an existing *directory*.
#[tokio::test]
async fn absolute_existing_directory() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("app").join("src").join("pages");
    fs::create_dir_all(&path).await.unwrap();

    let (pathbuf, glob) = IncludeUpdater::process_include(path.to_str().unwrap()).unwrap();

    assert_eq!(pathbuf, path);
    assert_eq!(glob, Glob::new(path.to_str().unwrap()).unwrap());
}

// ----- Absolute glob patterns -----

/// Absolute base + wildcard suffix: base is extracted correctly.
#[tokio::test]
async fn absolute_glob_existing_base() {
    let temp = TempDir::new().unwrap();
    let folder = temp.path().join("src").join("pages");
    fs::create_dir_all(&folder).await.unwrap();

    let pattern = format!("{}/**/*.rs", folder.to_str().unwrap());
    let (pathbuf, glob) = IncludeUpdater::process_include(&pattern).unwrap();

    assert_eq!(pathbuf, folder);
    assert_eq!(glob, Glob::new(&pattern).unwrap());
}

/// Absolute base + `?` wildcard.
#[tokio::test]
async fn absolute_glob_question_mark() {
    let temp = TempDir::new().unwrap();
    let folder = temp.path().join("src");
    fs::create_dir_all(&folder).await.unwrap();

    let pattern = format!("{}/?.rs", folder.to_str().unwrap());
    let (pathbuf, glob) = IncludeUpdater::process_include(&pattern).unwrap();

    assert_eq!(pathbuf, folder);
    assert_eq!(glob, Glob::new(&pattern).unwrap());
}

/// Absolute glob whose base directory does not exist.
#[tokio::test]
async fn absolute_glob_nonexistent_base() {
    let temp = TempDir::new().unwrap();
    let missing = temp.path().join("does_not_exist");
    // deliberately NOT creating `missing`
    let pattern = format!("{}/*.rs", missing.to_str().unwrap());

    let result = IncludeUpdater::process_include(&pattern);
    assert!(
        matches!(result, Err(Error::PathNotExists { ref pathbuf }) if pathbuf == &missing),
        "expected PathNotExists({missing:?}), got {result:#?}"
    );
}
