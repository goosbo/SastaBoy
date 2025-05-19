mod cpu;
mod interrupt;
mod memory;

use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::interrupt::InterruptHandlerThing;
use crate::memory::Mem;
fn main(){
    // components are their own thing now :D
    let mem = Rc::new(RefCell::new(Mem::new()));
    let interrupt_handler = Rc::new(RefCell::new(InterruptHandlerThing::new(Rc::clone(&mem))));

    let mut cpu = CPU::new(
        Rc::clone(&mem),
        Rc::clone(&interrupt_handler)
    );
    cpu.run_opcode(0);
    println!("CPU: {:?}", cpu);
    println!("weeee wooo");
}