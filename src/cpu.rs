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
    bus: MemBus,
    ime: bool,
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
            ime: false,
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

    // helper functions to handle instructions
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

    fn compare(&mut self, val:u8){
        let result = self.reg_a.wrapping_sub(val);
        self.set_zero(result == 0);
        self.set_neg(true);
        self.set_halfcarry((self.reg_a&0x0F) < (val&0x0F));
        self.set_carry((self.reg_a as u16) < (val as u16));
    }

    fn increment(&mut self, val:u8) -> u8{
        let result = val.wrapping_add(1);
        self.set_zero(result == 0);
        self.set_neg(false);
        self.set_halfcarry((val&0x0F) + 1 > 0x0F);
        return result
    }
    fn decrement(&mut self, val:u8) -> u8{
        let result = val.wrapping_sub(1);
        self.set_zero(result == 0);
        self.set_neg(true);
        self.set_halfcarry((val&0x0F) < 1);
        return result
    }

    fn and(&mut self, val:u8){
        let result = self.reg_a & val;
        self.set_zero(result == 0);
        self.set_neg(true);
        self.set_halfcarry(true);
        self.set_carry(false);
        self.reg_a = result;
    }

    fn or(&mut self, val:u8){
        let result = self.reg_a | val;
        self.set_zero(result == 0);
        self.set_neg(false);
        self.set_halfcarry(false);
        self.set_carry(false);
        self.reg_a = result;
    }

    fn xor(&mut self, val:u8){
        let result = self.reg_a ^ val;
        self.set_zero(result == 0);
        self.set_neg(false);
        self.set_halfcarry(false);
        self.set_carry(false);
        self.reg_a = result;
    }

    fn add16(&mut self, val:u16){
        let result = self.get_hl().wrapping_add(val);
        self.set_neg(false);
        self.set_halfcarry((self.get_hl()&0x0FFF) + (val&0x0FFF) > 0x0FFF);
        self.set_carry((self.get_hl() as u32) + (val as u32) > 0xFFFF);
        self.set_hl(result);
    }

    fn rotate_left(&mut self, val:u8) -> u8{
        let new_carry = val >> 7;
        let carry = self.get_carry() as u8;
        let result = (val << 1) | carry;
        self.set_carry(new_carry != 0);
        self.set_neg(false);
        self.set_halfcarry(false);
        return result
    }
    fn rotate_right(&mut self, val:u8) -> u8{
        let new_carry = val & 1;
        let carry = self.get_carry() as u8;
        let result = (val >> 1) | (carry << 7);
        self.set_carry(new_carry != 0);
        self.set_neg(false);
        self.set_halfcarry(false);
        return result
    }
    fn rotate_left_carry(&mut self, val:u8) -> u8{
        let carry = val >> 7;
        let result = (val << 1) | carry;
        self.set_carry(carry != 0);
        self.set_neg(false);
        self.set_halfcarry(false);
        return result
    }
    fn rotate_right_carry(&mut self, val:u8) -> u8{
        let carry = val & 1;
        let result = (val >> 1) | (carry << 7);
        self.set_carry(carry != 0);
        self.set_neg(false);
        self.set_halfcarry(false);
        return result
    }

    fn sla(&mut self, val:u8) -> u8{
        let result = val << 1;
        self.set_carry((val >>7) != 0);
        self.set_zero(result == 0);
        self.set_neg(false);
        self.set_halfcarry(false);
        return result
    }

    fn sra(&mut self, val:u8) -> u8{
        let result = (val as i8) >> 1;
        self.set_carry((val & 1) != 0);
        self.set_zero(result == 0);
        self.set_neg(false);
        self.set_halfcarry(false);
        return result as u8
    }

    fn swap(&mut self, val:u8) -> u8{
        let result = ((val & 0x0F) << 4) | (val >> 4);
        self.set_zero(result == 0);
        self.set_neg(false);
        self.set_halfcarry(false);
        self.set_carry(false);
        return result
    }

    fn srl(&mut self, val:u8) -> u8{
        let result = val >> 1;
        self.set_carry((val & 1) != 0);
        self.set_zero(result == 0);
        self.set_neg(false);
        self.set_halfcarry(false);
        return result
    }

    fn bit(&mut self, val:u8, bit:u8){
        let result = (val>>bit) & 1;
        self.set_zero(result == 0);
        self.set_neg(false);
        self.set_halfcarry(true);
    }

    fn res(&mut self, val:u8, bit:u8) -> u8{
        let result = val & !(1<<bit);
        return result
    }

    fn set(&mut self, val:u8, bit:u8) -> u8{
        let result = val | (1<<bit);
        return result
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

            0xB8 => {
                self.compare(self.reg_b);
                mcycles = 1;
            },
            0xB9 => {
                self.compare(self.reg_c);
                mcycles = 1;
            },
            0xBA => {
                self.compare(self.reg_d);
                mcycles = 1;
            },
            0xBB => {
                self.compare(self.reg_e);
                mcycles = 1;
            },
            0xBC => {
                self.compare(self.reg_h);
                mcycles = 1;
            },
            0xBD => {
                self.compare(self.reg_l);
                mcycles = 1;
            },
            0xBF => {
                self.compare(self.reg_a);
                mcycles = 1;
            },
            0xBE => {
                self.compare(self.bus.read(self.get_hl() as usize));
                mcycles = 2;
            },
            0xFE => {
                let z:u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.compare(z);
                mcycles = 2;
            },

            0x04 => {
                self.reg_b = self.increment(self.reg_b);
                mcycles = 1;
            },
            0x0C => {
                self.reg_c = self.increment(self.reg_c);
                mcycles = 1;
            },
            0x14 => {
                self.reg_d = self.increment(self.reg_d);
                mcycles = 1;
            },
            0x1C => {
                self.reg_e = self.increment(self.reg_e);
                mcycles = 1;
            },
            0x24 => {
                self.reg_h = self.increment(self.reg_h);
                mcycles = 1;
            },
            0x2C => {
                self.reg_l = self.increment(self.reg_l);
                mcycles = 1;
            },
            0x3C => {
                self.reg_a = self.increment(self.reg_a);
                mcycles = 1;
            },
            0x34 => {
                let z = self.bus.read(self.get_hl() as usize);
                let result = self.increment(z);
                self.bus.write(self.get_hl() as usize, result);
                mcycles = 3;
            },
            0x05 => {
                self.reg_b = self.decrement(self.reg_b);
                mcycles = 1;
            },
            0x0D => {
                self.reg_c = self.decrement(self.reg_c);
                mcycles = 1;
            },
            0x15 => {
                self.reg_d = self.decrement(self.reg_d);
                mcycles = 1;
            },
            0x1D => {
                self.reg_e = self.decrement(self.reg_e);
                mcycles = 1;
            },
            0x25 => {
                self.reg_h = self.decrement(self.reg_h);
                mcycles = 1;
            },
            0x2D => {
                self.reg_l = self.decrement(self.reg_l);
                mcycles = 1;
            },
            0x3D => {
                self.reg_a = self.decrement(self.reg_a);
                mcycles = 1;
            },
            0x35 => {
                let z = self.bus.read(self.get_hl() as usize);
                let result = self.decrement(z);
                self.bus.write(self.get_hl() as usize, result);
                mcycles = 3;
            },

            0xA0 => {
                self.and(self.reg_b);
                mcycles = 1;
            },
            0xA1 => {
                self.and(self.reg_c);
                mcycles = 1;
            },
            0xA2 => {
                self.and(self.reg_d);
                mcycles = 1;
            },
            0xA3 => {
                self.and(self.reg_e);
                mcycles = 1;
            },
            0xA4 => {
                self.and(self.reg_h);
                mcycles = 1;
            },
            0xA5 => {
                self.and(self.reg_l);
                mcycles = 1;
            },
            0xA7 => {
                self.and(self.reg_a);
                mcycles = 1;
            },
            0xA6 => {
                self.and(self.bus.read(self.get_hl() as usize));
                mcycles = 2;
            },
            0xE6 => {
                let z:u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.and(z);
                mcycles = 2;
            },

            0xB0 => {
                self.or(self.reg_b);
                mcycles = 1;
            },
            0xB1 => {
                self.or(self.reg_c);
                mcycles = 1;
            },
            0xB2 => {
                self.or(self.reg_d);
                mcycles = 1;
            },
            0xB3 => {
                self.or(self.reg_e);
                mcycles = 1;
            },
            0xB4 => {
                self.or(self.reg_h);
                mcycles = 1;
            },
            0xB5 => {
                self.or(self.reg_l);
                mcycles = 1;
            },
            0xB7 => {
                self.or(self.reg_a);
                mcycles = 1;
            },
            0xB6 => {
                self.or(self.bus.read(self.get_hl() as usize));
                mcycles = 2;
            },
            0xF6 => {
                let z:u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.or(z);
                mcycles = 2;
            },

            0xA8 => {
                self.xor(self.reg_b);
                mcycles = 1;
            },
            0xA9 => {
                self.xor(self.reg_c);
                mcycles = 1;
            },
            0xAA => {
                self.xor(self.reg_d);
                mcycles = 1;
            },
            0xAB => {
                self.xor(self.reg_e);
                mcycles = 1;
            },
            0xAC => {
                self.xor(self.reg_h);
                mcycles = 1;
            },
            0xAD => {
                self.xor(self.reg_l);
                mcycles = 1;
            },
            0xAF => {
                self.xor(self.reg_a);
                mcycles = 1;
            },
            0xAE => {
                self.xor(self.bus.read(self.get_hl() as usize));
                mcycles = 2;
            },
            0xEE => {
                let z:u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.xor(z);
                mcycles = 2;
            },

            0x3F => {
                self.set_neg(false);
                self.set_halfcarry(false);
                self.set_carry(!self.get_carry());
                mcycles = 1;
            }
            0x37 => {
                self.set_neg(false);
                self.set_halfcarry(false);
                self.set_carry(true);
                mcycles = 1;
            },

            0x27 =>{
                let mut offset:u8 = 0;
                let mut carry = false;
                if (!self.get_neg() && self.reg_a & 0x0F > 0x09) || self.get_halfcarry() {
                    offset |= 0x06;
                } 
                if(!self.get_neg() && self.reg_a > 0x99) || self.get_carry() {
                    offset |= 0x60;
                    carry = true;
                }

                if self.get_neg() {
                    self.reg_a = self.reg_a.wrapping_sub(offset);
                } else {
                    self.reg_a = self.reg_a.wrapping_add(offset);
                }
                self.set_zero(self.reg_a == 0);
                self.set_halfcarry(false);
                self.set_carry(carry);
                mcycles = 1;

            },

            0x2F => {
                self.reg_a = !self.reg_a;
                self.set_neg(true);
                self.set_halfcarry(false);
                mcycles = 1;
            },

            0x03 =>{
                self.set_bc(self.get_bc()+1);
                mcycles = 2; 
            },
            0x13 =>{
                self.set_de(self.get_de()+1);
                mcycles = 2; 
            },
            0x23 =>{
                self.set_hl(self.get_hl()+1);
                mcycles = 2; 
            },

            0x0B =>{
                self.set_bc(self.get_bc()-1);
                mcycles = 2; 
            },
            0x1B =>{
                self.set_de(self.get_de()-1);
                mcycles = 2; 
            },
            0x2B =>{
                self.set_hl(self.get_hl()-1);
                mcycles = 2; 
            },

            0x09 => {
                self.add16(self.get_bc());
                mcycles = 2;
            },
            0x19 => {
                self.add16(self.get_de());
                mcycles = 2;
            },
            0x29 => {
                self.add16(self.get_hl());
                mcycles = 2;
            },

            0xE8 => {
                let z:i8 = self.bus.read(self.pc as usize) as i8;
                self.pc += 1;
                let result = self.get_hl().wrapping_add(z as u16);
                self.set_hl(result);
                self.set_zero(false);
                self.set_neg(false);
                self.set_halfcarry((self.get_hl()&0x0F) + (z as u16&0x0F) > 0x0F);
                self.set_carry((self.get_hl()&0xFF) + (z as u16&0xFF) > 0xFF);
                mcycles = 4;
            },

            0x07 => {
                self.reg_a = self.rotate_left_carry(self.reg_a);
                self.set_zero(false);
                mcycles = 1;
            },

            0x0F => {
                self.reg_a = self.rotate_right_carry(self.reg_a);
                self.set_zero(false);
                mcycles = 1;
            },

            0x17 => {
                self.reg_a = self.rotate_left(self.reg_a);
                self.set_zero(false);
                mcycles = 1;
            },

            0x1F => {
                self.reg_a = self.rotate_right(self.reg_a);
                self.set_zero(false);
                mcycles = 1;
            },
            // cb instructions (might try to shrink it later cuz code repeated a lot)
            0xCB => {
                let op2:u8 = self.bus.read(self.pc as usize);
                self.pc += 1;
                match op2{
                    0x00 => {
                        self.reg_b = self.rotate_left_carry(self.reg_b);
                        self.set_zero(self.reg_b == 0);
                        mcycles = 2;
                    },
                    0x01 => {
                        self.reg_c = self.rotate_left_carry(self.reg_c);
                        self.set_zero(self.reg_c == 0);
                        mcycles = 2;
                    },
                    0x02 => {
                        self.reg_d = self.rotate_left_carry(self.reg_d);
                        self.set_zero(self.reg_d == 0);
                        mcycles = 2;
                    },
                    0x03 => {
                        self.reg_e = self.rotate_left_carry(self.reg_e);
                        self.set_zero(self.reg_e == 0);
                        mcycles = 2;
                    },
                    0x04 => {
                        self.reg_h = self.rotate_left_carry(self.reg_h);
                        self.set_zero(self.reg_h == 0);
                        mcycles = 2;
                    },
                    0x05 => {
                        self.reg_l = self.rotate_left_carry(self.reg_l);
                        self.set_zero(self.reg_l == 0);
                        mcycles = 2;
                    },
                    0x07 => {
                        self.reg_a = self.rotate_left_carry(self.reg_a);
                        self.set_zero(self.reg_a == 0);
                        mcycles = 2;
                    },

                    0x06 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.rotate_left_carry(z);
                        self.bus.write(self.get_hl() as usize, result);
                        self.set_zero(result == 0);
                        mcycles = 4;
                    },

                    0x08 => {
                        self.reg_b = self.rotate_right_carry(self.reg_b);
                        self.set_zero(self.reg_b == 0);
                        mcycles = 2;
                    },
                    0x09 => {
                        self.reg_c = self.rotate_right_carry(self.reg_c);
                        self.set_zero(self.reg_c == 0);
                        mcycles = 2;
                    },
                    0x0A => {
                        self.reg_d = self.rotate_right_carry(self.reg_d);
                        self.set_zero(self.reg_d == 0);
                        mcycles = 2;
                    },
                    0x0B => {
                        self.reg_e = self.rotate_right_carry(self.reg_e);
                        self.set_zero(self.reg_e == 0);
                        mcycles = 2;
                    },
                    0x0C => {
                        self.reg_h = self.rotate_right_carry(self.reg_h);
                        self.set_zero(self.reg_h == 0);
                        mcycles = 2;
                    },
                    0x0D => {
                        self.reg_l = self.rotate_right_carry(self.reg_l);
                        self.set_zero(self.reg_l == 0);
                        mcycles = 2;
                    },
                    0x0F => {
                        self.reg_a = self.rotate_right_carry(self.reg_a);
                        self.set_zero(self.reg_a == 0);
                        mcycles = 2;
                    },
                    0x0E => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.rotate_right_carry(z);
                        self.bus.write(self.get_hl() as usize, result);
                        self.set_zero(result == 0);
                        mcycles = 4;
                    },
                    0x10 => {
                        self.reg_b = self.rotate_left(self.reg_b);
                        self.set_zero(self.reg_b == 0);
                        mcycles = 2;
                    },
                    0x11 => {
                        self.reg_c = self.rotate_left(self.reg_c);
                        self.set_zero(self.reg_c == 0);
                        mcycles = 2;
                    },
                    0x12 => {
                        self.reg_d = self.rotate_left(self.reg_d);
                        self.set_zero(self.reg_d == 0);
                        mcycles = 2;
                    },
                    0x13 => {
                        self.reg_e = self.rotate_left(self.reg_e);
                        self.set_zero(self.reg_e == 0);
                        mcycles = 2;
                    },
                    0x14 => {
                        self.reg_h = self.rotate_left(self.reg_h);
                        self.set_zero(self.reg_h == 0);
                        mcycles = 2;
                    },
                    0x15 => {
                        self.reg_l = self.rotate_left(self.reg_l);
                        self.set_zero(self.reg_l == 0);
                        mcycles = 2;
                    },
                    0x17 => {
                        self.reg_a = self.rotate_left(self.reg_a);
                        self.set_zero(self.reg_a == 0);
                        mcycles = 2;
                    },
                    0x16 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.rotate_left(z);
                        self.bus.write(self.get_hl() as usize, result);
                        self.set_zero(result == 0);
                        mcycles = 4;
                    },
                    0x18 => {
                        self.reg_b = self.rotate_right(self.reg_b);
                        self.set_zero(self.reg_b == 0);
                        mcycles = 2;
                    },
                    0x19 => {
                        self.reg_c = self.rotate_right(self.reg_c);
                        self.set_zero(self.reg_c == 0);
                        mcycles = 2;
                    },
                    0x1A => {
                        self.reg_d = self.rotate_right(self.reg_d);
                        self.set_zero(self.reg_d == 0);
                        mcycles = 2;
                    },
                    0x1B => {
                        self.reg_e = self.rotate_right(self.reg_e);
                        self.set_zero(self.reg_e == 0);
                        mcycles = 2;
                    },
                    0x1C => {
                        self.reg_h = self.rotate_right(self.reg_h);
                        self.set_zero(self.reg_h == 0);
                        mcycles = 2;
                    },
                    0x1D => {
                        self.reg_l = self.rotate_right(self.reg_l);
                        self.set_zero(self.reg_l == 0);
                        mcycles = 2;
                    },
                    0x1F => {
                        self.reg_a = self.rotate_right(self.reg_a);
                        self.set_zero(self.reg_a == 0);
                        mcycles = 2;
                    },
                    0x1E => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.rotate_right(z);
                        self.bus.write(self.get_hl() as usize, result);
                        self.set_zero(result == 0);
                        mcycles = 4;
                    },

                    0x20 => {
                        self.reg_b = self.sla(self.reg_b);
                        mcycles = 2;
                    },
                    0x21 => {
                        self.reg_c = self.sla(self.reg_c);
                        mcycles = 2;
                    },
                    0x22 => {
                        self.reg_d = self.sla(self.reg_d);
                        mcycles = 2;
                    },
                    0x23 => {
                        self.reg_e = self.sla(self.reg_e);
                        mcycles = 2;
                    },
                    0x24 => {
                        self.reg_h = self.sla(self.reg_h);
                        mcycles = 2;
                    },
                    0x25 => {
                        self.reg_l = self.sla(self.reg_l);
                        mcycles = 2;
                    },
                    0x27 => {
                        self.reg_a = self.sla(self.reg_a);
                        mcycles = 2;
                    },
                    0x26 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.sla(z);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },

                    0x28 => {
                        self.reg_b = self.sra(self.reg_b);
                        mcycles = 2;
                    },
                    0x29 => {
                        self.reg_c = self.sra(self.reg_c);
                        mcycles = 2;
                    },
                    0x2A => {
                        self.reg_d = self.sra(self.reg_d);
                        mcycles = 2;
                    },
                    0x2B => {
                        self.reg_e = self.sra(self.reg_e);
                        mcycles = 2;
                    },
                    0x2C => {
                        self.reg_h = self.sra(self.reg_h);
                        mcycles = 2;
                    },
                    0x2D => {
                        self.reg_l = self.sra(self.reg_l);
                        mcycles = 2;
                    },
                    0x2F => {
                        self.reg_a = self.sra(self.reg_a);
                        mcycles = 2;
                    },
                    0x2E => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.sra(z);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },

                    0x30 => {
                        self.reg_b = self.swap(self.reg_b);
                        mcycles = 2;
                    },
                    0x31 => {
                        self.reg_c = self.swap(self.reg_c);
                        mcycles = 2;
                    },
                    0x32 => {
                        self.reg_d = self.swap(self.reg_d);
                        mcycles = 2;
                    },
                    0x33 => {
                        self.reg_e = self.swap(self.reg_e);
                        mcycles = 2;
                    },
                    0x34 => {
                        self.reg_h = self.swap(self.reg_h);
                        mcycles = 2;
                    },
                    0x35 => {
                        self.reg_l = self.swap(self.reg_l);
                        mcycles = 2;
                    },
                    0x37 => {
                        self.reg_a = self.swap(self.reg_a);
                        mcycles = 2;
                    },
                    0x36 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.swap(z);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },

                    0x38 => {
                        self.reg_b = self.srl(self.reg_b);
                        mcycles = 2;
                    },
                    0x39 => {
                        self.reg_c = self.srl(self.reg_c);
                        mcycles = 2;
                    },
                    0x3A => {
                        self.reg_d = self.srl(self.reg_d);
                        mcycles = 2;
                    },
                    0x3B => {
                        self.reg_e = self.srl(self.reg_e);
                        mcycles = 2;
                    },
                    0x3C => {
                        self.reg_h = self.srl(self.reg_h);
                        mcycles = 2;
                    },
                    0x3D => {
                        self.reg_l = self.srl(self.reg_l);
                        mcycles = 2;
                    },
                    0x3F => {
                        self.reg_a = self.srl(self.reg_a);
                        mcycles = 2;
                    },
                    0x3E => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.srl(z);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },

                    0x40 => {
                        self.bit(self.reg_b, 0);
                        mcycles = 2;
                    },
                    0x41 => {
                        self.bit(self.reg_c, 0);
                        mcycles = 2;
                    },
                    0x42 => {
                        self.bit(self.reg_d, 0);
                        mcycles = 2;
                    },
                    0x43 => {
                        self.bit(self.reg_e, 0);
                        mcycles = 2;
                    },
                    0x44 => {
                        self.bit(self.reg_h, 0);
                        mcycles = 2;
                    },
                    0x45 => {
                        self.bit(self.reg_l, 0);
                        mcycles = 2;
                    },
                    0x47 => {
                        self.bit(self.reg_a, 0);
                        mcycles = 2;
                    },
                    0x46 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        self.bit(z, 0);
                        mcycles = 3;
                    },
                    0x48 => {
                        self.bit(self.reg_b, 1);
                        mcycles = 2;
                    },
                    0x49 => {
                        self.bit(self.reg_c, 1);
                        mcycles = 2;
                    },
                    0x4A => {
                        self.bit(self.reg_d, 1);
                        mcycles = 2;
                    },
                    0x4B => {
                        self.bit(self.reg_e, 1);
                        mcycles = 2;
                    },
                    0x4C => {
                        self.bit(self.reg_h, 1);
                        mcycles = 2;
                    },
                    0x4D => {
                        self.bit(self.reg_l, 1);
                        mcycles = 2;
                    },
                    0x4F => {
                        self.bit(self.reg_a, 1);
                        mcycles = 2;
                    },
                    0x4E => {
                        let z = self.bus.read(self.get_hl() as usize);
                        self.bit(z, 1);
                        mcycles = 3;
                    },
                    0x50 => {
                        self.bit(self.reg_b, 2);
                        mcycles = 2;
                    },
                    0x51 => {
                        self.bit(self.reg_c, 2);
                        mcycles = 2;
                    },
                    0x52 => {
                        self.bit(self.reg_d, 2);
                        mcycles = 2;
                    },
                    0x53 => {
                        self.bit(self.reg_e, 2);
                        mcycles = 2;
                    },
                    0x54 => {
                        self.bit(self.reg_h, 2);
                        mcycles = 2;
                    },
                    0x55 => {
                        self.bit(self.reg_l, 2);
                        mcycles = 2;
                    },
                    0x57 => {
                        self.bit(self.reg_a, 2);
                        mcycles = 2;
                    },
                    0x56 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        self.bit(z, 2);
                        mcycles = 3;
                    },
                    0x58 => {
                        self.bit(self.reg_b, 3);
                        mcycles = 2;
                    },
                    0x59 => {
                        self.bit(self.reg_c, 3);
                        mcycles = 2;
                    },
                    0x5A => {
                        self.bit(self.reg_d, 3);
                        mcycles = 2;
                    },
                    0x5B => {
                        self.bit(self.reg_e, 3);
                        mcycles = 2;
                    },
                    0x5C => {
                        self.bit(self.reg_h, 3);
                        mcycles = 2;
                    },
                    0x5D => {
                        self.bit(self.reg_l, 3);
                        mcycles = 2;
                    },
                    0x5F => {
                        self.bit(self.reg_a, 3);
                        mcycles = 2;
                    },
                    0x5E => {
                        let z = self.bus.read(self.get_hl() as usize);
                        self.bit(z, 3);
                        mcycles = 3;
                    },
                    0x60 => {
                        self.bit(self.reg_b, 4);
                        mcycles = 2;
                    },
                    0x61 => {
                        self.bit(self.reg_c, 4);
                        mcycles = 2;
                    },
                    0x62 => {
                        self.bit(self.reg_d, 4);
                        mcycles = 2;
                    },
                    0x63 => {
                        self.bit(self.reg_e, 4);
                        mcycles = 2;
                    },
                    0x64 => {
                        self.bit(self.reg_h, 4);
                        mcycles = 2;
                    },
                    0x65 => {
                        self.bit(self.reg_l, 4);
                        mcycles = 2;
                    },
                    0x67 => {
                        self.bit(self.reg_a, 4);
                        mcycles = 2;
                    },
                    0x66 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        self.bit(z, 4);
                        mcycles = 3;
                    },
                    0x68 => {
                        self.bit(self.reg_b, 5);
                        mcycles = 2;
                    },
                    0x69 => {
                        self.bit(self.reg_c, 5);
                        mcycles = 2;
                    },
                    0x6A => {
                        self.bit(self.reg_d, 5);
                        mcycles = 2;
                    },
                    0x6B => {
                        self.bit(self.reg_e, 5);
                        mcycles = 2;
                    },
                    0x6C => {
                        self.bit(self.reg_h, 5);
                        mcycles = 2;
                    },
                    0x6D => {
                        self.bit(self.reg_l, 5);
                        mcycles = 2;
                    },
                    0x6F => {
                        self.bit(self.reg_a, 5);
                        mcycles = 2;
                    },
                    0x6E => {
                        let z = self.bus.read(self.get_hl() as usize);
                        self.bit(z, 5);
                        mcycles = 3;
                    },
                    0x70 => {
                        self.bit(self.reg_b, 6);
                        mcycles = 2;
                    },
                    0x71 => {
                        self.bit(self.reg_c, 6);
                        mcycles = 2;
                    },
                    0x72 => {
                        self.bit(self.reg_d, 6);
                        mcycles = 2;
                    },
                    0x73 => {
                        self.bit(self.reg_e, 6);
                        mcycles = 2;
                    },
                    0x74 => {
                        self.bit(self.reg_h, 6);
                        mcycles = 2;
                    },
                    0x75 => {
                        self.bit(self.reg_l, 6);
                        mcycles = 2;
                    },
                    0x77 => {
                        self.bit(self.reg_a, 6);
                        mcycles = 2;
                    },
                    0x76 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        self.bit(z, 6);
                        mcycles = 3;
                    },
                    0x78 => {
                        self.bit(self.reg_b, 7);
                        mcycles = 2;
                    },
                    0x79 => {
                        self.bit(self.reg_c, 7);
                        mcycles = 2;
                    },
                    0x7A => {
                        self.bit(self.reg_d, 7);
                        mcycles = 2;
                    },
                    0x7B => {
                        self.bit(self.reg_e, 7);
                        mcycles = 2;
                    },
                    0x7C => {
                        self.bit(self.reg_h, 7);
                        mcycles = 2;
                    },
                    0x7D => {
                        self.bit(self.reg_l, 7);
                        mcycles = 2;
                    },
                    0x7F => {
                        self.bit(self.reg_a, 7);
                        mcycles = 2;
                    },
                    0x7E => {
                        let z = self.bus.read(self.get_hl() as usize);
                        self.bit(z, 7);
                        mcycles = 3;
                    },

                    0x80 => {
                        self.reg_b = self.res(self.reg_b, 0);
                        mcycles = 2;
                    },
                    0x81 => {
                        self.reg_c = self.res(self.reg_c, 0);
                        mcycles = 2;
                    },
                    0x82 => {
                        self.reg_d = self.res(self.reg_d, 0);
                        mcycles = 2;
                    },
                    0x83 => {
                        self.reg_e = self.res(self.reg_e, 0);
                        mcycles = 2;
                    },
                    0x84 => {
                        self.reg_h = self.res(self.reg_h, 0);
                        mcycles = 2;
                    },
                    0x85 => {
                        self.reg_l = self.res(self.reg_l, 0);
                        mcycles = 2;
                    },
                    0x87 => {
                        self.reg_a = self.res(self.reg_a, 0);
                        mcycles = 2;
                    },
                    0x86 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.res(z, 0);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0x88 => {
                        self.reg_b = self.res(self.reg_b, 1);
                        mcycles = 2;
                    },
                    0x89 => {
                        self.reg_c = self.res(self.reg_c, 1);
                        mcycles = 2;
                    },
                    0x8A => {
                        self.reg_d = self.res(self.reg_d, 1);
                        mcycles = 2;
                    },
                    0x8B => {
                        self.reg_e = self.res(self.reg_e, 1);
                        mcycles = 2;
                    },
                    0x8C => {
                        self.reg_h = self.res(self.reg_h, 1);
                        mcycles = 2;
                    },
                    0x8D => {
                        self.reg_l = self.res(self.reg_l, 1);
                        mcycles = 2;
                    },
                    0x8F => {
                        self.reg_a = self.res(self.reg_a, 1);
                        mcycles = 2;
                    },
                    0x8E => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.res(z, 1);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0x90 => {
                        self.reg_b = self.res(self.reg_b, 2);
                        mcycles = 2;
                    },
                    0x91 => {
                        self.reg_c = self.res(self.reg_c, 2);
                        mcycles = 2;
                    },
                    0x92 => {
                        self.reg_d = self.res(self.reg_d, 2);
                        mcycles = 2;
                    },
                    0x93 => {
                        self.reg_e = self.res(self.reg_e, 2);
                        mcycles = 2;
                    },
                    0x94 => {
                        self.reg_h = self.res(self.reg_h, 2);
                        mcycles = 2;
                    },
                    0x95 => {
                        self.reg_l = self.res(self.reg_l, 2);
                        mcycles = 2;
                    },
                    0x97 => {
                        self.reg_a = self.res(self.reg_a, 2);
                        mcycles = 2;
                    },
                    0x96 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.res(z, 2);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0x98 => {
                        self.reg_b = self.res(self.reg_b, 3);
                        mcycles = 2;
                    },
                    0x99 => {
                        self.reg_c = self.res(self.reg_c, 3);
                        mcycles = 2;
                    },
                    0x9A => {
                        self.reg_d = self.res(self.reg_d, 3);
                        mcycles = 2;
                    },
                    0x9B => {
                        self.reg_e = self.res(self.reg_e, 3);
                        mcycles = 2;
                    },
                    0x9C => {
                        self.reg_h = self.res(self.reg_h, 3);
                        mcycles = 2;
                    },
                    0x9D => {
                        self.reg_l = self.res(self.reg_l, 3);
                        mcycles = 2;
                    },
                    0x9F => {
                        self.reg_a = self.res(self.reg_a, 3);
                        mcycles = 2;
                    },
                    0x9E => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.res(z, 3);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0xA0 => {
                        self.reg_b = self.res(self.reg_b, 4);
                        mcycles = 2;
                    },
                    0xA1 => {
                        self.reg_c = self.res(self.reg_c, 4);
                        mcycles = 2;
                    },
                    0xA2 => {
                        self.reg_d = self.res(self.reg_d, 4);
                        mcycles = 2;
                    },
                    0xA3 => {
                        self.reg_e = self.res(self.reg_e, 4);
                        mcycles = 2;
                    },
                    0xA4 => {
                        self.reg_h = self.res(self.reg_h, 4);
                        mcycles = 2;
                    },
                    0xA5 => {
                        self.reg_l = self.res(self.reg_l, 4);
                        mcycles = 2;
                    },
                    0xA7 => {
                        self.reg_a = self.res(self.reg_a, 4);
                        mcycles = 2;
                    },
                    0xA6 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.res(z, 4);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0xA8 => {
                        self.reg_b = self.res(self.reg_b, 5);
                        mcycles = 2;
                    },
                    0xA9 => {
                        self.reg_c = self.res(self.reg_c, 5);
                        mcycles = 2;
                    },
                    0xAA => {
                        self.reg_d = self.res(self.reg_d, 5);
                        mcycles = 2;
                    },
                    0xAB => {
                        self.reg_e = self.res(self.reg_e, 5);
                        mcycles = 2;
                    },
                    0xAC => {
                        self.reg_h = self.res(self.reg_h, 5);
                        mcycles = 2;
                    },
                    0xAD => {
                        self.reg_l = self.res(self.reg_l, 5);
                        mcycles = 2;
                    },
                    0xAF => {
                        self.reg_a = self.res(self.reg_a, 5);
                        mcycles = 2;
                    },
                    0xAE => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.res(z, 5);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0xB0 => {
                        self.reg_b = self.res(self.reg_b, 6);
                        mcycles = 2;
                    },
                    0xB1 => {
                        self.reg_c = self.res(self.reg_c, 6);
                        mcycles = 2;
                    },
                    0xB2 => {
                        self.reg_d = self.res(self.reg_d, 6);
                        mcycles = 2;
                    },
                    0xB3 => {
                        self.reg_e = self.res(self.reg_e, 6);
                        mcycles = 2;
                    },
                    0xB4 => {
                        self.reg_h = self.res(self.reg_h, 6);
                        mcycles = 2;
                    },
                    0xB5 => {
                        self.reg_l = self.res(self.reg_l, 6);
                        mcycles = 2;
                    },
                    0xB7 => {
                        self.reg_a = self.res(self.reg_a, 6);
                        mcycles = 2;
                    },
                    0xB6 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.res(z, 6);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0xB8 => {
                        self.reg_b = self.res(self.reg_b, 7);
                        mcycles = 2;
                    },
                    0xB9 => {
                        self.reg_c = self.res(self.reg_c, 7);
                        mcycles = 2;
                    },
                    0xBA => {
                        self.reg_d = self.res(self.reg_d, 7);
                        mcycles = 2;
                    },
                    0xBB => {
                        self.reg_e = self.res(self.reg_e, 7);
                        mcycles = 2;
                    },
                    0xBC => {
                        self.reg_h = self.res(self.reg_h, 7);
                        mcycles = 2;
                    },
                    0xBD => {
                        self.reg_l = self.res(self.reg_l, 7);
                        mcycles = 2;
                    },
                    0xBF => {
                        self.reg_a = self.res(self.reg_a, 7);
                        mcycles = 2;
                    },
                    0xBE => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.res(z, 7);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },

                    0xC0 => {
                        self.reg_b = self.set(self.reg_b, 0);
                        mcycles = 2;
                    },
                    0xC1 => {
                        self.reg_c = self.set(self.reg_c, 0);
                        mcycles = 2;
                    },
                    0xC2 => {
                        self.reg_d = self.set(self.reg_d, 0);
                        mcycles = 2;
                    },
                    0xC3 => {
                        self.reg_e = self.set(self.reg_e, 0);
                        mcycles = 2;
                    },
                    0xC4 => {
                        self.reg_h = self.set(self.reg_h, 0);
                        mcycles = 2;
                    },
                    0xC5 => {
                        self.reg_l = self.set(self.reg_l, 0);
                        mcycles = 2;
                    },
                    0xC7 => {
                        self.reg_a = self.set(self.reg_a, 0);
                        mcycles = 2;
                    },
                    0xC6 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.set(z, 0);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0xC8 => {
                        self.reg_b = self.set(self.reg_b, 1);
                        mcycles = 2;
                    },
                    0xC9 => {
                        self.reg_c = self.set(self.reg_c, 1);
                        mcycles = 2;
                    },
                    0xCA => {
                        self.reg_d = self.set(self.reg_d, 1);
                        mcycles = 2;
                    },
                    0xCB => {
                        self.reg_e = self.set(self.reg_e, 1);
                        mcycles = 2;
                    },
                    0xCC => {
                        self.reg_h = self.set(self.reg_h, 1);
                        mcycles = 2;
                    },
                    0xCD => {
                        self.reg_l = self.set(self.reg_l, 1);
                        mcycles = 2;
                    },
                    0xCF => {
                        self.reg_a = self.set(self.reg_a, 1);
                        mcycles = 2;
                    },
                    0xCE => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.set(z, 1);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0xD0 => {
                        self.reg_b = self.set(self.reg_b, 2);
                        mcycles = 2;
                    },
                    0xD1 => {
                        self.reg_c = self.set(self.reg_c, 2);
                        mcycles = 2;
                    },
                    0xD2 => {
                        self.reg_d = self.set(self.reg_d, 2);
                        mcycles = 2;
                    },
                    0xD3 => {
                        self.reg_e = self.set(self.reg_e, 2);
                        mcycles = 2;
                    },
                    0xD4 => {
                        self.reg_h = self.set(self.reg_h, 2);
                        mcycles = 2;
                    },
                    0xD5 => {
                        self.reg_l = self.set(self.reg_l, 2);
                        mcycles = 2;
                    },
                    0xD7 => {
                        self.reg_a = self.set(self.reg_a, 2);
                        mcycles = 2;
                    },
                    0xD6 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.set(z, 2);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0xD8 => {
                        self.reg_b = self.set(self.reg_b, 3);
                        mcycles = 2;
                    },
                    0xD9 => {
                        self.reg_c = self.set(self.reg_c, 3);
                        mcycles = 2;
                    },
                    0xDA => {
                        self.reg_d = self.set(self.reg_d, 3);
                        mcycles = 2;
                    },
                    0xDB => {
                        self.reg_e = self.set(self.reg_e, 3);
                        mcycles = 2;
                    },
                    0xDC => {
                        self.reg_h = self.set(self.reg_h, 3);
                        mcycles = 2;
                    },
                    0xDD => {
                        self.reg_l = self.set(self.reg_l, 3);
                        mcycles = 2;
                    },
                    0xDF => {
                        self.reg_a = self.set(self.reg_a, 3);
                        mcycles = 2;
                    },
                    0xDE => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.set(z, 3);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0xE0 => {
                        self.reg_b = self.set(self.reg_b, 4);
                        mcycles = 2;
                    },
                    0xE1 => {
                        self.reg_c = self.set(self.reg_c, 4);
                        mcycles = 2;
                    },
                    0xE2 => {
                        self.reg_d = self.set(self.reg_d, 4);
                        mcycles = 2;
                    },
                    0xE3 => {
                        self.reg_e = self.set(self.reg_e, 4);
                        mcycles = 2;
                    },
                    0xE4 => {
                        self.reg_h = self.set(self.reg_h, 4);
                        mcycles = 2;
                    },
                    0xE5 => {
                        self.reg_l = self.set(self.reg_l, 4);
                        mcycles = 2;
                    },
                    0xE7 => {
                        self.reg_a = self.set(self.reg_a, 4);
                        mcycles = 2;
                    },
                    0xE6 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.set(z, 4);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0xE8 => {
                        self.reg_b = self.set(self.reg_b, 5);
                        mcycles = 2;
                    },
                    0xE9 => {
                        self.reg_c = self.set(self.reg_c, 5);
                        mcycles = 2;
                    },
                    0xEA => {
                        self.reg_d = self.set(self.reg_d, 5);
                        mcycles = 2;
                    },
                    0xEB => {
                        self.reg_e = self.set(self.reg_e, 5);
                        mcycles = 2;
                    },
                    0xEC => {
                        self.reg_h = self.set(self.reg_h, 5);
                        mcycles = 2;
                    },
                    0xED => {
                        self.reg_l = self.set(self.reg_l, 5);
                        mcycles = 2;
                    },
                    0xEF => {
                        self.reg_a = self.set(self.reg_a, 5);
                        mcycles = 2;
                    },
                    0xEE => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.set(z, 5);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0xF0 => {
                        self.reg_b = self.set(self.reg_b, 6);
                        mcycles = 2;
                    },
                    0xF1 => {
                        self.reg_c = self.set(self.reg_c, 6);
                        mcycles = 2;
                    },
                    0xF2 => {
                        self.reg_d = self.set(self.reg_d, 6);
                        mcycles = 2;
                    },
                    0xF3 => {
                        self.reg_e = self.set(self.reg_e, 6);
                        mcycles = 2;
                    },
                    0xF4 => {
                        self.reg_h = self.set(self.reg_h, 6);
                        mcycles = 2;
                    },
                    0xF5 => {
                        self.reg_l = self.set(self.reg_l, 6);
                        mcycles = 2;
                    },
                    0xF7 => {
                        self.reg_a = self.set(self.reg_a, 6);
                        mcycles = 2;
                    },
                    0xF6 => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.set(z, 6);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },
                    0xF8 => {
                        self.reg_b = self.set(self.reg_b, 7);
                        mcycles = 2;
                    },
                    0xF9 => {
                        self.reg_c = self.set(self.reg_c, 7);
                        mcycles = 2;
                    },
                    0xFA => {
                        self.reg_d = self.set(self.reg_d, 7);
                        mcycles = 2;
                    },
                    0xFB => {
                        self.reg_e = self.set(self.reg_e, 7);
                        mcycles = 2;
                    },
                    0xFC => {
                        self.reg_h = self.set(self.reg_h, 7);
                        mcycles = 2;
                    },
                    0xFD => {
                        self.reg_l = self.set(self.reg_l, 7);
                        mcycles = 2;
                    },
                    0xFF => {
                        self.reg_a = self.set(self.reg_a, 7);
                        mcycles = 2;
                    },
                    0xFE => {
                        let z = self.bus.read(self.get_hl() as usize);
                        let result = self.set(z, 7);
                        self.bus.write(self.get_hl() as usize, result);
                        mcycles = 4;
                    },

                }
            },

            0xC3 =>{
                let low = self.bus.read(self.pc as usize);
                let high = self.bus.read((self.pc + 1) as usize);
                self.pc = ((high as u16) << 8) | (low as u16);
                mcycles = 4;
            },
            0xE9 =>{
                self.pc = self.get_hl();
                mcycles = 1;
            },
            0xC2 =>{
                let low = self.bus.read(self.pc as usize);
                self.pc += 1;
                let high = self.bus.read(self.pc as usize);
                self.pc += 1;
                if !self.get_zero() {
                    self.pc = ((high as u16) << 8) | (low as u16);
                    mcycles = 4;
                } else {
                    mcycles = 3;
                }
            },
            0xCA => {
                let low = self.bus.read(self.pc as usize);
                self.pc += 1;
                let high = self.bus.read(self.pc as usize);
                self.pc += 1;
                if self.get_zero() {
                    self.pc = ((high as u16) << 8) | (low as u16);
                    mcycles = 4;
                } else {
                    mcycles = 3;
                }
            },
            0xD2 => {
                let low = self.bus.read(self.pc as usize);
                self.pc += 1;
                let high = self.bus.read(self.pc as usize);
                self.pc += 1;
                if !self.get_carry() {
                    self.pc = ((high as u16) << 8) | (low as u16);
                    mcycles = 4;
                } else {
                    mcycles = 3;
                }
            },
            0xDA => {
                let low = self.bus.read(self.pc as usize);
                self.pc += 1;
                let high = self.bus.read(self.pc as usize);
                self.pc += 1;
                if self.get_carry() {
                    self.pc = ((high as u16) << 8) | (low as u16);
                    mcycles = 4;
                } else {
                    mcycles = 3;
                }
            },

            0x18 => {
                let offset = self.bus.read(self.pc as usize) as i8;
                self.pc += 1;
                self.pc = self.pc.wrapping_add(offset as u16);
                mcycles = 3;
            },

            0x20 => {
                let offset = self.bus.read(self.pc as usize) as i8;
                self.pc += 1;
                if !self.get_zero() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    mcycles = 3;
                } else {
                    mcycles = 2;
                }
            },
            0x28 => {
                let offset = self.bus.read(self.pc as usize) as i8;
                self.pc += 1;
                if self.get_zero() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    mcycles = 3;
                } else {
                    mcycles = 2;
                }
            },
            0x30 => {
                let offset = self.bus.read(self.pc as usize) as i8;
                self.pc += 1;
                if !self.get_carry() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    mcycles = 3;
                } else {
                    mcycles = 2;
                }
            },
            0x38 => {
                let offset = self.bus.read(self.pc as usize) as i8;
                self.pc += 1;
                if self.get_carry() {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    mcycles = 3;
                } else {
                    mcycles = 2;
                }
            },

            0xCD => {
                let low = self.bus.read(self.pc as usize);
                self.pc += 1;
                let high = self.bus.read(self.pc as usize);
                self.pc += 1;
                self.push_stack(self.pc);
                self.pc = ((high as u16) << 8) | (low as u16);
                mcycles = 6;
            },

            0xC4 => {
                let low = self.bus.read(self.pc as usize);
                self.pc += 1;
                let high = self.bus.read(self.pc as usize);
                self.pc += 1;
                if !self.get_zero() {
                    self.push_stack(self.pc);
                    self.pc = ((high as u16) << 8) | (low as u16);
                    mcycles = 6;
                } else {
                    mcycles = 3;
                }
            },
            0xCC => {
                let low = self.bus.read(self.pc as usize);
                self.pc += 1;
                let high = self.bus.read(self.pc as usize);
                self.pc += 1;
                if self.get_zero() {
                    self.push_stack(self.pc);
                    self.pc = ((high as u16) << 8) | (low as u16);
                    mcycles = 6;
                } else {
                    mcycles = 3;
                }
            },
            0xD4 => {
                let low = self.bus.read(self.pc as usize);
                self.pc += 1;
                let high = self.bus.read(self.pc as usize);
                self.pc += 1;
                if !self.get_carry() {
                    self.push_stack(self.pc);
                    self.pc = ((high as u16) << 8) | (low as u16);
                    mcycles = 6;
                } else {
                    mcycles = 3;
                }
            },
            0xDC => {
                let low = self.bus.read(self.pc as usize);
                self.pc += 1;
                let high = self.bus.read(self.pc as usize);
                self.pc += 1;
                if self.get_carry() {
                    self.push_stack(self.pc);
                    self.pc = ((high as u16) << 8) | (low as u16);
                    mcycles = 6;
                } else {
                    mcycles = 3;
                }
            },

            0xC9 => {
                self.pc = self.pop_stack();
                mcycles = 4;
            },
            0xC0 => {
                if !self.get_zero() {
                    self.pc = self.pop_stack();
                    mcycles = 5;
                } else {
                    mcycles = 2;
                }
            },
            0xC8 => {
                if self.get_zero() {
                    self.pc = self.pop_stack();
                    mcycles = 5;
                } else {
                    mcycles = 2;
                }
            },
            0xD0 => {
                if !self.get_carry() {
                    self.pc = self.pop_stack();
                    mcycles = 5;
                } else {
                    mcycles = 2;
                }
            },
            0xD8 => {
                if self.get_carry() {
                    self.pc = self.pop_stack();
                    mcycles = 5;
                } else {
                    mcycles = 2;
                }
            },
            0xD9 => {
                // haven't done anything for interrupts yet so temporary code
                self.ime = true;
                self.pc = self.pop_stack();
                mcycles = 4;
            },
            0xDF => {
                self.push_stack(self.pc);
                self.pc = 0x18;
                mcycles = 4;
            },
            0x00 => {
                mcycles = 1;
            },
            // i'll add halt stop and ei later after i figure out how to handle them
            _ => ()
        }
        //self.pc += 1;
        return mcycles
    }

}
