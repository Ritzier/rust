use std::path::{Path, PathBuf, absolute};
use std::sync::Arc;

use globset::{Glob, GlobSet, GlobSetBuilder};
use tokio::sync::mpsc::{self, Receiver};
use tokio::sync::{RwLock, oneshot};
use tokio::task::JoinHandle;
use watchexec::Watchexec;

use crate::Error;

use super::include_sender::IncludeSender;

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
