use std::time::Duration;
use std::thread;

use crate::worker;

pub struct Screwdriver {

}

impl worker::Worker for Screwdriver {
    fn work(&self) -> Result<(), &'static str> {
        thread::sleep(Duration::from_millis(400));
        println!("screwdriver work completed");
        Ok(())
    }
}