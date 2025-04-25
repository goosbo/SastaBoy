pub struct Cpu {
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_f: u8,
    reg_h: u8,
    reg_l: u8,
    sp: u16,
    pc: u16
}

impl Cpu{
    pub fn new() -> Self{
        Cpu{
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_f: 0,
            reg_h: 0,
            reg_l: 0,
            sp: 0,
            pc: 0
        }

    }

    pub fn get_af(self) -> u16{
        ((self.reg_a as u16) << 8)|(self.reg_f as u16)
    }

    pub fn set_af(&mut self,val:u16) {
        self.reg_a = (val&0xff00>>8) as u8;
        self.reg_f = (val&0xff) as u8
    }

    pub fn get_bc(self) -> u16{
        ((self.reg_b as u16) << 8)|(self.reg_c as u16)
    }

    pub fn set_bc(&mut self,val:u16) {
        self.reg_b = (val&0xff00>>8) as u8;
        self.reg_c = (val&0xff) as u8
    }

    pub fn get_de(self) -> u16{
        ((self.reg_d as u16) << 8)|(self.reg_e as u16)
    }

    pub fn set_de(&mut self,val:u16) {
        self.reg_d = (val&0xff00>>8) as u8;
        self.reg_e = (val&0xff) as u8
    }

    pub fn get_hl(self) -> u16{
        ((self.reg_h as u16) << 8)|(self.reg_l as u16)
    }

    pub fn set_hl(&mut self,val:u16) {
        self.reg_h = (val&0xff00>>8) as u8;
        self.reg_l = (val&0xff) as u8
    }

    pub fn get_zflag(self)-> bool{
        (self.reg_f & 1<<7) != 0
    }

    pub fn get_nflag(self)-> bool{
        (self.reg_f & 1<<6) != 0
    }

    pub fn get_hflag(self)-> bool{
        (self.reg_f & 1<<5) != 0
    }

    pub fn get_cflag(self)-> bool{
        (self.reg_f & 1<<4) != 0
    }

}


fn main() {
    println!("Hello, world!");
}
