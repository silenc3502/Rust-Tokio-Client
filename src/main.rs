mod thread_control;
//
// use tokio::time::{sleep, Duration};
//
// struct TestObject;
//
// impl TestObject {
//     // 동기 및 비동기 함수를 모두 인자로 받을 수 있는 메서드
//     async fn register_thread_function<F>(&self, custom_function: Option<F>)
//         where
//             F: FnOnce() -> () + Send + 'static,
//     {
//         // 비동기 작업 시뮬레이션을 위한 sleep
//         sleep(Duration::from_secs(1)).await;
//
//         // 등록된 함수 실행
//         if let Some(func) = custom_function {
//             func();
//         } else {
//             println!("None function registered!");
//         }
//     }
// }
//
// fn my_sync_function() {
//     println!("Synchronous function is executed!");
// }
//
// async fn my_async_function() {
//     println!("Asynchronous function is executed!");
// }
//
// #[tokio::main]
// async fn main() {
//     let test_object = TestObject;
//
//     // 동기 함수를 객체의 메서드로 등록 및 실행
//     test_object.register_thread_function(Some(|| {
//         my_sync_function();
//     })).await;
//
//     // 비동기 함수를 객체의 메서드로 등록 및 실행
//     test_object.register_thread_function(Some(|| {
//         tokio::spawn(my_async_function());
//     })).await;
//
//     let custom_function = Some(move || {
//         println!("Custom function executed!");
//     });
//
//     test_object.register_thread_function(custom_function).await;
//
//     test_object.register_thread_function(None::<fn()>).await;
// }

// A trait 정의
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

// A Repository trait 정의
trait ARepository {
    fn a_repo_call(&self);
    fn add_b_service_function(&mut self, name: &'static str, func: Arc<Mutex<Box<dyn Fn() + 'static + Send>>>);
    fn execute_b_service_function(&self, name: &'static str);
}

// A Repository의 구현체
struct ARepositoryImpl {
    b_service_functions: HashMap<&'static str, Arc<Mutex<Box<dyn Fn() + 'static + Send>>>>,
}

impl ARepositoryImpl {
    fn new() -> Self {
        ARepositoryImpl {
            b_service_functions: HashMap::new(),
        }
    }
}

impl ARepository for ARepositoryImpl {
    fn a_repo_call(&self) {
        println!("A Repository Call");
    }

    fn add_b_service_function(&mut self, name: &'static str, func: Arc<Mutex<Box<dyn Fn() + 'static + Send>>>) {
        self.b_service_functions.insert(name, func);
    }

    fn execute_b_service_function(&self, name: &'static str) {
        if let Some(func) = self.b_service_functions.get(name) {
            let guard = func.lock().unwrap();
            (*guard)();
        } else {
            println!("Function {} not found", name);
        }
    }
}

// A Service trait 정의
trait AService {
    fn a_service_call(&self, repository: &mut dyn ARepository);
}

// A Service의 구현체
struct AServiceImpl;

impl AService for AServiceImpl {
    fn a_service_call(&self, repository: &mut dyn ARepository) {
        println!("A Service Call");
        repository.a_repo_call();
        repository.execute_b_service_function("b_service_function1");
    }
}

// B Service trait 정의
trait BService {
    fn b_service_call(&self);
}

// B Service의 구현체
struct BServiceImpl;

impl BService for BServiceImpl {
    fn b_service_call(&self) {
        println!("B Service Call");
    }
}

// lazy_static을 사용하여 Singleton으로 만들기
lazy_static! {
    static ref A_REPO_INSTANCE: Mutex<ARepositoryImpl> = Mutex::new(ARepositoryImpl::new());
    static ref A_SERVICE_INSTANCE: AServiceImpl = AServiceImpl;
    static ref B_SERVICE_INSTANCE: BServiceImpl = BServiceImpl;
}

struct TestStruct;

// 구현하려는 함수
fn execute_function(func: &Arc<Mutex<Box<dyn Fn() + Send>>>) {
    let guard = func.lock().unwrap(); // 여기서는 lock이 실패할 일이 없다고 가정합니다.
    let closure: &dyn Fn() = &*guard;
    closure(); // 클로저 호출
}

fn main() {
    let mut a_repo_instance = A_REPO_INSTANCE.lock().unwrap();
    a_repo_instance.add_b_service_function("b_service_function1", Arc::new(Mutex::new(Box::new(|| {
        B_SERVICE_INSTANCE.b_service_call();
    }) as Box<dyn Fn() + 'static + Send>)));

    // A Service 호출
    A_SERVICE_INSTANCE.a_service_call(&mut *a_repo_instance);

    let func: Arc<Mutex<Box<dyn Fn() + Send>>> = Arc::new(Mutex::new(Box::new(|| {
        println!("Executing the function");
    }) as Box<dyn Fn() + Send>));

    // 클로저 호출
    execute_function(&func);

    // let mut a_repo_instance = A_REPO_INSTANCE.lock().unwrap();
    //
    // // A Repository에 B Service 함수 추가
    // a_repo_instance.add_b_service_function("b_service_function1", Box::new(|| {
    //     B_SERVICE_INSTANCE.b_service_call();
    // }));
    //
    // // A Service 호출
    // A_SERVICE_INSTANCE.a_service_call(&mut *a_repo_instance);
}
