use parking_lot::Mutex;
use std::sync::mpsc::{self, Receiver, SyncSender};

pub struct DebugState {
    pub paused: Mutex<bool>,
    pub unpause_waiter: Mutex<Receiver<()>>,
    pub unpause_breaker: Mutex<SyncSender<()>>,
}

impl Default for DebugState {
    fn default() -> Self {
        let (send, recv) = mpsc::sync_channel(1);
        Self {
            paused: false.into(),
            unpause_waiter: recv.into(),
            unpause_breaker: send.into(),
        }
    }
}
