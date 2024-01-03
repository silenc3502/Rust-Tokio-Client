use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::runtime::Handle;
use tokio::task;
use crate::thread_control::repository::thread_worker_repository::ThreadWorkerRepositoryTrait;
use crate::thread_control::repository::thread_worker_repository_impl::ThreadWorkerRepositoryImpl;
use crate::thread_control::service::thread_worker_service::ThreadWorkerServiceTrait;

pub struct ThreadWorkerServiceImpl {
    repository: Arc<Mutex<ThreadWorkerRepositoryImpl>>,
}

impl ThreadWorkerServiceImpl {
    pub fn new(repository: Arc<Mutex<ThreadWorkerRepositoryImpl>>) -> Self {
        ThreadWorkerServiceImpl { repository }
    }

    pub fn get_instance() -> Arc<Mutex<ThreadWorkerServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<Mutex<ThreadWorkerServiceImpl>> =
                Arc::new(Mutex::new(ThreadWorkerServiceImpl::new(ThreadWorkerRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ThreadWorkerServiceTrait for ThreadWorkerServiceImpl {
    fn save_async_thread_worker(&mut self, name: &str, will_be_execute_function: Arc<Mutex<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send>>) {
        let async_function = move || -> Pin<Box<dyn Future<Output = ()>>> {
            let will_be_execute_function = Arc::clone(&will_be_execute_function);
            Box::pin(async move {
                (will_be_execute_function.lock().unwrap())().await
            })
        };

        self.repository.lock().unwrap().save_thread_worker(name, Some(Box::new(async_function)));
    }

    fn save_sync_thread_worker(&mut self, name: &str, will_be_execute_function: Arc<Mutex<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send>>) {
        let sync_function = move || -> Pin<Box<dyn Future<Output = ()>>> {
            let will_be_execute_function = Arc::clone(&will_be_execute_function);
            Box::pin(async move {
                (will_be_execute_function.lock().unwrap())().await
            })
        };

        self.repository.lock().unwrap().save_thread_worker(name, Some(Box::new(sync_function)));
    }

    async fn start_thread_worker(self, name: &str) {
        let repository_lock = self.repository.lock().unwrap();

        task::block_in_place(move || {
            Handle::current().block_on(async move {
                repository_lock.start_thread_worker(name).await;
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    fn my_sync_function() {
        println!("Synchronous function is executed!");
    }

    async fn my_async_function() {
        println!("Asynchronous function is executed!");
    }

    #[test]
    async fn test_save_async_thread_worker() {
        let thread_worker_repository = ThreadWorkerRepositoryImpl::get_instance();
        let mut service = ThreadWorkerServiceImpl::new(thread_worker_repository);
        // let mut service = ThreadWorkerServiceImpl::get_instance();

        let async_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom async function executed!");
            })
        };

        service.save_async_thread_worker("AsyncTestWorker", Arc::new(Mutex::new(async_function)));

        // Retrieve the saved worker and execute its function
        if let Some(worker) = service.repository.lock().unwrap().find_by_name("AsyncTestWorker") {
            let function_arc = Arc::clone(&worker.get_will_be_execute_function().unwrap());

            // Lock the Mutex to get the guard
            let guard = function_arc.lock().await;

            // Extract the closure from the Box inside the Mutex guard
            let function = &*guard;

            // Call the closure and execute the future
            let future = function();
            future.await;

            // Add an assertion to check if the worker name matches
            assert_eq!(worker.name(), "AsyncTestWorker");
        } else {
            panic!("Thread worker not found: AsyncTestWorker");
        };
    }

    #[test]
    async fn test_save_sync_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();
        let mut service = ThreadWorkerServiceImpl::new(repository);

        let sync_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom sync function executed!");
            })
        };

        service.save_sync_thread_worker("SyncTestWorker", Arc::new(Mutex::new(sync_function)));

        // Retrieve the saved worker and execute its function
        if let Some(worker) = service.repository.lock().unwrap().find_by_name("SyncTestWorker") {
            let function_arc = Arc::clone(&worker.get_will_be_execute_function().unwrap());

            // Lock the Mutex to get the guard
            let guard = function_arc.lock().await;

            // Extract the closure from the Box inside the Mutex guard
            let function = &*guard;

            // Call the closure and execute the future
            let future = function();
            future.await;

            // Add an assertion to check if the worker name matches
            assert_eq!(worker.name(), "SyncTestWorker");
        } else {
            panic!("Thread worker not found: SyncTestWorker");
        };
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_async_thread_and_start() {
        let thread_worker_repository = ThreadWorkerRepositoryImpl::get_instance();
        let mut service = ThreadWorkerServiceImpl::new(thread_worker_repository);
        // let mut service = ThreadWorkerServiceImpl::get_instance();

        let async_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom async function executed!");
            })
        };

        service.save_async_thread_worker("AsyncTestWorker", Arc::new(Mutex::new(async_function)));
        service.start_thread_worker("AsyncTestWorker").await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_save_sync_thread_and_start() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();
        let mut service = ThreadWorkerServiceImpl::new(repository);

        let sync_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom sync function executed!");
            })
        };

        service.save_sync_thread_worker("SyncTestWorker", Arc::new(Mutex::new(sync_function)));
        service.start_thread_worker("SyncTestWorker").await;
    }
}