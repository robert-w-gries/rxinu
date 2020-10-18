extern crate alloc;

use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use rxinu::task::{self, Task, TaskFuture};
use rxinu::task::scheduler::{RoundRobinScheduler, Scheduler};

#[test_case]
fn run() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut scheduler = RoundRobinScheduler::new();
    let num_tasks = 5;
    for _ in 0..num_tasks {
        let c = counter.clone();
        
        scheduler.spawn(Task::new(async move {
            c.fetch_add(1, Ordering::SeqCst);
        })).unwrap();
    }
    scheduler.run_ready_tasks();
    assert_eq!(counter.load(Ordering::SeqCst), num_tasks);
}

#[test_case]
fn kill() {
    let mut scheduler = RoundRobinScheduler::new();
    let task = Task::new(async move {
        panic!("Process should have been killed");
    });
    let pid = task.id();
    scheduler.spawn(task).unwrap();
    scheduler.kill(pid).unwrap();
    scheduler.run_ready_tasks();
}

/// Spawn Task1 then spawn Task2
/// Task1 yields
/// Task2 sets has_run to true and finishes
/// Task1 returns to yield point and asserts has_run is true
#[test_case]
fn yield_now() {
    let has_run = Arc::new(AtomicBool::new(false));
    let ref1 = has_run.clone();
    let task1 = async move {
        task::yield_now().await;
        assert!(ref1.load(Ordering::SeqCst));
    };
    let task2 = async move {
        has_run.store(true, Ordering::SeqCst);
    };
    let mut executor = RoundRobinScheduler::new();
    executor.spawn(Task::new(task1)).unwrap();
    executor.spawn(Task::new(task2)).unwrap();
    executor.run_ready_tasks();
}
