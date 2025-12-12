use ipc_channel::ipc::IpcSender;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub response_sender: IpcSender<Response>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub message: String,
}
