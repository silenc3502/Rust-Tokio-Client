mod thread_control;

use tokio::time::{sleep, Duration};

struct TestObject;

impl TestObject {
    // 동기 및 비동기 함수를 모두 인자로 받을 수 있는 메서드
    async fn register_thread_function<F>(&self, custom_function: Option<F>)
        where
            F: FnOnce() -> () + Send + 'static,
    {
        // 비동기 작업 시뮬레이션을 위한 sleep
        sleep(Duration::from_secs(1)).await;

        // 등록된 함수 실행
        if let Some(func) = custom_function {
            func();
        } else {
            println!("None function registered!");
        }
    }
}

fn my_sync_function() {
    println!("Synchronous function is executed!");
}

async fn my_async_function() {
    println!("Asynchronous function is executed!");
}

#[tokio::main]
async fn main() {
    let test_object = TestObject;

    // 동기 함수를 객체의 메서드로 등록 및 실행
    test_object.register_thread_function(Some(|| {
        my_sync_function();
    })).await;

    // 비동기 함수를 객체의 메서드로 등록 및 실행
    test_object.register_thread_function(Some(|| {
        tokio::spawn(my_async_function());
    })).await;

    let custom_function = Some(move || {
        println!("Custom function executed!");
    });

    test_object.register_thread_function(custom_function).await;

    test_object.register_thread_function(None::<fn()>).await;
}