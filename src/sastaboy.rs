use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::cpu::CPU;
use crate::interrupt::InterruptHandlerThing;
use crate::memory::Mem;
use crate::timer::Timer;

#[derive(Debug)]
pub struct SastaBoy {
    pub cpu: Rc<RefCell<CPU>>,
    interrupt_handler: Rc<RefCell<InterruptHandlerThing>>,
    mem: Rc<RefCell<Mem>>,
    timer: Rc<RefCell<Timer>>
}

impl SastaBoy {
    pub fn new() -> Self{
        let mem = Rc::new(RefCell::new(Mem::new(Weak::new())));
        let interrupt_handler = Rc::new(RefCell::new(InterruptHandlerThing::new(Rc::downgrade(&mem))));
        let timer = Rc::new(RefCell::new(Timer::new(Rc::downgrade(&interrupt_handler))));
        mem.borrow_mut().timer = Rc::downgrade(&timer);
        let cpu = Rc::new(RefCell::new(CPU::new(Rc::downgrade(&mem), Rc::downgrade(&interrupt_handler))));

        SastaBoy { 
            cpu: cpu,
            interrupt_handler: interrupt_handler,
            mem: mem,
            timer: timer    
        }
    }

    pub fn run(&self){
        // temporary
        self.cpu.borrow_mut().run_opcode(0xFB);
    }
}