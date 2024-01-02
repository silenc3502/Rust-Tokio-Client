use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;
use crate::thread_control::entity::thread_worker::ThreadWorker;

#[async_trait]
pub trait ThreadWorkerRepositoryTrait {
    fn save_thread_worker(
        &mut self,
        name: &str,
        will_be_execute_function: Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>>,
    );
    fn find_by_name(&self, name: &str) -> Option<ThreadWorker>;
    async fn start_thread_worker(&self, name: &str);
}