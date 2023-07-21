use crate::cpu::CPU;

mod cpu;
mod parser;

fn main() {
    let input = include_str!("./data/input.txt");
    let (leftover, (directives, instructions)) = parser::parse(input).unwrap();
    assert!(leftover.is_empty());

    let mut cpu = CPU::new();
    cpu.load_program(directives.ip_register, instructions);

    while cpu.tick() {
        let registers = cpu.get_registers();
        // print!("ip={:2} [{:8}, {:8}, {:8}, {:8}, {:8}, {:8}]\r", cpu.get_ip(), registers[0], registers[1], registers[2], registers[3], registers[4], registers[5]);
        if cpu.get_ip() == 28 {
            print!("ip={:2} [{:8}, {:8}, {:8}, {:8}, {:8}, {:8}]\r", cpu.get_ip(), registers[0], registers[1], registers[2], registers[3], registers[4], registers[5]);
            break;
        }
    }
    println!();
}
