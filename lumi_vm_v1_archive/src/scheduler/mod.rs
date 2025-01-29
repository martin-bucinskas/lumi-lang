use crate::vm::{VMEvent, VM};
use std::cell::Cell;
use std::thread;

#[derive(Default)]
pub struct Scheduler {
    next_pid: Cell<u32>,
    max_pid: u32,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            next_pid: Cell::new(0),
            max_pid: 50000,
        }
    }

    pub fn get_thread(&self, mut vm: VM) -> thread::JoinHandle<Vec<VMEvent>> {
        let pid = self.next_pid.get();
        if pid >= self.max_pid {
            // TODO: handle the case where max_pid is reached
        }

        let thread_name = pid.to_string();
        self.next_pid.set(pid + 1);
        let builder = thread::Builder::new().name(thread_name);
        builder
            .spawn(move || vm.run())
            .expect("Failed to spawn thread")
    }
}
