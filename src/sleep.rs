use std::{thread::sleep, time::Duration};
use rand::{self, Rng};

pub fn wait_rand_sec() {
    let rand_mili = rand::thread_rng().gen_range(0..3560);
    println!("[SLEEPING] {} ms", rand_mili);
    sleep(Duration::from_millis(rand_mili));
}