use std::{cell::RefCell, rc::Weak};
use crate::memory::Mem;
#[derive(Debug)]
pub struct InterruptHandlerThing {
    pub ime: bool,
    mem: Weak<RefCell<Mem>> 
}

#[allow(dead_code)]
impl InterruptHandlerThing {
    pub fn new(memm: Weak<RefCell<Mem>>) -> Self{
        InterruptHandlerThing{
            ime: false,
            mem: memm
        }
    }
    
    
    pub const IE_ADDR : usize = 0xFFFF;
    pub const IF_ADDR : usize = 0xFF0F;

    pub const VBLANK_BIT : u8 = 1;
    pub const LCD_BIT : u8 = 0b10;
    pub const TIMER_BIT : u8 = 0b100;
    pub const SERIAL_LINK_BIT : u8 = 0b1000;
    pub const JOYPAD_BIT : u8 = 0b10000;

    pub const ISR_VBLANK_ADDR : u16 = 0x0040;
    pub const ISR_LCD_ADDR : u16 = 0x0048;
    pub const ISR_TIMER_ADDR : u16 = 0x0050;
    pub const ISR_SERIAL_LINK_ADDR : u16 = 0x0058;
    pub const ISR_JOYPAD_ADDR : u16 = 0x0060;

    fn req_intrpt(&self,bit:u8){
        if let Some(memory) = self.mem.upgrade(){
            let mut if_: u8 = memory.borrow_mut().read(Self::IF_ADDR);
            if_ |= bit;
            memory.borrow_mut().write(Self::IF_ADDR, if_);
        }
        else {
            panic!("memory manager reference dropped!");
        }
        
    }

    pub fn req_vblank(&self){
        Self::req_intrpt(self,Self::VBLANK_BIT);
    }

    pub fn req_lcd(&self){
        Self::req_intrpt(self, Self::LCD_BIT);
    }

    pub fn req_timer(&self){
        Self::req_intrpt( self,Self::TIMER_BIT);
    }

    pub fn req_serial_link(&self){
        Self::req_intrpt( self,Self::SERIAL_LINK_BIT);
    }

    pub fn req_joypad(&self){
        Self::req_intrpt(self, Self::JOYPAD_BIT);
    }

    pub fn set_ime(&mut self,set_value:bool){
        self.ime = set_value;
    }

    pub fn interrupt_requested(&mut self)-> bool{
        let memory = self.mem.upgrade().expect("memory manager reference dropped");

        let if_ = memory.borrow_mut().read(Self::IF_ADDR);
        let ie = memory.borrow_mut().read(Self::IE_ADDR);

        let enabled_interrupts = (if_ & ie) != 0;
        enabled_interrupts
    }

    pub fn check_interrupt(&mut self) -> u16{
        let memory = self.mem.upgrade().expect("memory manager reference dropped");
        if !self.ime {
            return 0;
        }

        let if_ = memory.borrow_mut().read(Self::IF_ADDR);
        let ie = memory.borrow_mut().read(Self::IE_ADDR);

        let enabled_interrupts = if_ & ie;
        if enabled_interrupts == 0{
            return 0;
        }

        if enabled_interrupts & Self::VBLANK_BIT != 0{
            memory.borrow_mut().write(Self::IF_ADDR, if_ & !Self::VBLANK_BIT);
            self.set_ime(false);
            return Self::ISR_VBLANK_ADDR;
        }
        else if enabled_interrupts & Self::LCD_BIT != 0 {
            memory.borrow_mut().write(Self::IF_ADDR, if_ & !Self::LCD_BIT);
            self.set_ime(false);
            return Self::ISR_LCD_ADDR;
        }
        else if enabled_interrupts & Self::TIMER_BIT != 0 {
            memory.borrow_mut().write(Self::IF_ADDR, if_ & !Self::TIMER_BIT);
            self.set_ime(false);
            return Self::ISR_TIMER_ADDR;
        }
        else if enabled_interrupts & Self::SERIAL_LINK_BIT != 0 {
            memory.borrow_mut().write(Self::IF_ADDR, if_ & !Self::SERIAL_LINK_BIT);
            self.set_ime(false);
            return Self::ISR_SERIAL_LINK_ADDR;
        }
        else if enabled_interrupts & Self::JOYPAD_BIT != 0 {
            memory.borrow_mut().write(Self::IF_ADDR, if_ & !Self::JOYPAD_BIT);
            self.set_ime(false);
            return Self::ISR_JOYPAD_ADDR;
        }
        else {
            return 0;
        }

    }

}

