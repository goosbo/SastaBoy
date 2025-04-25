mod cpu;

fn main(){
    let mut c = cpu::CPU::new();
    c.run_opcode(0);
    println!("CPU: {:?}", c);
    println!("weeee wooo");
}