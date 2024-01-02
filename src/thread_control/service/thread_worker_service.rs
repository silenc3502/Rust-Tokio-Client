use async_trait::async_trait;

#[async_trait]
pub trait ThreadWorkerServiceTrait {
    fn save_async_thread_worker(
        self,
        name: &str,
        will_be_execute_function: fn());
    fn save_sync_thead_worker(
        self,
        name: &str,
        will_be_execute_function: fn());
}

