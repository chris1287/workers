mod worker;

use std::thread;

fn main() {
    let workers = vec![
        thread::spawn(|| {
            let worker = worker::get_worker(worker::Job::KnockNails);
            worker.work().unwrap();
        }),
        thread::spawn(|| {
            let worker = worker::get_worker(worker::Job::LooseScrews);
            worker.work().unwrap();
        }),
        thread::spawn(|| {
            let worker = worker::get_worker(worker::Job::ReleaseBolts);
            worker.work().unwrap();
        })
    ];

    for worker in workers {
        worker.join().unwrap();
    }

    println!("all workers successfully completed");

}
