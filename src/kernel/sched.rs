
use alloc::boxed::Box;
use collections::Vec;

struct Task {
    sp: u32,
}

pub type TaskList = Vec<Box<Task>>;

fn reschedule(tasks: &mut TaskList) -> Box<Task> {
    tasks.pop().unwrap()
}

