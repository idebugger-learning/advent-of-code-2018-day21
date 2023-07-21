pub type Registers = [isize; 6];
pub type Instruction = (Opcode, isize, isize, isize);
pub struct CPU {
    registers: Registers,
    ip_register: usize,
    ip: usize,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0, 0, 0, 0, 0, 0],
            ip_register: 0,
            ip: 0,
            instructions: Vec::new(),
        }
    }

    pub fn get_ip(&self) -> usize {
        self.ip
    }

    pub fn get_registers(&self) -> Registers {
        self.registers
    }

    pub fn load_program(&mut self, ip_register: usize, instructions: Vec<Instruction>) {
        self.ip_register = ip_register;
        self.instructions = instructions;
    }

    pub fn tick(&mut self) -> bool {
        if self.ip >= self.instructions.len() {
            return false;
        }

        let (opcode, a, b, c) = self.instructions[self.ip];
        self.do_opcode(opcode, a, b, c);

        true
    }

    fn do_opcode(&mut self, opcode: Opcode, a: isize, b: isize, c: isize) {
        match opcode {
            Opcode::Addr => self.do_addr(a as usize, b as usize, c as usize),
            Opcode::Addi => self.do_addi(a as usize, b, c as usize),
            Opcode::Mulr => self.do_mulr(a as usize, b as usize, c as usize),
            Opcode::Muli => self.do_muli(a as usize, b, c as usize),
            Opcode::Banr => self.do_banr(a as usize, b as usize, c as usize),
            Opcode::Bani => self.do_bani(a as usize, b, c as usize),
            Opcode::Borr => self.do_borr(a as usize, b as usize, c as usize),
            Opcode::Bori => self.do_bori(a as usize, b, c as usize),
            Opcode::Setr => self.do_setr(a as usize, c as usize),
            Opcode::Seti => self.do_seti(a, c as usize),
            Opcode::Gtir => self.do_gtir(a, b as usize, c as usize),
            Opcode::Gtri => self.do_gtri(a as usize, b, c as usize),
            Opcode::Gtrr => self.do_gtrr(a as usize, b as usize, c as usize),
            Opcode::Eqir => self.do_eqir(a, b as usize, c as usize),
            Opcode::Eqri => self.do_eqri(a as usize, b, c as usize),
            Opcode::Eqrr => self.do_eqrr(a as usize, b as usize, c as usize),
        }

        self.ip = self.registers[self.ip_register] as usize;
        self.ip += 1;
        self.registers[self.ip_register] = self.ip as isize;
    }

    fn do_addr(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] + self.registers[reg_b];
    }

    fn do_addi(&mut self, reg_a: usize, val_b: isize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] + val_b;
    }

    fn do_mulr(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] * self.registers[reg_b];
    }

    fn do_muli(&mut self, reg_a: usize, val_b: isize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] * val_b;
    }

    fn do_banr(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] & self.registers[reg_b];
    }

    fn do_bani(&mut self, reg_a: usize, val_b: isize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] & val_b;
    }

    fn do_borr(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] | self.registers[reg_b];
    }

    fn do_bori(&mut self, reg_a: usize, val_b: isize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a] | val_b;
    }

    fn do_setr(&mut self, reg_a: usize, reg_c: usize) {
        self.registers[reg_c] = self.registers[reg_a];
    }

    fn do_seti(&mut self, val_a: isize, reg_c: usize) {
        self.registers[reg_c] = val_a;
    }

    fn do_gtir(&mut self, val_a: isize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = if val_a > self.registers[reg_b] { 1 } else { 0 };
    }

    fn do_gtri(&mut self, reg_a: usize, val_b: isize, reg_c: usize) {
        self.registers[reg_c] = if self.registers[reg_a] > val_b { 1 } else { 0 };
    }

    fn do_gtrr(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = if self.registers[reg_a] > self.registers[reg_b] { 1 } else { 0 };
    }

    fn do_eqir(&mut self, val_a: isize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = if val_a == self.registers[reg_b] { 1 } else { 0 };
    }

    fn do_eqri(&mut self, reg_a: usize, val_b: isize, reg_c: usize) {
        self.registers[reg_c] = if self.registers[reg_a] == val_b { 1 } else { 0 };
    }

    fn do_eqrr(&mut self, reg_a: usize, reg_b: usize, reg_c: usize) {
        self.registers[reg_c] = if self.registers[reg_a] == self.registers[reg_b] { 1 } else { 0 };
    }
}
