// use std::future::Future;
// use std::pin::Pin;
// use async_trait::async_trait;
// use crate::thread_control::repository::thread_worker_repository::ThreadWorkerRepositoryTrait;
// use crate::thread_control::repository::thread_worker_repository_impl::ThreadWorkerRepositoryImpl;
// use crate::thread_control::service::thread_worker_service::ThreadWorkerServiceTrait;
//
// pub struct ThreadWorkerServiceImpl {
//     repository: ThreadWorkerRepositoryImpl,
// }
//
// impl ThreadWorkerServiceImpl {
//     pub fn new(repository: ThreadWorkerRepositoryImpl) -> Self {
//         ThreadWorkerServiceImpl { repository }
//     }
// }
//
// static mut THREAD_WORKER_SERVICE: Option<ThreadWorkerServiceImpl> = None;
//
// // Expose a function to get the singleton instance
// pub fn get_instance() -> &'static ThreadWorkerServiceImpl {
//     unsafe {
//         if THREAD_WORKER_SERVICE.is_none() {
//             let service_instance = ThreadWorkerServiceImpl::new(ThreadWorkerRepositoryImpl::new());
//             THREAD_WORKER_SERVICE = Some(service_instance);
//         }
//         THREAD_WORKER_SERVICE.as_ref().unwrap()
//     }
// }
//
// #[async_trait]
// impl ThreadWorkerServiceTrait for ThreadWorkerServiceImpl {
//     fn save_async_thread_worker(mut self, name: &str, will_be_execute_function: fn()) {
//         let async_function = || {
//             Box::pin(async {
//                 will_be_execute_function()
//             }) as Pin<Box<dyn Future<Output = ()>>>
//         };
//
//         self.repository.save_thread_worker(name, Some(Box::new(async_function)));
//     }
//
//     fn save_sync_thead_worker(mut self, name: &str, will_be_execute_function: fn()) {
//         let sync_function = || {
//             Box::pin(async {
//                 will_be_execute_function()
//             }) as Pin<Box<dyn Future<Output = ()>>>
//         };
//
//         self.repository.save_thread_worker(name, Some(Box::new(sync_function)));
//     }
// }