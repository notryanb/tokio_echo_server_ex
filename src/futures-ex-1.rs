extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;

fn main() {
    let pool = CpuPool::new_num_cpus();

    let prime_future = pool.spawn_fn(|| {
        let prime = is_prime(BIG_PRIME);
        let res: Result<bool, ()> = Ok(prime);
        res
    });

    println!("Create a Future!");

    if prime_future.wait().unwrap() {
        println!("Prime!");
    } else {
        println!("Not Prime!");
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