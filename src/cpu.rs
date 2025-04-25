
#[derive(Debug)]
pub struct MemBus{
    mem: [u8; 0xFFFF]
}

impl MemBus{
    pub fn read(&self, addr: usize) -> u8{
        self.mem[addr]
    }

    pub fn write(&mut self, addr: usize, val: u8){
        self.mem[addr] = val;
    }
}

#[derive(Debug)]
pub struct CPU {
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_f: u8,
    reg_h: u8,
    reg_l: u8,
    sp: u16,
    pc: u16,
    bus: MemBus
}

impl CPU{
    pub fn new() -> Self{
        CPU{
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_f: 0,
            reg_h: 0,
            reg_l: 0,
            sp: 0,
            pc: 0,
            bus: MemBus{
                mem: [0; 0xFFFF]
            }
        }

    }

    fn get_af(&self) -> u16{
        ((self.reg_a as u16) << 8)|(self.reg_f as u16)
    }

    fn set_af(&mut self,val:u16) {
        self.reg_a = (val&0xff00>>8) as u8;
        self.reg_f = (val&0xff) as u8
    }

    fn get_bc(&self) -> u16{
        ((self.reg_b as u16) << 8)|(self.reg_c as u16)
    }

    fn set_bc(&mut self,val:u16) {
        self.reg_b = (val&0xff00>>8) as u8;
        self.reg_c = (val&0xff) as u8
    }

    fn get_de(&self) -> u16{
        ((self.reg_d as u16) << 8)|(self.reg_e as u16)
    }

    fn set_de(&mut self,val:u16) {
        self.reg_d = (val&0xff00>>8) as u8;
        self.reg_e = (val&0xff) as u8
    }

    fn get_hl(&self) -> u16{
        ((self.reg_h as u16) << 8)|(self.reg_l as u16)
    }

    fn set_hl(&mut self,val:u16) {
        self.reg_h = (val&0xff00>>8) as u8;
        self.reg_l = (val&0xff) as u8
    }

    fn get_zero(&self)-> bool{
         (self.reg_f & 1<<7) != 0
    }

    fn get_neg(&self)-> bool{
         (self.reg_f & 1<<6) != 0
    }

    fn get_halfcarry(&self)-> bool{
         (self.reg_f & 1<<5) != 0
    }

    fn get_carry(&self)-> bool{
        (self.reg_f & 1<<4) != 0
    }

    fn set_zero(&mut self, val:bool){
        if val{
            self.reg_f |= 1<<7;
        }else{
            self.reg_f &= !(1<<7);
        }
    }
    fn set_neg(&mut self, val:bool){
        if val{
            self.reg_f |= 1<<6;
        }else{
            self.reg_f &= !(1<<6);
        }
    }
    fn set_halfcarry(&mut self, val:bool){
        if val{
            self.reg_f |= 1<<5;
        }else{
            self.reg_f &= !(1<<5);
        }
    }
    fn set_carry(&mut self, val:bool){
        if val{
            self.reg_f |= 1<<4;
        }else{
            self.reg_f &= !(1<<4);
        }
    }

   fn push_stack(&mut self, val:u16){
        self.sp -= 2;
        self.bus.write(self.sp as usize, (val&0xff) as u8);
        self.bus.write((self.sp+1) as usize, (val>>8) as u8);
    }
    fn pop_stack(&mut self) -> u16{
        let val = (self.bus.read(self.sp as usize) as u16) | ((self.bus.read((self.sp+1) as usize) as u16)<<8);
        self.sp += 2;
        return val
    }

    fn add(&mut self, val:u8){
        let result = self.reg_a.wrapping_add(val);
        self.set_zero(result == 0);
        self.set_neg(false);
        self.set_halfcarry((self.reg_a&0x0F) + (val&0x0F) > 0x0F);
        self.set_carry((self.reg_a as u16&0xFF) + (val as u16&0xFF) > 0xFF);
        self.reg_a = result;
    }

    fn add_carry(&mut self, val:u8){
        let carry = self.get_carry() as u8;
        let result = self.reg_a.wrapping_add(val).wrapping_add(carry);
        self.set_zero(result == 0);
        self.set_neg(false);
        self.set_halfcarry((self.reg_a&0x0F) + (val&0x0F) + carry> 0x0F);
        self.set_carry((self.reg_a as u16&0xFF) + (val as u16&0xFF) + carry as u16 > 0xFF);
        self.reg_a = result;
    }

