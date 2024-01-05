use std::{thread::sleep, time::Duration};
use rand::{self, Rng};
use log;

pub fn wait_rand_sec(max_sleep_mili: u64) {
    let rand_mili = rand::thread_rng().gen_range(0..max_sleep_mili);
    log::info!("sleeping for {}ms", rand_mili);
    sleep(Duration::from_millis(rand_mili));
}