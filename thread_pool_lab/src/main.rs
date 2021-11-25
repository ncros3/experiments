use rand::{distributions::Uniform, prelude::*};
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;
use threadpool::ThreadPool;

fn main() {
    let n_workers = 4;
    let n_jobs = 18;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for index in 0..n_jobs {
        let tx = tx.clone();
        // execute will dispatch and schedule jobs to
        // threads availables in the thread pool
        pool.execute(move || {
            // we are waiting for an unknowned duration
            let side = Uniform::new(0, 5);
            let mut rng = rand::thread_rng();
            let sleep_time: u64 = rng.sample(side);
            sleep(Duration::new(sleep_time, 0));
            // we send the message to the channel
            tx.send(index)
                .expect("channel will be there waiting for the pool");
        });
    }

    // this is equal to a while loop due to the iter() method
    // iter() will always return even if there is no messages
    // in the channel
    // we use take(n_jobs) to only iterate on n_jobs jobs
    for msg in rx.iter().take(n_jobs) {
        println!("received message : {}", msg);
    }
}
