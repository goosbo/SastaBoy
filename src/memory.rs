use std::rc::Weak;
use std::cell::RefCell;
use crate::timer::Timer;
use crate::interrupt::InterruptHandlerThing;

#[derive(Debug)]
pub struct Mem{
    memory: [u8; 0x10000],
    pub timer: Weak<RefCell<Timer>>,
    interrupt_handler: Weak<RefCell<InterruptHandlerThing>>
}

impl Mem{
    pub fn new(tim:Weak<RefCell<Timer>>,intrrpt:Weak<RefCell<InterruptHandlerThing>>) -> Self{
        Mem{
            memory: [0x00; 0x10000],
            timer:tim,
            interrupt_handler: intrrpt
        }
        
    }

    pub fn read(&self, addr: usize) -> u8{
        let timer = self.timer.upgrade().expect("Timer reference dropped!");
        let interrupt_handl = self.interrupt_handler.upgrade().expect("Interrupt handler reference dropped!");
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
        else if addr == InterruptHandlerThing::IE_ADDR{
            return interrupt_handl.borrow().ie;
        }
        else if addr == InterruptHandlerThing::IF_ADDR{
            return interrupt_handl.borrow().if_;
        }
        else if addr == 0xFF44{
            return 0x90;
        }
        self.memory[addr]
    }

    pub fn write(&mut self, addr: usize, val: u8){
        let timer = self.timer.upgrade().expect("Timer reference dropped!");
        let interrupt_handl = self.interrupt_handler.upgrade().expect("Interrupt handler reference dropped!");

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
        else if addr == InterruptHandlerThing::IE_ADDR{
            interrupt_handl.borrow_mut().ie = val;
            return;
        }
        else if addr == InterruptHandlerThing::IF_ADDR{
            interrupt_handl.borrow_mut().if_ = val;
            return;
        }
        self.memory[addr] = val;
    }
}

