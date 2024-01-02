use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::thread_control::entity::thread_worker::ThreadWorker;
use crate::thread_control::repository::thread_worker_repository::ThreadWorkerRepositoryTrait;

pub struct ThreadWorkerRepositoryImpl {
    thread_worker_list: HashMap<String, ThreadWorker>,
}

impl ThreadWorkerRepositoryImpl {
    pub fn new() -> Self {
        ThreadWorkerRepositoryImpl {
            thread_worker_list: HashMap::new(),
        }
    }

    pub fn get_instance() -> Arc<Mutex<ThreadWorkerRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<Mutex<ThreadWorkerRepositoryImpl>> =
                Arc::new(Mutex::new(ThreadWorkerRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ThreadWorkerRepositoryTrait for ThreadWorkerRepositoryImpl {
    fn save_thread_worker(
        &mut self,
        name: &str,
        will_be_execute_function: Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>>,
    ) {
        let thread_worker = ThreadWorker::new(name, will_be_execute_function);
        self.thread_worker_list.insert(name.to_string(), thread_worker);
    }

    fn find_by_name(&self, name: &str) -> Option<ThreadWorker> {
        self.thread_worker_list.get(name).cloned()
    }

    async fn start_thread_worker(&self, name: &str) {
        if let Some(worker) = self.thread_worker_list.get(name) {
            let function_arc = Arc::clone(&worker.get_will_be_execute_function().unwrap());

            // Lock the Mutex to get the guard
            let guard = function_arc.lock().await;

            // Extract the closure from the Box inside the Mutex guard
            let function = &*guard;

            // Call the closure and execute the future
            println!("Before calling the function");
            let future = function.as_ref()();
            future;

            println!("After calling the function");

            // Add an assertion to check if the worker name matches
            assert_eq!(worker.name(), name);
        } else {
            panic!("Thread worker not found: {}", name);
        }
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
    async fn test_singleton() {
        let instance1 = ThreadWorkerRepositoryImpl::get_instance();
        let instance2 = ThreadWorkerRepositoryImpl::get_instance();

        // Ensure that both instances are the same
        assert_eq!(Arc::ptr_eq(&instance1, &instance2), true);
    }

    #[tokio::test]
    async fn test_save_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository
        let mut repository = repository.lock().unwrap();

        let custom_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom function executed!");
            })
        };

        // Save a thread worker
        repository.save_thread_worker("TestWorker", Some(Box::new(custom_function)));

        // Retrieve the saved worker and execute its function
        if let Some(worker) = repository.thread_worker_list.get("TestWorker") {
            let function_arc = Arc::clone(&worker.get_will_be_execute_function().unwrap());

            // Lock the Mutex to get the guard
            let guard = function_arc.lock().await;

            // Extract the closure from the Box inside the Mutex guard
            let function = &*guard;

            // Call the closure and execute the future
            let future = function();
            future.await;

            // Add an assertion to check if the worker name matches
            assert_eq!(worker.name(), "TestWorker");
        } else {
            panic!("Thread worker not found!");
        }
    }

    #[tokio::test]
    async fn test_save_sync_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository
        let mut repository = repository.lock().unwrap();

        // Synchronous function
        let sync_custom_function = || {
            Box::pin(async {
                my_sync_function();
            }) as Pin<Box<dyn Future<Output = ()>>>
        };

        // Save a thread worker with a synchronous function
        repository.save_thread_worker("SyncTestWorker", Some(Box::new(sync_custom_function)));

        // Retrieve and execute the saved worker's function
        if let Some(worker) = repository.thread_worker_list.get("SyncTestWorker") {
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
        }
    }

    #[tokio::test]
    async fn test_save_async_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository
        let mut repository = repository.lock().unwrap();

        // Asynchronous function
        let async_custom_function = || {
            Box::pin(async {
                my_async_function().await;
            }) as Pin<Box<dyn Future<Output = ()>>>
        };

        // Save a thread worker with an asynchronous function
        repository.save_thread_worker("AsyncTestWorker", Some(Box::new(async_custom_function)));

        // Retrieve and execute the saved worker's function
        if let Some(worker) = repository.thread_worker_list.get("AsyncTestWorker") {
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
        }
    }

    #[tokio::test]
    async fn test_shared_thread_worker_list() {
        let instance1 = ThreadWorkerRepositoryImpl::get_instance();
        let instance2 = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository through instance1
        let mut repository1 = instance1.lock().unwrap();

        // Save a thread worker through instance1
        repository1.save_thread_worker("SharedTestWorker", None);

        // Drop the lock to allow other instances to access the repository
        drop(repository1);

        // Lock the mutex to access the repository through instance2
        let repository2 = instance2.lock().unwrap();

        // Check if the saved worker is visible through instance2
        assert!(repository2.thread_worker_list.contains_key("SharedTestWorker"));
    }

    #[test]
    async fn test_new_save_thread_worker() {
        let repository = ThreadWorkerRepositoryImpl::get_instance();

        // Lock the mutex to access the repository
        let mut repository = repository.lock().unwrap();

        let custom_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom function executed!");
            })
        };

        // Save a thread worker
        repository.save_thread_worker("TestWorker", Some(Box::new(custom_function)));

        repository.start_thread_worker("TestWorker").await;
    }
}
