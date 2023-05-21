use reformation::Reformation;
use std::collections::HashMap;
use std::fmt::Write;

#[repr(u8)]
#[derive(Reformation, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtithmeticsOptcode {
    #[reformation("add")]
    Add = 0x0,
    #[reformation("sub")]
    Sub = 0x1,
    #[reformation("xor")]
    Xor = 0x7,
}

#[repr(u8)]
#[derive(Reformation, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortArithmeticsOptcode {
    #[reformation("addq")]
    Addq = 0x10,
    #[reformation("subq")]
    Subq = 0x11,
}

impl ShortArithmeticsOptcode {
    pub fn as_long(self) -> ArtithmeticsOptcode {
        match self {
            ShortArithmeticsOptcode::Addq => ArtithmeticsOptcode::Add,
            ShortArithmeticsOptcode::Subq => ArtithmeticsOptcode::Sub,
        }
    }
}

#[repr(u8)]
#[derive(Reformation, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemOpt {
    #[reformation("ldq")]
    Ldq = 0x30,
    #[reformation("stq")]
    Stq = 0x31,
}

#[derive(Reformation, Debug, Clone, PartialEq, Eq)]
pub enum Asm {
    #[reformation("{}")]
    Instruction(Instruction),
    #[reformation("{}:")]
    Label(String),
    #[reformation("brq-{ivnz} {label}")]
    Jump {
        label: String,
        ivnz: u8,
    },
    #[reformation("#{}")]
    Comment(String), 
}

#[derive(Reformation, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    #[reformation("{optcode} r{target} <- r{left}, r{right}")]
    Arithmetic {
        optcode: ArtithmeticsOptcode,
        target: u8,
        left: u8,
        right: u8,
    },
    #[reformation("{optcode} r{target} <- r{left}, {right_value}")]
    ArithmeticQuick {
        optcode: ShortArithmeticsOptcode,
        target: u8,
        left: u8,
        right_value: i8,
    },
    #[reformation("{optcode} r{value_reg} (<-|->) r{addr}, {offset}")]
    Mem {
        optcode: MemOpt,
        value_reg: u8,
        addr: u8,
        offset: i8,
    },
    /// Short relative jump (Brq-invz)
    #[reformation("brq-raw-{ivnz} {offset}")]
    Jump {
        /// only 4 lower bit matter
        ivnz: u8,
        offset: i8, 
    },
    Noop,
}


pub fn compile_asm(asm: &[Asm]) -> Vec<Instruction> {
    let mut result = vec![Instruction::Noop; asm.len()];
    let mut labels = HashMap::new();
    let mut offset = 0;
    for op in asm.iter() {
        match op {
            Asm::Instruction(ins) => {
                result[offset] = *ins;
                offset += 1;
            },
            Asm::Label(name) => {
                labels.insert(name.clone(), offset as i32);
            },
            Asm::Jump { .. } => {
                offset += 1;
            },
            _ => {},
        }
    }
    offset = 0;
    for op in asm.iter() {
        match op {
            Asm::Instruction(_) => {
                offset += 1;
            },
            Asm::Jump { ivnz, label } => {
                result[offset] = Instruction::Jump { 
                    ivnz: *ivnz,
                    offset: (labels.get(label).unwrap() - offset as i32).try_into().unwrap(),
                };
                offset += 1;
            },
            _ => {},
        }
    }
    while result.last() == Some(&Instruction::Noop) {
        result.pop();
    }
    result
}

pub fn compile_opt(ins: &[Instruction]) -> String {
    let mut result = String::new();
    for (i, instruction) in ins.iter().enumerate() {
        let word = compile_instruction( *instruction);
        let upper = word >> 16;
        let lower = word & 0xffff;
        write!(&mut result, "{i}=>X\"{upper:04X}_{lower:04X}\",\n").unwrap();
    }
    return result;
}

fn compile_instruction(ins: Instruction) -> u32 {
    match ins {
        Instruction::Arithmetic { optcode, target, left, right } => {
            ((optcode as u32) << 24)
            | ((target as u32) << 16)
            | ((left as u32) << 8)
            | (right as u32)
        },
        Instruction::ArithmeticQuick { optcode, target, left, right_value } => {
            ((optcode as u32) << 24)
            | ((target as u32) << 16)
            | ((left as u32) << 8)
            | ((right_value as u8) as u32)
        },
        Instruction::Jump { ivnz, offset } => {
            (0x51 << 24)
            | ((ivnz as u32) << 16)
            | ((offset as u8) as u32)
        },
        Instruction::Mem { optcode, value_reg, addr, offset } => {
            ((optcode as u32) << 24)
            | ((value_reg as u32) << 16)
            | ((addr as u32) << 8)
            | ((offset as u8) as u32)
        },
        Instruction::Noop => {panic!()},
    }
}
