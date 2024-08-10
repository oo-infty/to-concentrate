use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::{self, Sender as OneshotSender};
use tokio::task::JoinHandle;
use tokio::time::Duration;

use crate::domain::entity::StageState;

/// Result of one query of the current state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryResponse {
    pub total: Duration,
    pub past: Duration,
    pub stage: StageState,
}

/// Actions that a [`WorkerRoutine`] runs.
#[derive(Debug)]
pub enum Command {
    Pause,
    Resume,
    Skip,
    Query {
        responder: OneshotSender<QueryResponse>,
    },
    Stop,
}

/// Handle that controls a [`WorkerRoutine`].
#[derive(Debug)]
pub struct WorkerHandle {
    requester: Sender<Command>,
    handle: JoinHandle<()>,
}

impl WorkerHandle {
    /// Creates a new [`WorkerHandle`].
    pub fn new(requester: Sender<Command>, handle: JoinHandle<()>) -> Self {
        Self { requester, handle }
    }

    /// Send [`Command::Pause`] to the background worker and pause the timer.
    pub async fn pause(&self) {
        match self.requester.send(Command::Pause).await {
            Ok(_) => {}
            Err(_) => unreachable!("Worker should not be shutted down"),
        };
    }

    /// Send [`Command::Resume`] to the background worker and resume the timer.
    pub async fn resume(&self) {
        match self.requester.send(Command::Resume).await {
            Ok(_) => {}
            Err(_) => unreachable!("Worker should not be shutted down"),
        };
    }

    /// Send [`Command::Skip`] to the background worker and skip to the next
    /// stage.
    pub async fn skip(&self) {
        match self.requester.send(Command::Skip).await {
            Ok(_) => {}
            Err(_) => unreachable!("Worker should not be shutted down"),
        };
    }

    /// Send [`Command::Query`] to the background worker to get the current
    /// state.
    pub async fn query(&self) -> QueryResponse {
        let (responder, receiver) = oneshot::channel();
        match self.requester.send(Command::Query { responder }).await {
            Ok(_) => match receiver.await {
                Ok(res) => res,
                Err(_) => unreachable!("Worker should not be shutted down"),
            },
            Err(_) => unreachable!("Worker should not be shutted down"),
        }
    }

    /// Stop the background worker and drop the handle.
    pub async fn stop(self) {
        match self.requester.send(Command::Stop).await {
            Ok(_) => {}
            Err(_) => unreachable!("Worker should not be shutted down"),
        };
        match self.handle.await {
            Ok(_) => tracing::info!("`WorkerRoutine` was shutted down"),
            Err(err) => tracing::error!(%err, "`WorkerRoutine` failed to join"),
        };
    }
}
