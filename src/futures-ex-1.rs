extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;

use std::time::Duration;

use futures::Future;
use futures_cpupool::CpuPool;
use tokio_timer::Timer;

fn main() {
    let pool = CpuPool::new_num_cpus();
    let timer = Timer::default();

    let timeout = timer.sleep(Duration::from_millis(750))
        .then(|_| Err(()));

    let prime = pool.spawn_fn(|| {
        Ok(is_prime(BIG_PRIME))
    });

    let winner =  timeout.select(prime).map(|(win, _)| win);

    match winner.wait() {
        Ok(true) => println!("Prime!"),
        Ok(false) => println!("Not Prime!"),
        Err(_) => println!("Time out."),
    }
}

const BIG_PRIME: u64 = 15485867;

// checks whether a number is prime, slowly
fn is_prime(num: u64) -> bool {
    for i in 2..num {
        if num % i == 0 { return false }
    }
    true
}