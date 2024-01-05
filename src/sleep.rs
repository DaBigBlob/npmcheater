use std::{thread::sleep, time::Duration};
use rand::{self, Rng, rngs::ThreadRng};
use log;

// pub fn wait_rand_sec(max_sleep_mili: u64) {
//     let rand_mili = rand::thread_rng().gen_range(0..max_sleep_mili);
//     log::info!("sleeping for {}ms", rand_mili);
//     sleep(Duration::from_millis(rand_mili));
// }

pub struct Rand {
    max_mili: u64,
    generator: ThreadRng
}
impl Rand {
    pub fn init(max_sleep_mili: u64) -> Rand{
        Rand {
            max_mili: max_sleep_mili,
            generator: rand::thread_rng()
        }
    }
    
    pub fn sleep(&mut self) {
        let rand_mili = self.generator.gen_range(0..self.max_mili);
        log::info!("sleeping for {}ms", rand_mili);
        sleep(Duration::from_millis(rand_mili));
    }
}