use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct IncludeSender {
    pub include_sender: Sender<(Vec<String>, oneshot::Sender<()>)>,
}

impl IncludeSender {
    pub async fn send(&self, include: Vec<String>) -> Result<(), oneshot::error::RecvError> {
        let (tx, rx) = oneshot::channel();

        let _ = self.include_sender.send((include, tx)).await;

        rx.await
    }
}
