use std::cell::RefCell;
use std::rc::Rc;

use crate::interrupt::InterruptHandlerThing;
use crate::memory::Mem;

pub struct Timer{
    div_ctr : u16,
    tima_ctr: u16,
    interrupt_handler: Rc<RefCell<InterruptHandlerThing>>,
    mem: Rc<RefCell<Mem>>
}

// did not implement obscure timer behaviour and integrate it with the cpu yet
impl Timer {

    pub const DIV_ADDR: usize = 0xFF04;
    pub const TIMA_ADDR: usize = 0xFF05;
    pub const TMA_ADDR: usize = 0xFF06;
    pub const TAC_ADDR: usize = 0xFF07;

    pub fn new(intrrpt: Rc<RefCell<InterruptHandlerThing>>,memm: Rc<RefCell<Mem>>) -> Self{
        Timer{
            div_ctr: 0,
            tima_ctr: 0,
            interrupt_handler: intrrpt,
            mem: memm
        }
    }

    pub fn tick(&mut self, mcycles:u8){
        let tcycles = 4*mcycles;
        self.div_ctr = self.div_ctr.wrapping_add(tcycles as u16);

        self.mem.borrow_mut().write(Self::DIV_ADDR, ((self.div_ctr>>8)&0xFF) as u8);

        let tac = self.mem.borrow_mut().read(Self::TAC_ADDR);
        let mut tima = self.mem.borrow_mut().read(Self::TIMA_ADDR);
        let tma = self.mem.borrow_mut().read(Self::TMA_ADDR);
        if tac & 4 != 0{
            let freq_div = match tac & 3 {
              0 =>  256,
              1 => 4,
              2 => 16,
              3 => 64,
              _ => unreachable!()
            };
            self.tima_ctr = self.tima_ctr.wrapping_add(mcycles as u16);
            let inc = self.tima_ctr/freq_div;

            if inc > 0{
                self.tima_ctr %= freq_div;

                for _ in 0..inc{
                    tima = tima.wrapping_add(1);
                    if tima == 0{
                        tima = tma;
                        self.interrupt_handler.borrow_mut().req_timer();
                    }
                }
                self.mem.borrow_mut().write(Self::TIMA_ADDR, tima);
            }

        }
    }
}