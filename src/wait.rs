use tokio::time::{Sleep, sleep, Duration};


pub fn wait(ms: u64) -> Sleep {
    sleep(Duration::from_millis(ms))
}