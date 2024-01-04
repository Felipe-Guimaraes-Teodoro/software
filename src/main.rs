mod sf;
mod ui;
mod application;
mod event_loop;
mod util;
mod worker;

use crate::util::Math;
use threadpool::ThreadPool;


lazy_static::lazy_static!{
    pub static ref GLOBAL_POOL: ThreadPool = 
        ThreadPool::new(std::thread::available_parallelism().unwrap().get());
}


fn main() {
    // let x = std::thread::spawn(|| {
    //     event_loop::run();
    // });

    event_loop::run();


    // let _ = x.join();
}

// add a thread pool to the application
