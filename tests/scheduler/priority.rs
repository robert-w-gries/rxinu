extern crate alloc;

use alloc::vec;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use rxinu::task::{self, Priority, PriorityTask, TaskFuture};
use rxinu::task::scheduler::{PriorityScheduler, Scheduler};

#[test_case]
fn priority() {
    let high_prio = Arc::new(AtomicBool::new(false));
    let med_prio = Arc::new(AtomicBool::new(false));
    let low_prio = Arc::new(AtomicBool::new(false));
    let mut scheduler = PriorityScheduler::new();
    for prio in vec![Priority::Low, Priority::Low, Priority::Medium, Priority::Medium, Priority::High, Priority::High] {
        let (h, m, l) = (high_prio.clone(), med_prio.clone(), low_prio.clone());

        scheduler.spawn(PriorityTask::new(prio, async move {
            match prio {
                Priority::High => {
                    h.store(true, Ordering::SeqCst);
                    assert!(!m.load(Ordering::SeqCst));
                    assert!(!l.load(Ordering::SeqCst));
                },
                Priority::Medium => {
                    assert!(h.load(Ordering::SeqCst));
                    m.store(true, Ordering::SeqCst);
                    assert!(!l.load(Ordering::SeqCst));
                },
                Priority::Low => {
                    assert!(h.load(Ordering::SeqCst));
                    assert!(m.load(Ordering::SeqCst));
                    l.store(true, Ordering::SeqCst);
                },
            }
        })).unwrap();
    }
    scheduler.run_ready_tasks();
    assert!(high_prio.load(Ordering::SeqCst));
    assert!(med_prio.load(Ordering::SeqCst));
    assert!(low_prio.load(Ordering::SeqCst));
}

#[test_case]
fn run() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut scheduler = PriorityScheduler::new();
    let num_tasks = 5;
    for _ in 0..num_tasks {
        let c = counter.clone();
        
        scheduler.spawn(PriorityTask::new(Priority::High, async move {
            c.fetch_add(1, Ordering::SeqCst);
        })).unwrap();
    }
    scheduler.run_ready_tasks();
    assert_eq!(counter.load(Ordering::SeqCst), num_tasks);
}

#[test_case]
fn kill() {
    let mut scheduler = PriorityScheduler::new();
    let task = PriorityTask::new(Priority::High, async move {
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
    let mut executor = PriorityScheduler::new();
    executor.spawn(PriorityTask::new(Priority::High, task1)).unwrap();
    executor.spawn(PriorityTask::new(Priority::High, task2)).unwrap();
    executor.run_ready_tasks();
}
