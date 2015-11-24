
use alloc::boxed::{Box, FnBox};
use collections::Vec;

enum State {
    Running,
    Ready,
    Blocked,
}

#[allow(dead_code)]
pub struct Task {
    stack_mem: Box<Stack>,
    pub stack_ptr: usize,
    state: State,
}

#[allow(dead_code)]
#[repr(packed)]
struct Stack {
    stack_size: [u8; 192], // be sure it's aligned with your arch
    r11: u32,
    r10: u32,
    r9: u32,
    r7: u32,
    r8: u32,
    r6: u32,
    r5: u32,
    r4: u32,
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    psr: u32,
}

pub struct TaskList {
  tasks: Vec<Box<Task>>,
  current: usize,
}

fn task_is_finished() {
    loop { }; // TODO: kill task
}

fn start_thread(arg: u32) {
    let f: Box<Box<FnBox()>> = unsafe { Box::from_raw(arg as *mut _) };
    f();
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList { tasks: Vec::new(), current: 0 }
    }

    pub fn reschedule(&mut self) -> &Box<Task> {
        let mut next = self.current + 1;
        if next >= self.tasks.len() {
            next = 0;
        }
        self.get_current_task_mut().state = State::Ready;
        self.current = next;
        let next_task = self.tasks.get_mut(next).unwrap();
        next_task.state = State::Running;
        next_task
    }

    pub fn get_current_task(&self) -> &Box<Task> {
        self.tasks.get(self.current).unwrap()
    }

    pub fn get_current_task_mut(&mut self) -> &mut Box<Task> {
        self.tasks.get_mut(self.current).unwrap()
    }

    pub fn spawn_task<F>(&mut self, f: F) where F: FnOnce() -> (), F: 'static {
        //let pc = unsafe { *(&f as *const _ as *const u32) };
        let box_fn: Box<Box<FnBox()>> = Box::new(Box::new(f));
        let box_fn_addr = Box::into_raw(box_fn) as *mut _ as u32; //*(&box_fn as *const _) as u32; //unsafe { *(&f as *const _ as *const u32) };

        let stack_mem = Box::new(Stack {
            stack_size: [0u8; 192],
            r11: 0,
            r10: 0,
            r9: 0,
            r8: 0,
            r7: 0,
            r6: 0,
            r5: 0,
            r4: 0,
            r0: box_fn_addr,
            r1: 0,
            r2: 0,
            r3: 0x1004,
            r12: 0x1003,
            lr: unsafe { *(&task_is_finished as *const _ as *const u32) },
            pc: unsafe { *(&start_thread as *const _ as *const u32) },
            psr: 0x21000000, // PSR thumb bit FIXME are you sure?
        });
        let stack_ptr = (&*stack_mem as *const _ as usize) + 192;

        let mut task = Box::new(Task {
            stack_mem: stack_mem, stack_ptr: stack_ptr, state: State::Ready });
        self.tasks.push(task);
    }
}
