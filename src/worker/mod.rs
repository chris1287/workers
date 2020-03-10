pub mod hammer;
pub mod screwdriver;
pub mod wrench;

pub trait Worker {
    fn work(&self) -> Result<(), &'static str>;
}

pub enum Job {
    KnockNails,
    LooseScrews,
    ReleaseBolts
}

pub fn get_worker(job: Job) -> Box<dyn Worker> {
    match job {
        Job::KnockNails => Box::new(hammer::Hammer{}),
        Job::LooseScrews => Box::new(screwdriver::Screwdriver{}),
        Job::ReleaseBolts => Box::new(wrench::Wrench{})
    }
}
