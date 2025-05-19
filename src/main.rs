mod cpu;
mod interrupt;
mod memory;
mod timer;
mod gba;


use gba::GBA;
fn main(){
    // components are their own thing now :D
    let gba = GBA::new();
    gba.run();
    println!("Emulator: {:?}", gba);
    println!("weeee wooo");
}