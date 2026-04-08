use std::path::{Path, PathBuf, absolute};
use std::sync::Arc;

use globset::{Glob, GlobSet, GlobSetBuilder};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::{RwLock, oneshot};
use tokio::task::JoinHandle;
use watchexec::Watchexec;

use crate::Error;

pub struct IncludeUpdater {
    include_receiver: Receiver<(Vec<String>, oneshot::Sender<()>)>,
    arc_wx: Arc<Watchexec>,
    arc_globset: Arc<RwLock<GlobSet>>,
    configuration_path: PathBuf,
}

pub struct IncludeUpdaterInit {
    pub include_updater_task: JoinHandle<Result<(), Error>>,
    pub include_sender: IncludeSender,
}

impl IncludeUpdater {
    pub fn build(
        arc_wx: Arc<Watchexec>,
        arc_globset: Arc<RwLock<GlobSet>>,
        configuration_path: PathBuf,
    ) -> IncludeUpdaterInit {
        let (include_sender, include_receiver) = mpsc::channel(32);

        let include_updater = IncludeUpdater {
            include_receiver,
            arc_wx,
            arc_globset,
            configuration_path,
        };
        let include_updater_task = tokio::spawn(async move { include_updater.watch().await });

        // `IncludeSender`
        let include_sender = IncludeSender { include_sender };

        IncludeUpdaterInit {
            include_updater_task,
            include_sender,
        }
    }

    pub async fn watch(self) -> Result<(), Error> {
        let Self {
            mut include_receiver,
            arc_wx,
            arc_globset,
            configuration_path,
        } = self;

        while let Some((include, oneshot_sender)) = include_receiver.recv().await {
            let mut builder = GlobSetBuilder::new();
            let mut paths = Vec::new();

            for path in include {
                match Self::process_include(&path) {
                    Ok((absolute_path, glob_pattern)) => {
                        paths.push(absolute_path);
                        builder.add(glob_pattern);
                    }
                    Err(e) => {
                        eprintln!("{e}")
                    }
                }
            }

            if paths.is_empty() {
                continue;
            }

            let glob_set = match builder.build() {
                Ok(glob_set) => glob_set,
                Err(e) => {
                    eprintln!("{e}");
                    continue;
                }
            };

            paths.push(configuration_path.clone());
            arc_wx.config.pathset(paths);
            *arc_globset.write().await = glob_set;

            let _ = oneshot_sender.send(());
        }

        Ok(())
    }

    // 1. `style.css` -> `/project/path/style.css` && `Glob::new("/project/path/style.css")`
    // 2. `app/` -> `/project/path/app/` && `Glob::new("/project/path/app/")`
    // 3. `*.rs` -> `/project/path/` && `Glob::new("/project/path/*.rs")`
    // 4. `/other/project/**/*.rs` -> `/other/project/`
    pub fn process_include(pattern: &str) -> Result<(PathBuf, Glob), Error> {
        let has_wildcard = pattern.contains('*') || pattern.contains('?');
        let path = Path::new(pattern);

        // 1. Determine base directory
        let base = match has_wildcard {
            true => path
                .components()
                .take_while(|c| {
                    let s = c.as_os_str().to_string_lossy();
                    !s.contains('*') && !s.contains('?')
                })
                .collect::<PathBuf>(),
            false => path.to_path_buf(),
        };

        let base = match base.as_os_str().is_empty() {
            true => PathBuf::from("."),
            false => base,
        };

        // 2. Make absolute
        let absolute_base = match base.is_absolute() {
            true => base,
            false => absolute(&base).map_err(Error::Absolute)?,
        };

        if !absolute_base.exists() {
            return Err(Error::PathNotExists {
                pathbuf: absolute_base,
            });
        }

        // 3. Build glob pattern
        let glob_pattern = match has_wildcard {
            true => match path.is_absolute() {
                true => pattern.to_string(),
                false => {
                    let base_str =
                        absolute_base
                            .to_str()
                            .ok_or_else(|| Error::PathIsNotValidUTF8 {
                                pathbuf: absolute_base.clone(),
                            })?;
                    format!("{}/{}", base_str, pattern)
                }
            },
            false => absolute_base
                .to_str()
                .ok_or_else(|| Error::PathIsNotValidUTF8 {
                    pathbuf: absolute_base.clone(),
                })?
                .to_string(),
        };

        let glob = Glob::new(&glob_pattern)?;

        Ok((absolute_base, glob))
    }
}

#[derive(Debug)]
pub struct IncludeSender {
    include_sender: Sender<(Vec<String>, oneshot::Sender<()>)>,
}

impl IncludeSender {
    pub async fn send(&self, include: Vec<String>) -> Result<(), oneshot::error::RecvError> {
        let (tx, rx) = oneshot::channel();

        let _ = self.include_sender.send((include, tx)).await;

        rx.await
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use tempfile::TempDir;
    use tokio::fs;

    use super::*;

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
}
