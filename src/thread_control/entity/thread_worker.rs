use std::fmt;
use std::future::Future;
use std::pin::Pin;

pub struct ThreadWorker {
    name: String,
    will_be_execute_function: Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>>,
}

impl ThreadWorker {
    pub fn new(
        name: &str,
        will_be_execute_function: Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>>,
    ) -> Self {
        ThreadWorker {
            name: name.to_string(),
            will_be_execute_function,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_will_be_execute_function(
        &self,
    ) -> Option<&Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>> {
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
            (Some(f1), Some(f2)) => std::ptr::eq(f1.as_ref(), f2.as_ref()),
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
                    Some(_) => "Some(Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + Send>)",
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
        let worker2 = ThreadWorker::new("John Doe", Some(Box::new(custom_function)));

        assert_eq!(worker, worker2);
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

        if let Some(function) = found_function {
            // 클로저 실행
            let future = function();
            future.await;
        } else {
            println!("No custom function found!");
        }
    }

    #[tokio::test]
    async fn test_my_sync_function() {
        let custom_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(async {
                my_sync_function();
            })
        };
        let worker = ThreadWorker::new("John Doe", Some(Box::new(custom_function)));

        if let Some(function) = worker.get_will_be_execute_function() {
            // 클로저 실행
            let future = function();
            future.await;
        } else {
            println!("No custom function found!");
        }
    }

    #[tokio::test]
    async fn test_my_async_function() {
        let custom_function = || -> Pin<Box<dyn Future<Output = ()>>> {
            Box::pin(my_async_function())
        };
        let worker = ThreadWorker::new("John Doe", Some(Box::new(custom_function)));

        if let Some(function) = worker.get_will_be_execute_function() {
            // 클로저 실행
            let future = function();
            future.await;
        } else {
            println!("No custom function found!");
        }
    }
}
