use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fs;
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
        let interrupt_handler = Rc::new(RefCell::new(InterruptHandlerThing::new()));
        let mem = Rc::new(RefCell::new(Mem::new(Weak::new(),Rc::downgrade(&interrupt_handler))));
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

    pub fn load_rom(&self,path: &str) {
        let data = fs::read(path).expect("Failed to read ROM file");
        for i in 0..data.len(){
            self.mem.borrow_mut().write(i, data[i]);
        }
        println!("rom size: {}",data.len());
        println!("Rom Loaded!");
    }

    pub fn run(&self){
        let mut output_buffer = String::from("");
        while self.cpu.borrow().pc < 0xFFFF {
            let mut mcycles = self.cpu.borrow_mut().execute();
            let interrupt_isr = self.interrupt_handler.borrow_mut().check_interrupt();
            if interrupt_isr != 0{
                let pc = self.cpu.borrow().pc;
                self.cpu.borrow_mut().push_stack(pc);
                self.cpu.borrow_mut().pc = interrupt_isr;
                mcycles += 5;
            }
            self.timer.borrow_mut().tick(mcycles);

            // printing serial port for blargg's test output
            if self.mem.borrow().read(0xFF02) == 0x81{
                let c = self.mem.borrow().read(0xFF01);
                print!("{}", c as char);
                output_buffer.push(c as char);
                if output_buffer.contains("Passed") || output_buffer.contains("Failed"){
                    break;
                }
                self.mem.borrow_mut().write(0xFF02, 0x0);
            }
            
        }
    }
}