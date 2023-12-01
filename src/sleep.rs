use std::{thread::sleep, time::Duration};
use rand::{self, Rng};
use crate::logs::log_sleep;

pub fn wait_rand_sec(max_sleep_mili: u64) {
    let rand_mili = rand::thread_rng().gen_range(0..max_sleep_mili);
    log_sleep(rand_mili);
    sleep(Duration::from_millis(rand_mili));
}