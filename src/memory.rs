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
        let timer = self.timer.upgrade().expect("Timer reference dropped!");
        if addr == Timer::DIV_ADDR {
            return timer.borrow().get_div();
        }
        else if addr == Timer::TAC_ADDR {
            return timer.borrow().tac;
        }
        else if addr == Timer::TIMA_ADDR{
            return timer.borrow().tima;
        }
        else if addr == Timer::TMA_ADDR{
            return timer.borrow().tma;
        }
        self.memory[addr]
    }

    pub fn write(&mut self, addr: usize, val: u8){
        let timer = self.timer.upgrade().expect("Timer reference dropped!");

        if addr == Timer::DIV_ADDR{
            timer.borrow_mut().write_div(); // does the div increment obscure thing
            return;
        }
        else if addr == Timer::TAC_ADDR{
            timer.borrow_mut().write_tac(val);
            return;
        }
        else if addr == Timer::TIMA_ADDR{
            timer.borrow_mut().write_tima(val);
            return;
        }
        else if addr == Timer::TMA_ADDR{
            timer.borrow_mut().tma = val;
            return;
        }
        self.memory[addr] = val;
    }
}