    fn sub(&mut self, val:u8){
        let result = self.reg_a.wrapping_sub(val);
        self.set_zero(result == 0);
        self.set_neg(true);
        self.set_halfcarry((self.reg_a&0x0F) < (val&0x0F));
        self.set_carry((self.reg_a as u16) < (val as u16));
        self.reg_a = result;
    }

    fn sub_carry(&mut self, val:u8){
        let carry = self.get_carry() as u8;
        let result = self.reg_a.wrapping_sub(val).wrapping_sub(carry);
        self.set_zero(result == 0);
        self.set_neg(true);
        self.set_halfcarry((self.reg_a&0x0F) < (val&0x0F) + carry);
        self.set_carry((self.reg_a as u16) < (val as u16) + carry as u16);
        self.reg_a = result;
    }

    pub fn run_opcode(&mut self,op:u8) -> u8{
        let mut mcycles = 0;
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

            0x06 => {
                let z: u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.reg_b = z;
                mcycles = 2;
            },
            0x0E => {
                let z: u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.reg_c = z;
                mcycles = 2;
            },
            0x16 => {
                let z: u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.reg_d = z;
                mcycles = 2;
            },
            0x1E => {
                let z: u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.reg_e = z;
                mcycles = 2;
            },
            0x26 => {
                let z: u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.reg_h = z;
                mcycles = 2;
            },
            0x2E => {
                let z: u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.reg_l = z;
                mcycles = 2;
            },
            0x3E => {
                let z: u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.reg_a = z;
                mcycles = 2;
            },

            0x46 => {
                let z:u8 = self.bus.read(self.get_hl() as usize);
                self.reg_b = z;
                mcycles = 2;
            },
            0x4E => {
                let z:u8 = self.bus.read(self.get_hl() as usize);
                self.reg_c = z;
                mcycles = 2;
            },
            0x56 => {
                let z:u8 = self.bus.read(self.get_hl() as usize);
                self.reg_d = z;
                mcycles = 2;
            },
            0x5E => {
                let z:u8 = self.bus.read(self.get_hl() as usize);
                self.reg_e = z;
                mcycles = 2;
            },
            0x66 => {
                let z:u8 = self.bus.read(self.get_hl() as usize);
                self.reg_h = z;
                mcycles = 2;
            },
            0x6E => {
                let z:u8 = self.bus.read(self.get_hl() as usize);
                self.reg_l = z;
                mcycles = 2;
            },
            0x7E => {
                let z:u8 = self.bus.read(self.get_hl() as usize);
                self.reg_a = z;
                mcycles = 2;
            },

            0x70 => {
                self.bus.write(self.get_hl() as usize, self.reg_b);
                mcycles = 2;
            },
            0x71 => {
                self.bus.write(self.get_hl() as usize, self.reg_c);
                mcycles = 2;
            },
            0x72 => {
                self.bus.write(self.get_hl() as usize, self.reg_d);
                mcycles = 2;
            },
            0x73 => {
                self.bus.write(self.get_hl() as usize, self.reg_e);
                mcycles = 2;
            },
            0x74 => {
                self.bus.write(self.get_hl() as usize, self.reg_h);
                mcycles = 2;
            },
            0x75 => {
                self.bus.write(self.get_hl() as usize, self.reg_l);
                mcycles = 2;
            },
            0x77 => {
                self.bus.write(self.get_hl() as usize, self.reg_a);
                mcycles = 2;
            },

            0x36 => {
                let z = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.bus.write(self.get_hl() as usize, z);
                mcycles = 3;
            },

            0x0A => {
                let z:u8 = self.bus.read(self.get_bc() as usize);
                self.reg_a = z;
                mcycles = 2;
            },
            0x1A => {
                let z:u8 = self.bus.read(self.get_de() as usize);
                self.reg_a = z;
                mcycles = 2;
            },
            0x02 => {
                self.bus.write(self.get_bc() as usize, self.reg_a);
                mcycles = 2;
            },
            0x12 => {
                self.bus.write(self.get_de() as usize, self.reg_a);
                mcycles = 2;
            },
            0xFA => {
                let z:u16 = self.bus.read(self.pc as usize) as u16;
                self.pc += 1;
                let z2:u16 = self.bus.read(self.pc as usize) as u16;
                self.pc += 1;
                self.reg_a = self.bus.read((z|z2<<8) as usize);
                mcycles = 4;
            },
            0xEA => {
                let z:u16 = self.bus.read(self.pc as usize) as u16;
                self.pc += 1;
                let z2:u16 = self.bus.read(self.pc as usize) as u16;
                self.pc += 1;
                self.bus.write((z|z2<<8) as usize, self.reg_a);
                mcycles = 4;
            },

            0xF2 => {
                let z:u8 = self.bus.read((0xFF00|(self.reg_c as u16)) as usize);
                self.reg_a = z;
                mcycles = 2;
            },
            0xE2 => {
                self.bus.write((0xFF00|self.reg_c as u16) as usize, self.reg_a);
                mcycles = 2;
            },

            0xF0 => {
                let z:u8 = self.bus.read((0xFF00|(self.pc as u16)) as usize);
                self.pc += 1;
                self.reg_a = z;
                mcycles = 3;
            },
            0xE0 => {
                let z:u8 = self.bus.read((0xFF00|(self.pc as u16)) as usize);
                self.pc += 1;
                self.bus.write((0xFF00|(z as u16)) as usize, self.reg_a);
                mcycles = 3;
            },

            0x3A => {
                let z:u8 = self.bus.read(self.get_hl() as usize);
                self.reg_a = z;
                self.set_hl(self.get_hl()-1);
                mcycles = 2;
            },
            0x32 => {
                self.bus.write(self.get_hl() as usize, self.reg_a);
                self.set_hl(self.get_hl()-1);
                mcycles = 2;
            },

            0x2A => {
                let z:u8 = self.bus.read(self.get_hl() as usize);
                self.reg_a = z;
                self.set_hl(self.get_hl()+1);
                mcycles = 2;
            },
            0x22 => {
                self.bus.write(self.get_hl() as usize, self.reg_a);
                self.set_hl(self.get_hl()+1);
                mcycles = 2;
            },

            0x01 => {
                let z:u16 = self.bus.read(self.pc as usize) as u16;
                self.pc += 1;
                let z2:u16 = self.bus.read(self.pc as usize) as u16;
                self.pc += 1;
                self.set_bc(z|(z2<<8));
                mcycles = 3;
            },
            0x11 => {
                let z:u16 = self.bus.read(self.pc as usize) as u16;
                self.pc += 1;
                let z2:u16 = self.bus.read(self.pc as usize) as u16;
                self.pc += 1;
                self.set_de(z|z2<<8);
                mcycles = 3;
            },
            0x21 => {
                let z:u16 = self.bus.read(self.pc as usize) as u16;
                self.pc += 1;
                let z2:u16 = self.bus.read(self.pc as usize) as u16;
                self.pc += 1;
                self.set_hl(z|z2<<8);
                mcycles = 3;
            },

            0x08 => {
                let z:u16 = self.bus.read(self.pc as usize) as u16;
                self.pc += 1;
                let z2:u16 = self.bus.read(self.pc as usize) as u16;
                self.pc += 1;
                self.bus.write((z|z2<<8) as usize, (self.sp&0xff) as u8);
                self.bus.write((z|z2<<8) as usize+1, (self.sp>>8) as u8);
                mcycles = 5;
            },
            0xF9 => {
                self.sp = self.get_hl();
                mcycles = 2;
            },

            0xC5 => {
                self.push_stack(self.get_bc());
                mcycles = 4;
            },
            0xD5 => {
                self.push_stack(self.get_de());
                mcycles = 4;
            },
            0xE5 => {
                self.push_stack(self.get_hl());
                mcycles = 4;
            },
            0xF5 => {
                self.push_stack(self.get_af());
                mcycles = 4;
            },

            0xC1 => {
                let z = self.pop_stack();
                self.set_bc(z);
                mcycles = 3;
            },
            0xD1 => {
                let z = self.pop_stack();
                self.set_de(z);
                mcycles = 3;
            },
            0xE1 => {
                let z = self.pop_stack();
                self.set_hl(z);
                mcycles = 3;
            },
            0xF1 => {
                let z = self.pop_stack();
                self.set_af(z);
                mcycles = 3;
            },

            // might not be implemented correctly so gotta check later
            0xF8 => {
                let z:i8 = self.bus.read(self.pc as usize) as i8;
                self.pc += 1;
                let result = self.sp.wrapping_add(z as u16);
                self.set_hl(result);
                self.set_zero(false);
                self.set_neg(false);
                self.set_halfcarry((self.sp&0x0F) + (z as u16&0x0F) > 0x0F);
                self.set_carry((self.sp&0xFF) + (z as u16&0xFF) > 0xFF);
                mcycles = 3;
            },

            0x80 => {
                self.add(self.reg_b);
                mcycles = 1;
            },
            0x81 => {
                self.add(self.reg_c);
                mcycles = 1;
            },
            0x82 => {
                self.add(self.reg_d);
                mcycles = 1;
            },
            0x83 => {
                self.add(self.reg_e);
                mcycles = 1;
            },
            0x84 => {
                self.add(self.reg_h);
                mcycles = 1;
            },
            0x85 => {
                self.add(self.reg_l);
                mcycles = 1;
            },
            0x87 => {
                self.add(self.reg_a);
                mcycles = 1;
            },
            0x86 => {
                self.add(self.bus.read(self.get_hl() as usize));
                mcycles = 2;
            },

            0xC6 => {
                let z:u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.add(z);
                mcycles = 2;
            },

            0x88 => {
                self.add_carry(self.reg_b);
                mcycles = 1;
            },
            0x89 => {
                self.add_carry(self.reg_c);
                mcycles = 1;
            },
            0x8A => {
                self.add_carry(self.reg_d);
                mcycles = 1;
            },
            0x8B => {
                self.add_carry(self.reg_e);
                mcycles = 1;
            },
            0x8C => {
                self.add_carry(self.reg_h);
                mcycles = 1;
            },
            0x8D => {
                self.add_carry(self.reg_l);
                mcycles = 1;
            },
            0x8F => {
                self.add_carry(self.reg_a);
                mcycles = 1;
            },

            0x8E => {
                self.add_carry(self.bus.read(self.get_hl() as usize));
                mcycles = 2;
            },

            0xCE => {
                let z:u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.add_carry(z);
                mcycles = 2;
            },

            0x90 => {
                self.sub(self.reg_b);
                mcycles = 1;
            },
            0x91 => {
                self.sub(self.reg_c);
                mcycles = 1;
            },
            0x92 => {
                self.sub(self.reg_d);
                mcycles = 1;
            },
            0x93 => {
                self.sub(self.reg_e);
                mcycles = 1;
            },
            0x94 => {
                self.sub(self.reg_h);
                mcycles = 1;
            },
            0x95 => {
                self.sub(self.reg_l);
                mcycles = 1;
            },
            0x97 => {
                self.sub(self.reg_a);
                mcycles = 1;
            },

            0x96 => {
                self.sub(self.bus.read(self.get_hl() as usize));
                mcycles = 2;
            },

            0xD6 => {
                let z:u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.sub(z);
                mcycles = 2;
            },

            0x98 => {
                self.sub_carry(self.reg_b);
                mcycles = 1;
            },
            0x99 => {
                self.sub_carry(self.reg_c);
                mcycles = 1;
            },
            0x9A => {
                self.sub_carry(self.reg_d);
                mcycles = 1;
            },
            0x9B => {
                self.sub_carry(self.reg_e);
                mcycles = 1;
            },
            0x9C => {
                self.sub_carry(self.reg_h);
                mcycles = 1;
            },
            0x9D => {
                self.sub_carry(self.reg_l);
                mcycles = 1;
            },
            0x9F => {
                self.sub_carry(self.reg_a);
                mcycles = 1;
            },

            0x9E => {
                self.sub_carry(self.bus.read(self.get_hl() as usize));
                mcycles = 2;
            },
            0xDE => {
                let z:u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.sub_carry(z);
                mcycles = 2;
            },
            _ => ()
        }
        //self.pc += 1;
        return mcycles
    }

}
