use crate::asm::{
    Instruction,
    ArtithmeticsOptcode,
    ShortArithmeticsOptcode,
    MemOpt,
};

pub struct Interpreter {
    code: Vec<Instruction>,
    pub memory: Vec<i32>,
    pub rx: [i32; 256],
    code_pointer: u32,
    flags: u8,
}


impl Interpreter {
    pub fn new(code: Vec<Instruction>, mem_size: usize) -> Self {
        Self {
            code,
            memory: vec![0; mem_size],
            rx: [0; 256],
            code_pointer: 0,
            flags: 0,
        }
    }

    pub fn run(&mut self) {
        while (self.code_pointer as usize) < self.code.len() {
            self.step();
        }
    }

    fn step(&mut self) {
        match self.code[self.code_pointer as usize] {
            Instruction::Arithmetic { optcode, target, left, right } => {
                self.rx[target as usize] = self.arithmetics(
                    optcode,
                    self.rx[left as usize],
                    self.rx[right as usize],
                );
                self.code_pointer += 1;
            },
            Instruction::ArithmeticQuick { optcode, target, left, right_value } => {
                self.rx[target as usize] = self.arithmetics(
                    optcode.as_long(),
                    self.rx[left as usize],
                    right_value as i32,
                );
                self.code_pointer += 1;
            },
            Instruction::Jump { ivnz, offset } => {
                if self.should_jump(ivnz) {
                    self.code_pointer = self.code_pointer.wrapping_add(offset as u32);
                } else {
                    self.code_pointer += 1;
                }
            },
            Instruction::Mem { optcode, value_reg, addr, offset } => {
                let addr = self.rx[addr as usize].wrapping_add(offset as i32) as u32;
                match optcode {
                    MemOpt::Ldq => {
                        self.rx[value_reg as usize] = self.memory[addr as usize];
                    }
                    MemOpt::Stq => {
                        self.memory[addr as usize] = self.rx[value_reg as usize];
                    }
                }
                self.code_pointer += 1;
            }
            Instruction::Noop => {
                self.code_pointer += 1;
            }
        }
    }

    fn should_jump(&self, ivnz: u8) -> bool {
        let i = ivnz >> 3;
        let checked = ivnz & self.flags;
        let v = (checked >> 2) & 1;
        let n = (checked >> 1) & 1;
        let z = checked & 1;
        i == n | v | z
    }

    fn clear_flags(&mut self) {
        self.flags = 0;
    }

    fn set_overflow(&mut self) {
        self.flags |= 0x4;
    }

    fn set_negative(&mut self) {
        self.flags |= 0x2;
    }

    fn set_zero(&mut self) {
        self.flags |= 0x1;
    }

    fn arithmetics(&mut self, op: ArtithmeticsOptcode, left: i32, right: i32) -> i32 {
        self.clear_flags();
        match op {
            ArtithmeticsOptcode::Add => {
                let (result, overflow) = left.carrying_add(right, false);
                if overflow {
                    self.set_overflow();
                }
                if result == 0 {
                    self.set_zero();
                }
                if result < 0 {
                    self.set_negative();
                }
                result
            },
            ArtithmeticsOptcode::Sub => {
                let result = left.checked_sub(right);
                if result.is_none() {
                    self.set_overflow();
                }
                let result = left.wrapping_sub(right);
                if result == 0 {
                    self.set_zero();
                }
                if result < 0 {
                    self.set_negative();
                }
                result
            },
            ArtithmeticsOptcode::Xor => {
                let result = left ^ right;
                if result < 0 {
                    self.set_negative();
                }
                if result == 0 {
                    self.set_zero();
                }
                result
            }
        }
    }
}