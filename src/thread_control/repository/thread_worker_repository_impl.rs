use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
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

impl ThreadWorkerRepositoryTrait for ThreadWorkerRepositoryImpl {
    fn save_thread_worker(
        &mut self,
        name: &str,
        will_be_execute_function: Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>>,
    ) {
        let thread_worker = ThreadWorker::new(name, will_be_execute_function);
        self.thread_worker_list.insert(name.to_string(), thread_worker);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

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
            let function = worker.get_will_be_execute_function().unwrap();

            // Use tokio::task::spawn_blocking to execute the function in a blocking context
            tokio::task::spawn_blocking(move || {
                // Synchronously execute the function using tokio::task::block_in_place
                tokio::task::block_in_place(|| {
                    tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(function());
                });
            })
                .await
                .unwrap();

            // Add an assertion to check if the worker name matches
            assert_eq!(worker.name(), "TestWorker");
        } else {
            panic!("Thread worker not found!");
        }
    }
}
