use std::rc::Weak;
use std::cell::RefCell;
use crate::timer::Timer;
#[derive(Debug)]
pub struct Mem{
    memory: [u8; 0xFFFF],
    pub timer: Weak<RefCell<Timer>>
}

impl Mem{
    pub fn new(tim:Weak<RefCell<Timer>>) -> Self{
        Mem{
            memory: [0; 0xFFFF],
            timer:tim
        }
        
    }

    pub fn read(&self, addr: usize) -> u8{
        self.memory[addr]
    }

    pub fn write(&mut self, addr: usize, val: u8){
        self.memory[addr] = val;
    }
}

