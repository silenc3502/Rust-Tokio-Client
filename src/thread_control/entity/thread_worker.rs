use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;

#[derive(Clone)]
pub struct ThreadWorker {
    name: String,
    will_be_execute_function: Option<Arc<Mutex<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>>>>,
}

impl ThreadWorker {
    pub fn new(
        name: &str,
        will_be_execute_function: Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>>,
    ) -> Self {
        let arc_function = will_be_execute_function.map(|func| Arc::new(Mutex::new(func)));

        ThreadWorker {
            name: name.to_string(),
            will_be_execute_function: arc_function,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_will_be_execute_function(
        &self,
    ) -> Option<Arc<Mutex<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send>>>> {
        self.will_be_execute_function.clone()
    }

    pub fn get_will_be_execute_function_ref(&self) -> Option<&Arc<Mutex<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send>>>> {
        self.will_be_execute_function.as_ref()
    }
}

impl AsRef<str> for ThreadWorker {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl PartialEq for ThreadWorker {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && match (&self.will_be_execute_function, &other.will_be_execute_function) {
            (Some(f1), Some(f2)) => {
                // Use `.await` to wait for the Future to resolve to a MutexGuard
                let guard1 = tokio::runtime::Runtime::new().unwrap().block_on(timeout(Duration::from_secs(5), f1.lock()));
                let guard2 = tokio::runtime::Runtime::new().unwrap().block_on(timeout(Duration::from_secs(5), f2.lock()));

                match (guard1, guard2) {
                    (Ok(guard1), Ok(guard2)) => std::ptr::eq(&*guard1, &*guard2),
                    _ => false, // Handle the case where the lock acquisition times out
                }
            }
            (None, None) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for ThreadWorker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThreadWorker")
            .field("name", &self.name)
            .field(
                "will_be_execute_function",
                &match &self.will_be_execute_function {
                    Some(_) => "Some(Arc<Mutex<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send>>>)",
                    None => "None",
                },
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn my_sync_function() {
        println!("Synchronous function is executed!");
    }

    async fn my_async_function() {
        println!("Asynchronous function is executed!");
    }

    #[tokio::test]
    async fn test_worker_creation() {
        let worker = ThreadWorker::new("John Doe", None);
        assert_eq!(worker.name(), "John Doe");
    }

    #[tokio::test]
    async fn test_worker_as_ref() {
        let worker = ThreadWorker::new("John Doe", None);
        let name_ref: &str = worker.as_ref();
        assert_eq!(name_ref, "John Doe");
    }

    #[tokio::test]
    async fn test_custom_function() {
        let custom_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom function executed!");
            })
        };

        let worker = ThreadWorker::new("John Doe", Some(Box::new(custom_function)));
        let found_function = worker.get_will_be_execute_function();

        assert_eq!(found_function.is_some(), true);

        if let Some(arc_function) = found_function {
            // Unwrap the Arc and lock the Mutex
            let guard = arc_function.lock().await;

            // Extract the closure from the Box
            let function = &*guard;

            // Call the closure and execute the future
            let future = (function as &dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>)();
            future.await;
        } else {
            println!("No custom function found!");
        }
    }

    #[tokio::test]
    async fn test_get_will_be_execute_function() {
        let custom_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                println!("Custom function executed!");
            })
        };

        let worker = ThreadWorker::new("John Doe", Some(Box::new(custom_function)));
        let found_function = worker.get_will_be_execute_function();

        assert_eq!(found_function.is_some(), true);

        if let Some(arc_function) = found_function {
            // Unwrap the Arc and lock the Mutex
            let guard = arc_function.lock().await;

            // Extract the closure from the Box
            let function = &*guard;

            // Call the closure and execute the future
            let future = function();
            future.await;
        } else {
            println!("No custom function found!");
        }
    }

    #[tokio::test]
    async fn test_my_sync_function() {
        // Use Box::pin(async {}) to wrap the synchronous function call in an asynchronous block
        let custom_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                my_sync_function();
            })
        };

        let worker = ThreadWorker::new("John Doe", Some(Box::new(custom_function)));

        if let Some(arc_function) = worker.get_will_be_execute_function() {
            let guard = arc_function.lock().await;

            // Extract the closure from the Box
            let function = &*guard;

            // Call the closure and execute the future
            let future = function();
            future.await;
        } else {
            println!("No custom function found!");
        }
    }

    #[tokio::test]
    async fn test_my_async_function() {
        let custom_function = || {
            Box::pin(my_async_function()) as Pin<Box<dyn Future<Output = ()>>>
        };

        let worker = ThreadWorker::new("John Doe", Some(Box::new(custom_function)));

        if let Some(arc_function) = worker.get_will_be_execute_function() {
            let guard = arc_function.lock().await;

            // Extract the closure from the Box
            let function = &*guard;

            // Call the closure and execute the future
            let future = function();
            future.await;
        } else {
            println!("No custom function found!");
        }
    }

}
