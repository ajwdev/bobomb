use std::sync::{Arc,Mutex,Condvar};

pub type ExecutorLock = Arc<(Mutex<bool>, Condvar)>;
