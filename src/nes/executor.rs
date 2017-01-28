use parking_lot::{Mutex,Condvar};
use std::sync::Arc;

pub type ExecutorLock = Arc<(Mutex<bool>, Condvar)>;
