mod cpu;

fn main(){
    let mut c = cpu::CPU::new();
    println!("CPU: {:?}", c);
    c.run_opcode(0x00);
    println!("weeee wooo");
}