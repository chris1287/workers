use std::time::Duration;
use std::thread;

use crate::worker;

pub struct Hammer {

}

impl worker::Worker for Hammer {
    fn work(&self) -> Result<(), &'static str> {
        thread::sleep(Duration::from_millis(300));
        println!("hammer work completed");
        Ok(())
    }
}