use std::sync::Arc;
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use tokio::sync::oneshot::Sender;


pub mod auth_service;
pub mod database;
pub mod esi;

lazy_static! {
    /// Channel used to send shutdown signal - wrapped in an Option to allow
    /// it to be taken by value (since oneshot channels consume themselves on
    /// send) and an Arc<Mutex> to allow it to be safely shared between threads
    pub static ref SHARED_TX: Arc<Mutex<Option<Sender<(String,String)>>>> = <_>::default();
}

