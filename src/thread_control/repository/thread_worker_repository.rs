use std::future::Future;
use std::pin::Pin;
use async_trait::async_trait;

#[async_trait]
pub trait ThreadWorkerRepositoryTrait {
    fn save_thread_worker(
        &mut self,
        name: &str,
        will_be_execute_function: Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>>,
    );
    // fn find_by_name(&self, name: &str) -> Option<ThreadWorker>;
    // fn start_thread_worker(&self, name: &str);
}