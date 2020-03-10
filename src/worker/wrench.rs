use std::time::Duration;
use std::thread;

use crate::worker;

pub struct Wrench {

}

impl worker::Worker for Wrench {
    fn work(&self) -> Result<(), &'static str> {
        thread::sleep(Duration::from_millis(600));
        println!("wrench work completed");
        Ok(())
    }
}