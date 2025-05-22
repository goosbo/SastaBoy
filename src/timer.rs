use std::cell::RefCell;
use std::rc::Weak;

use crate::interrupt::InterruptHandlerThing;

#[derive(Debug)]
pub struct Timer{
    div_ctr : u16,
    pub tma: u8,
    pub tac: u8,
    pub tima: u8,
    interrupt_handler: Weak<RefCell<InterruptHandlerThing>>,
    tima_overflow_pending: bool,
    tima_overflow_tcycles: u8
}

// did not implement obscure timer behaviour and integrate it with the cpu yet
impl Timer {

    pub const DIV_ADDR: usize = 0xFF04;
    pub const TIMA_ADDR: usize = 0xFF05;
    pub const TMA_ADDR: usize = 0xFF06;
    pub const TAC_ADDR: usize = 0xFF07;

    pub fn new(intrrpt: Weak<RefCell<InterruptHandlerThing>>) -> Self{
        Timer{
            div_ctr: 0,
            interrupt_handler: intrrpt,
            tima: 0,
            tma: 0,
            tac: 0,
            tima_overflow_pending: false,
            tima_overflow_tcycles: 0
        }
    }

    pub fn get_div(&self) -> u8{
        ((self.div_ctr>>8)&0xFF) as u8
    }

    pub fn write_div(&mut self){
       let interrupt_handl = self.interrupt_handler.upgrade().expect("Interrupt handler reference dropped!");
        let old_and_result = self.and_result();
        self.div_ctr = 0;
        let new_and_result = self.and_result();
        if self.tac & 4 != 0 && old_and_result && !new_and_result && !self.tima_overflow_pending{
            self.tima = self.tima.wrapping_add(1);
            if self.tima == 0{
                self.tima = self.tma;
                interrupt_handl.borrow_mut().req_timer();
            }
        }
    }

    pub fn write_tac(&mut self,val:u8){
        let interrupt_handl = self.interrupt_handler.upgrade().expect("Interrupt handler reference dropped!");
        let old_enable = self.tac & 4 != 0;
        let old_and_result = self.and_result();
        self.tac = val;
        let new_and_result = self.and_result();

        if old_enable && old_and_result && !new_and_result && !self.tima_overflow_pending{
            self.tima = self.tima.wrapping_add(1);
            if self.tima == 0{
                self.tima = self.tma;
                interrupt_handl.borrow_mut().req_timer();
            }
        }
    }

    pub fn write_tima(&mut self,val: u8){
        self.tima = val;
        if self.tima_overflow_pending{
            self.tima_overflow_pending = false;
            self.tima_overflow_tcycles = 0;
        }
    }

    fn and_result(&self)-> bool{
        let bit: u8 = match self.tac & 3 {
            0 => 9,
            1 => 3,
            2 => 5,
            3 => 7,
            _ => unreachable!()
        };
        return ((self.div_ctr >> bit)&1)&(((self.tac as u16 &0x4)>>2)&1) != 0
    }

    pub fn tick(&mut self, mcycles:u8){
        let interrupt_handl = self.interrupt_handler.upgrade().expect("Interrupt handler reference dropped!");
        let tcycles = 4*mcycles;

        for _ in 0..tcycles{
            let old_and_result = self.and_result();
            self.div_ctr = self.div_ctr.wrapping_add(1);
            
            let new_and_result = self.and_result();

            if self.tima_overflow_pending{
                self.tima_overflow_tcycles -= 1;
                    if self.tima_overflow_tcycles == 0{
                    self.tima = self.tma;
                    self.tima_overflow_pending = false;
                    interrupt_handl.borrow_mut().req_timer();
                }
            }

            
            if self.tac & 4 != 0 && old_and_result && !new_and_result && !self.tima_overflow_pending{
                self.tima = self.tima.wrapping_add(1);
                if self.tima == 0{
                    self.tima_overflow_pending = true;
                    self.tima_overflow_tcycles = 4;
                }
            }
        }
        
    }
}