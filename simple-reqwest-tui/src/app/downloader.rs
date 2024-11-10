use super::{App, DownloadItem, DownloadStatus, Event, Result};
use reqwest::StatusCode;

impl App {
    pub async fn get_url(&mut self, url: String) -> Result<()> {
        let client = self.client.clone();
        let event_tx = self.event_tx.clone();

        // Insert into HashMap instead of pushing to Vec
        self.download_hashmap.insert(
            url.clone(),
            DownloadItem {
                status: DownloadStatus::Loading,
                loading_frame: 0,
            },
        );

        tokio::spawn(async move {
            let status = match client.get(&url).send().await {
                Ok(response) => match response.status() {
                    StatusCode::OK => DownloadStatus::Complete(format!("Success: {}", url)),
                    status => DownloadStatus::Error(format!("Failed ({}): {}", status, url)),
                },
                Err(e) => DownloadStatus::Error(format!("Request error: {} - {}", url, e)),
            };

            let download_item = DownloadItem {
                status,
                loading_frame: 0,
            };

            // Send the event with the URL as the key and the updated DownloadItem
            if let Err(e) = event_tx.send(Event::DownloadStatus(url, download_item)) {
                eprintln!("Failed to send download status: {}", e);
            }
        });

        Ok(())
    }
}
