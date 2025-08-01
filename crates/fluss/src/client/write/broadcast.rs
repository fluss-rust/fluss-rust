use std::sync::Arc;
use parking_lot::RwLock;
use thiserror::Error;
use tokio::sync::Notify;
use tracing::warn;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub type BatchWriteResult = Result<(), Error>;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("BroadcastOnce dropped")]
    Dropped,
}

#[derive(Debug, Clone)]
pub struct BroadcastOnceReceiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T: Clone + Send + Sync> BroadcastOnceReceiver<T> {
    /// Returns `Some(_)` if data has been produced
    pub fn peek(&self) -> Option<Result<T>> {
        self.shared.data.read().clone()
    }

    /// Waits for [`BroadcastOnce::broadcast`] to be called or returns an error
    /// if the [`BroadcastOnce`] is dropped without a value being published
    pub async fn receive(&self) -> Result<T> {
        let notified = self.shared.notify.notified();

        if let Some(v) = self.peek() {
            return v;
        }

        notified.await;

        self.peek().expect("just got notified")
    }
}

#[derive(Debug)]
struct Shared<T> {
    data: RwLock<Option<Result<T>>>,
    notify: Notify,
}

#[derive(Debug)]
pub struct BroadcastOnce<T>
where
    T: Send + Sync,
{
    shared: Arc<Shared<T>>,
}

impl<T> Default for BroadcastOnce<T>
where
    T: Send + Sync,
{
    fn default() -> Self {
        Self {
            shared: Arc::new(Shared {
                data: Default::default(),
                notify: Default::default(),
            }),
        }
    }
}

impl<T: Clone + Send + Sync> BroadcastOnce<T> {
    /// Returns a [`BroadcastOnceReceiver`] that can be used to wait on
    /// a call to [`BroadcastOnce::broadcast`] on this instance
    pub fn receiver(&self) -> BroadcastOnceReceiver<T> {
        BroadcastOnceReceiver {
            shared: Arc::clone(&self.shared),
        }
    }

    /// Broadcast a value to all [`BroadcastOnceReceiver`] handles
    pub fn broadcast(&self, r: T) {
        let mut locked = self.shared.data.write();
        assert!(locked.is_none(), "double publish");

        *locked = Some(Ok(r));
        self.shared.notify.notify_waiters();
    }
}

impl<T> Drop for BroadcastOnce<T>
where
    T: Send + Sync,
{
    fn drop(&mut self) {
        let mut data = self.shared.data.write();
        if data.is_none() {
            warn!("BroadcastOnce dropped without producing");
            *data = Some(Err(Error::Dropped));
            self.shared.notify.notify_waiters();
        }
    }
}