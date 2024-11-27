use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use task_executor::TaskExecutor;
use tokio::select;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::{mpsc, Semaphore};
use tracing::{error, warn};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub max_workers: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_workers: num_cpus::get(),
        }
    }
}

pub struct Sender {
    name: &'static str,
    tx: mpsc::Sender<WorkItem>,
}

impl Sender {
    fn new(name: &'static str, tx: mpsc::Sender<WorkItem>) -> Self {
        Self { name, tx }
    }

    pub fn send_async(&mut self, future: AsyncFn) {
        self.send_work_item(WorkItem::new_async(self.name, future));
    }

    pub fn send_blocking(&mut self, func: BlockingFn) {
        self.send_work_item(WorkItem::new_blocking(self.name, func));
    }

    pub fn send_work_item(&mut self, item: WorkItem) {
        if let Err(err) = self.tx.try_send(item) {
            match err {
                TrySendError::Full(item) => {
                    warn!(task = item.name, "Processor queue full")
                }
                TrySendError::Closed(_) => {
                    error!("Processor queue closed unexpectedly")
                }
            }
        }
    }
}

pub struct Senders {
    example_tx: Sender,
    // todo add all the needed queues here
}

struct Receivers {
    example_rx: mpsc::Receiver<WorkItem>,
    // todo add all the needed queues here
}

pub type AsyncFn = Pin<Box<dyn Future<Output = ()> + Send + Sync>>;
pub type BlockingFn = Box<dyn FnOnce() + Send + Sync>;

enum AsyncOrBlocking {
    Async(AsyncFn),
    Blocking(BlockingFn),
}
pub struct WorkItem {
    name: &'static str,
    func: AsyncOrBlocking,
}

impl WorkItem {
    pub fn new_async(name: &'static str, func: AsyncFn) -> Self {
        Self {
            name,
            func: AsyncOrBlocking::Async(func),
        }
    }

    pub fn new_blocking(name: &'static str, func: BlockingFn) -> Self {
        Self {
            name,
            func: AsyncOrBlocking::Blocking(func),
        }
    }
}

pub async fn spawn(config: Config, executor: TaskExecutor) -> Senders {
    // todo macro? just specifying name and capacity?
    let (example_tx, example_rx) = mpsc::channel(1000);

    let senders = Senders {
        example_tx: Sender::new("example", example_tx),
    };
    let receivers = Receivers { example_rx };

    executor.spawn(processor(config, receivers, executor.clone()), "processor");
    senders
}

async fn processor(config: Config, mut receivers: Receivers, executor: TaskExecutor) {
    // TODO: consider having separate limits for blocking and async?
    let semaphore = Arc::new(Semaphore::new(config.max_workers));

    loop {
        let Ok(permit) = semaphore.clone().acquire_owned().await else {
            error!("Processor semaphore closed unexpectedly");
            break;
        };

        let work_item = select! {
            biased;
            Some(w) = receivers.example_rx.recv() => w,
            else => {
                error!("Processor queues closed unexpectedly");
                break;
            }
        };

        match work_item.func {
            AsyncOrBlocking::Async(async_fn) => executor.spawn(
                async move {
                    async_fn.await;
                    drop(permit);
                },
                work_item.name,
            ),
            AsyncOrBlocking::Blocking(blocking_fn) => {
                executor.spawn_blocking(
                    move || {
                        blocking_fn();
                        drop(permit);
                    },
                    work_item.name,
                );
            }
        }
    }
}
