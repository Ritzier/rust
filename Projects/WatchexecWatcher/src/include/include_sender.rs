use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use super::errors::IncludeError;

type OneshotResult = Result<(), IncludeError>;

#[derive(Debug)]
pub struct IncludeSender {
    pub include_sender: Sender<(Vec<String>, oneshot::Sender<OneshotResult>)>,
}

impl IncludeSender {
    pub async fn send(
        &self,
        include: Vec<String>,
    ) -> Result<Result<(), IncludeError>, oneshot::error::RecvError> {
        let (tx, rx) = oneshot::channel();

        let _ = self.include_sender.send((include, tx)).await;

        rx.await
    }
}
