use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::oneshot::Sender;
use tokio::sync::Mutex;

pub mod auth_service;
pub mod esi;
pub mod objects;

pub type SharedTxMessage<T> = Arc<Mutex<Option<Sender<T>>>>;

lazy_static! {
    /// Channel used to send shutdown signal - wrapped in an Option to allow
    /// it to be taken by value (since oneshot channels consume themselves on
    /// send) and an Arc<Mutex> to allow it to be safely shared between threads
    pub static ref SHARED_TX:SharedTxMessage<(String,String)> = <_>::default();
}
