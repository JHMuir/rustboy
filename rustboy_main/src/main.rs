// mod rustboy_lib;

use rustboy_lib::cpu::CPU;

fn main() {
    let cpu = CPU::new();
    dbg!(cpu);
    println!("Hello, world!");
}
