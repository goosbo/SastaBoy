#[derive(Debug)]
pub struct Mem{
    memory: [u8; 0xFFFF]
}

impl Mem{
    pub fn new() -> Self{
        Mem{
            memory: [0; 0xFFFF]
        }
        
    }

    pub fn read(&self, addr: usize) -> u8{
        self.memory[addr]
    }

    pub fn write(&mut self, addr: usize, val: u8){
        self.memory[addr] = val;
    }
}

