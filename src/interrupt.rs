use crate::cpu::MemBus;
#[derive(Debug)]
pub struct InterruptHandlerThing {
    ime: bool,
}

impl InterruptHandlerThing {
    pub fn new() -> Self{
        InterruptHandlerThing{
            ime: false,
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

    fn req_intrpt(membus: &mut MemBus,bit:u8){
        let mut if_: u8 = membus.read(Self::IF_ADDR);
        if_ |= bit;
        membus.write(Self::IF_ADDR, if_);
    }

    pub fn req_vblank(membus: &mut MemBus){
        Self::req_intrpt(membus, Self::VBLANK_BIT);
    }

    pub fn req_lcd(membus: &mut MemBus){
        Self::req_intrpt(membus, Self::LCD_BIT);
    }

    pub fn req_timer(membus: &mut MemBus){
        Self::req_intrpt(membus, Self::TIMER_BIT);
    }

    pub fn req_serial_link(membus: &mut MemBus){
        Self::req_intrpt(membus, Self::SERIAL_LINK_BIT);
    }

    pub fn req_joypad(membus: &mut MemBus){
        Self::req_intrpt(membus, Self::JOYPAD_BIT);
    }

    pub fn set_ime(&mut self,set_value:bool){
        self.ime = set_value;
    }

    pub fn check_interrupt(&mut self, membus:&mut MemBus) -> u16{
        if !self.ime {
            return 0;
        }

        let if_ = membus.read(Self::IF_ADDR);
        let ie = membus.read(Self::IE_ADDR);

        let enabled_interrupts = if_ & ie;
        if enabled_interrupts == 0{
            return 0;
        }

        if enabled_interrupts & Self::VBLANK_BIT != 0{
            membus.write(Self::IF_ADDR, if_ & !Self::VBLANK_BIT);
            self.set_ime(false);
            return Self::ISR_VBLANK_ADDR;
        }
        else if enabled_interrupts & Self::LCD_BIT != 0 {
            membus.write(Self::IF_ADDR, if_ & !Self::LCD_BIT);
            self.set_ime(false);
            return Self::ISR_LCD_ADDR;
        }
        else if enabled_interrupts & Self::TIMER_BIT != 0 {
            membus.write(Self::IF_ADDR, if_ & !Self::TIMER_BIT);
            self.set_ime(false);
            return Self::ISR_TIMER_ADDR;
        }
        else if enabled_interrupts & Self::SERIAL_LINK_BIT != 0 {
            membus.write(Self::IF_ADDR, if_ & !Self::SERIAL_LINK_BIT);
            self.set_ime(false);
            return Self::ISR_SERIAL_LINK_ADDR;
        }
        else if enabled_interrupts & Self::JOYPAD_BIT != 0 {
            membus.write(Self::IF_ADDR, if_ & !Self::JOYPAD_BIT);
            self.set_ime(false);
            return Self::ISR_JOYPAD_ADDR;
        }
        else {
            return 0;
        }

    }

}

