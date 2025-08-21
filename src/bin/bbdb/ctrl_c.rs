use std::sync::Arc;

use ctrlc;
use futures::channel::oneshot;
use parking_lot::Mutex;

// TODO Replace this with tokio signal

pub struct CtrlCHandler {
    ch: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl CtrlCHandler {
    pub fn new() -> Self {
        Self {
            ch: Arc::new(Mutex::new(None)),
        }
    }

    pub fn register(&self) -> Result<(), ctrlc::Error> {
        let ch_cloned = self.ch.clone();

        ctrlc::set_handler(move || {
            let owned_ch = ch_cloned.lock().take();
            if let Some(ch) = owned_ch {
                if let Err(why) = ch.send(()) {
                    eprintln!("channel error: {:?}", why);
                }
            }
        })
    }

    pub fn notify(&mut self) -> oneshot::Receiver<()> {
        let (snd, recv) = oneshot::channel::<()>();
        let mut ch = self.ch.lock();
        *ch = Some(snd);
        recv
    }
}
