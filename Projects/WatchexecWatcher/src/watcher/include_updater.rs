use std::path::{PathBuf, absolute};
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
    arc_globset: Arc<RwLock<Option<GlobSet>>>,
    configuration_path: PathBuf,
}

pub struct IncludeUpdaterInit {
    pub include_updater_task: JoinHandle<Result<(), Error>>,
    pub include_sender: IncludeSender,
}

impl IncludeUpdater {
    pub fn build(
        arc_wx: Arc<Watchexec>,
        arc_globset: Arc<RwLock<Option<GlobSet>>>,
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
                    Ok(Some((absolute_path, glob_pattern))) => {
                        paths.push(absolute_path);
                        builder.add(glob_pattern);
                    }
                    Ok(None) => {
                        eprintln!("{path} fail")
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
            *arc_globset.write().await = Some(glob_set);

            let _ = oneshot_sender.send(());
        }

        Ok(())
    }

    pub fn process_include(pattern: &String) -> Result<Option<(PathBuf, Glob)>, Error> {
        let has_wildcard = pattern.contains('*') || pattern.contains('?');

        match has_wildcard {
            true => {
                //
                Ok(None)
            }

            false => {
                let pathbuf = PathBuf::from(pattern);
                let absolute_path = absolute(&pathbuf).map_err(Error::Absolute)?;
                let a = absolute_path.to_str().ok_or(Error::PathIsNotValidUTF8 {
                    pathbuf: absolute_path.clone(),
                })?;
                let glob_set = Glob::new(a)?;

                match pathbuf.exists() {
                    true => Ok(Some((absolute_path, glob_set))),
                    false => Ok(None),
                }
            }
        }
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
