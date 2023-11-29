use std::{thread::sleep, time::Duration};
use rand::{self, Rng};

pub fn wait_rand_sec() {
    let rand_sec = rand::thread_rng().gen_range(0..6);
    println!("[SLEEPING] {} sec", rand_sec);
    sleep(Duration::from_secs(rand_sec));
}