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

    fn get_af(self) -> u16{
        ((self.reg_a as u16) << 8)|(self.reg_f as u16)
    }

    fn set_af(&mut self,val:u16) {
        self.reg_a = (val&0xff00>>8) as u8;
        self.reg_f = (val&0xff) as u8
    }

    fn get_bc(self) -> u16{
        ((self.reg_b as u16) << 8)|(self.reg_c as u16)
    }

    fn set_bc(&mut self,val:u16) {
        self.reg_b = (val&0xff00>>8) as u8;
        self.reg_c = (val&0xff) as u8
    }

    fn get_de(self) -> u16{
        ((self.reg_d as u16) << 8)|(self.reg_e as u16)
    }

    fn set_de(&mut self,val:u16) {
        self.reg_d = (val&0xff00>>8) as u8;
        self.reg_e = (val&0xff) as u8
    }

    fn get_hl(self) -> u16{
        ((self.reg_h as u16) << 8)|(self.reg_l as u16)
    }

    fn set_hl(&mut self,val:u16) {
        self.reg_h = (val&0xff00>>8) as u8;
        self.reg_l = (val&0xff) as u8
    }

    fn get_zflag(self)-> bool{
        (self.reg_f & 1<<7) != 0
    }

    fn get_nflag(self)-> bool{
        (self.reg_f & 1<<6) != 0
    }

    fn get_hflag(self)-> bool{
        (self.reg_f & 1<<5) != 0
    }

    fn get_cflag(self)-> bool{
        (self.reg_f & 1<<4) != 0
    }

    fn run_opcode(&mut self,op:u8) -> u8{
        let mcycles;
        match op{
            0x40 => {self.reg_b = self.reg_b; mcycles = 1},
            0x41 => {self.reg_b = self.reg_c; mcycles = 1},
            0x42 => {self.reg_b = self.reg_d; mcycles = 1},
            0x43 => {self.reg_b = self.reg_e; mcycles = 1},
            0x44 => {self.reg_b = self.reg_h; mcycles = 1},
            0x45 => {self.reg_b = self.reg_l; mcycles = 1},
            0x47 => {self.reg_b = self.reg_a; mcycles = 1},

            0x48 => {self.reg_c = self.reg_b; mcycles = 1},
            0x49 => {self.reg_c = self.reg_c; mcycles = 1},
            0x4A => {self.reg_c = self.reg_d; mcycles = 1},
            0x4B => {self.reg_c = self.reg_e; mcycles = 1},
            0x4C => {self.reg_c = self.reg_h; mcycles = 1},
            0x4D => {self.reg_c = self.reg_l; mcycles = 1},
            0x4F => {self.reg_c = self.reg_a; mcycles = 1},

            0x50 => {self.reg_d = self.reg_b; mcycles = 1},
            0x51 => {self.reg_d = self.reg_c; mcycles = 1},
            0x52 => {self.reg_d = self.reg_d; mcycles = 1},
            0x53 => {self.reg_d = self.reg_e; mcycles = 1},
            0x54 => {self.reg_d = self.reg_h; mcycles = 1},
            0x55 => {self.reg_d = self.reg_l; mcycles = 1},
            0x57 => {self.reg_d = self.reg_a; mcycles = 1},

            0x58 => {self.reg_e = self.reg_b; mcycles = 1},
            0x59 => {self.reg_e = self.reg_c; mcycles = 1},
            0x5A => {self.reg_e = self.reg_d; mcycles = 1},
            0x5B => {self.reg_e = self.reg_e; mcycles = 1},
            0x5C => {self.reg_e = self.reg_h; mcycles = 1},
            0x5D => {self.reg_e = self.reg_l; mcycles = 1},
            0x5F => {self.reg_e = self.reg_a; mcycles = 1},

            0x60 => {self.reg_h = self.reg_b; mcycles = 1},
            0x61 => {self.reg_h = self.reg_c; mcycles = 1},
            0x62 => {self.reg_h = self.reg_d; mcycles = 1},
            0x63 => {self.reg_h = self.reg_e; mcycles = 1},
            0x64 => {self.reg_h = self.reg_h; mcycles = 1},
            0x65 => {self.reg_h = self.reg_l; mcycles = 1},
            0x67 => {self.reg_h = self.reg_a; mcycles = 1},

            0x68 => {self.reg_l = self.reg_b; mcycles = 1},
            0x69 => {self.reg_l = self.reg_c; mcycles = 1},
            0x6A => {self.reg_l = self.reg_d; mcycles = 1},
            0x6B => {self.reg_l = self.reg_e; mcycles = 1},
            0x6C => {self.reg_l = self.reg_h; mcycles = 1},
            0x6D => {self.reg_l = self.reg_l; mcycles = 1},
            0x6F => {self.reg_l = self.reg_a; mcycles = 1},

            0x78 => {self.reg_a = self.reg_b; mcycles = 1},
            0x79 => {self.reg_a = self.reg_c; mcycles = 1},
            0x7A => {self.reg_a = self.reg_d; mcycles = 1},
            0x7B => {self.reg_a = self.reg_e; mcycles = 1},
            0x7C => {self.reg_a = self.reg_h; mcycles = 1},
            0x7D => {self.reg_a = self.reg_l; mcycles = 1},
            0x7F => {self.reg_a = self.reg_a; mcycles = 1},
            _ => {return 0}
        }
        self.pc += 1;
        return mcycles
    }

}


fn main() {
    println!("Hello, world!");
}
