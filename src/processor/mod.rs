use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::types::CommitBlockInfoV1;

pub mod json;
pub mod tree;

#[async_trait]
pub trait Processor {
    async fn run(self, rx: mpsc::Receiver<CommitBlockInfoV1>);
}
