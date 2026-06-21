use std::process;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Push8 = 1,
    Push32 = 2,
    Pop = 3,
    Top = 4,
    Add = 5,
    Sub = 6,
    Mul = 7,
    Div = 8,
    Mod = 9,
    Char = 10,
    Print8 = 11,
    Print32 = 12,
    Jump8 = 13,
    Jez8 = 14,
    Jeq8 = 15,
    Jgt8 = 16,
    Jlt8 = 17,
    Jge8 = 18,
    Jle8 = 19,
    Jump32 = 20,
    Jez32 = 21,
    Jeq32 = 22,
    Jgt32 = 23,
    Jlt32 = 24,
    Jge32 = 25,
    Jle32 = 26,
    Store = 27,
    Load = 28,
    End = 29,
    PushConst0 = 30,
    PushConst1 = 31,
    PushConst2 = 32,
    PushConst3 = 33,
    PushConst4 = 34,
    PushConst5 = 35,
    PushConst6 = 36,
    PushConst7 = 37,
    PushConst8 = 38,
    PushConst9 = 39,
    Inc = 40,
    Dec = 41,
}

impl Opcode {
    fn from_u8(byte: u8) -> Option<Opcode> {
        match byte {
            1 => Some(Opcode::Push8),
            2 => Some(Opcode::Push32),
            3 => Some(Opcode::Pop),
            4 => Some(Opcode::Top),
            5 => Some(Opcode::Add),
            6 => Some(Opcode::Sub),
            7 => Some(Opcode::Mul),
            8 => Some(Opcode::Div),
            9 => Some(Opcode::Mod),
            10 => Some(Opcode::Char),
            11 => Some(Opcode::Print8),
            12 => Some(Opcode::Print32),
            13 => Some(Opcode::Jump8),
            14 => Some(Opcode::Jez8),
            15 => Some(Opcode::Jeq8),
            16 => Some(Opcode::Jgt8),
            17 => Some(Opcode::Jlt8),
            18 => Some(Opcode::Jge8),
            19 => Some(Opcode::Jle8),
            20 => Some(Opcode::Jump32),
            21 => Some(Opcode::Jez32),
            22 => Some(Opcode::Jeq32),
            23 => Some(Opcode::Jgt32),
            24 => Some(Opcode::Jlt32),
            25 => Some(Opcode::Jge32),
            26 => Some(Opcode::Jle32),
            27 => Some(Opcode::Store),
            28 => Some(Opcode::Load),
            29 => Some(Opcode::End),
            30 => Some(Opcode::PushConst0),
            31 => Some(Opcode::PushConst1),
            32 => Some(Opcode::PushConst2),
            33 => Some(Opcode::PushConst3),
            34 => Some(Opcode::PushConst4),
            35 => Some(Opcode::PushConst5),
            36 => Some(Opcode::PushConst6),
            37 => Some(Opcode::PushConst7),
            38 => Some(Opcode::PushConst8),
            39 => Some(Opcode::PushConst9),
            40 => Some(Opcode::Inc),
            41 => Some(Opcode::Dec),
            _ => None,
        }
    }
}

fn print_err(error_msg: &str, pc: usize) {
    println!("\n--- halt execution! ---");
    println!("cola: error at pc = {}", pc);
    println!("{}", error_msg);
    println!("aborting...");
    process::exit(0x01);
}

fn pop(stack: &mut Vec<i32>, pc: usize) -> i32 {
    match stack.pop() {
        Some(x) => x,
        None => {
            print_err("unexpected stack underflow", pc);
            0
        }
    }
}

pub fn execute_program(program: Vec<u8>) {
    let mut stack: Vec<i32> = Vec::with_capacity(1024);
    let mut vars: [i32; 256] = [0; 256];
    let mut pc: usize = 0;

    while pc < program.len() {
        let raw_byte: u8 = program[pc];

        match Opcode::from_u8(raw_byte) {
            Some(Opcode::Push8) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 2) {
                    let bytes: [u8; 1] = byte_slice.try_into().unwrap();
                    stack.push(i8::from_le_bytes(bytes) as i32);
                    pc += 2;
                } else {
                    print_err("malformed argument for PUSH", pc);
                }
            }
            Some(Opcode::Push32) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    stack.push(i32::from_le_bytes(bytes));
                    pc += 5;
                } else {
                    print_err("malformed argument for PUSH", pc);
                }
            }
            Some(Opcode::Pop) => {
                stack.pop();
                pc += 1;
            }
            Some(Opcode::Top) => {
                if stack.is_empty() {
                    print_err("stack underflow", pc);
                }
                print!("{}", stack[stack.len() - 1]);
                pc += 1;
            }
            Some(Opcode::Add) => {
                if stack.len() >= 2 {
                    let a = pop(&mut stack, pc);
                    let b = pop(&mut stack, pc);
                    stack.push(a + b);
                } else {
                    print_err("stack too short for ADD", pc);
                }
                pc += 1;
            }
            Some(Opcode::Sub) => {
                if stack.len() >= 2 {
                    let a = pop(&mut stack, pc);
                    let b = pop(&mut stack, pc);
                    stack.push(b - a);
                } else {
                    print_err("stack too short for SUB", pc);
                }
                pc += 1;
            }
            Some(Opcode::Mul) => {
                if stack.len() >= 2 {
                    let a = pop(&mut stack, pc);
                    let b = pop(&mut stack, pc);
                    stack.push(a * b);
                } else {
                    print_err("stack too short for MUL", pc);
                }
                pc += 1;
            }
            Some(Opcode::Div) => {
                if stack.len() >= 2 {
                    let a = pop(&mut stack, pc);
                    if a == 0 {
                        print_err("division by zero", pc);
                    }
                    let b = pop(&mut stack, pc);
                    stack.push(b / a);
                } else {
                    print_err("stack too short for DIV", pc);
                }
                pc += 1;
            }
            Some(Opcode::Mod) => {
                if stack.len() >= 2 {
                    let a = pop(&mut stack, pc);
                    if a == 0 {
                        print_err("division by zero", pc);
                    }
                    let b = pop(&mut stack, pc);
                    stack.push(b % a);
                } else {
                    print_err("stack too short for MOD", pc);
                }
                pc += 1;
            }
            Some(Opcode::Char) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 2) {
                    let mut length = byte_slice[0];
                    if stack.len() < length.into() {
                        print_err("stack underflow", pc);
                        pc += (length as usize) + 1;
                        continue;
                    }
                    let buf = length;
                    let mut chars_to_print = Vec::new();
                    while length != 0 {
                        chars_to_print
                            .push(std::char::from_u32(pop(&mut stack, pc) as u32).unwrap_or('?'));
                        length -= 1;
                    }
                    for c in chars_to_print.iter().rev() {
                        print!("{}", c);
                    }
                    pc += (buf as usize) + 1;
                } else {
                    print_err("unexpected end of file", pc);
                }
            }
            Some(Opcode::Print8) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 2) {
                    let length = byte_slice[0] as usize;
                    if pc + 2 + length <= program.len() {
                        for i in 0..length {
                            print!("{}", program[pc + 2 + i] as char);
                        }
                        pc += 2 + length;
                    } else {
                        print_err("unexpected end of file", pc);
                    }
                } else {
                    print_err("missing length argument for PRINT", pc);
                }
            }
            Some(Opcode::Print32) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let length = i32::from_le_bytes(byte_slice.try_into().unwrap()) as usize;
                    if pc + 5 + length <= program.len() {
                        for i in 0..length {
                            print!("{}", program[pc + 5 + i] as char);
                        }
                        pc += 5 + length;
                    } else {
                        print_err("unexpected end of file", pc);
                    }
                } else {
                    print_err("missing length argument for PRINT", pc);
                }
            }
            Some(Opcode::Jump8) => {
                if let Some(b) = program.get(pc + 1) {
                    pc = *b as usize;
                } else {
                    print_err("missing jumping address for JUMP", pc);
                }
            }
            Some(Opcode::Jump32) => {
                if let Some(b) = program.get(pc + 1..pc + 5) {
                    pc = i32::from_le_bytes(b.try_into().unwrap()) as usize;
                } else {
                    print_err("missing jumping address for JUMP", pc);
                }
            }
            Some(Opcode::Jez8) => {
                if let Some(b) = program.get(pc + 1) {
                    if !stack.is_empty() {
                        if pop(&mut stack, pc) == 0 {
                            pc = *b as usize;
                        } else {
                            pc += 2;
                        }
                    } else {
                        print_err("stack too short for JEZ", pc);
                    }
                } else {
                    print_err("missing address for JEZ", pc);
                }
            }
            Some(Opcode::Jez32) => {
                if let Some(b) = program.get(pc + 1..pc + 5) {
                    if !stack.is_empty() {
                        if pop(&mut stack, pc) == 0 {
                            pc = i32::from_le_bytes(b.try_into().unwrap()) as usize;
                        } else {
                            pc += 5;
                        }
                    } else {
                        print_err("stack too short for JEZ", pc);
                    }
                } else {
                    print_err("missing address for JEZ", pc);
                }
            }
            Some(Opcode::Jeq8) => {
                if let Some(b) = program.get(pc + 1) {
                    if stack.len() >= 2 {
                        let a = pop(&mut stack, pc);
                        let b_val = pop(&mut stack, pc);
                        if b_val == a {
                            pc = *b as usize;
                        } else {
                            pc += 2;
                        }
                    } else {
                        print_err("stack too short for JEQ", pc);
                    }
                } else {
                    print_err("missing address for JEQ", pc);
                }
            }
            Some(Opcode::Jeq32) => {
                if let Some(b) = program.get(pc + 1..pc + 5) {
                    if stack.len() >= 2 {
                        let a = pop(&mut stack, pc);
                        let b_val = pop(&mut stack, pc);
                        if b_val == a {
                            pc = i32::from_le_bytes(b.try_into().unwrap()) as usize;
                        } else {
                            pc += 5;
                        }
                    } else {
                        print_err("stack too short for JEQ", pc);
                    }
                } else {
                    print_err("missing address for JEQ", pc);
                }
            }
            Some(Opcode::Jgt8) => {
                if let Some(b) = program.get(pc + 1) {
                    if stack.len() >= 2 {
                        let a = pop(&mut stack, pc);
                        let b_val = pop(&mut stack, pc);
                        if b_val > a {
                            pc = *b as usize;
                        } else {
                            pc += 2;
                        }
                    } else {
                        print_err("stack too short for JGT", pc);
                    }
                } else {
                    print_err("missing address for JGT", pc);
                }
            }
            Some(Opcode::Jgt32) => {
                if let Some(b) = program.get(pc + 1..pc + 5) {
                    if stack.len() >= 2 {
                        let a = pop(&mut stack, pc);
                        let b_val = pop(&mut stack, pc);
                        if b_val > a {
                            pc = i32::from_le_bytes(b.try_into().unwrap()) as usize;
                        } else {
                            pc += 5;
                        }
                    } else {
                        print_err("stack too short for JGT", pc);
                    }
                } else {
                    print_err("missing address for JGT", pc);
                }
            }
            Some(Opcode::Jlt8) => {
                if let Some(b) = program.get(pc + 1) {
                    if stack.len() >= 2 {
                        let a = pop(&mut stack, pc);
                        let b_val = pop(&mut stack, pc);
                        if b_val < a {
                            pc = *b as usize;
                        } else {
                            pc += 2;
                        }
                    } else {
                        print_err("stack too short for JLT", pc);
                    }
                } else {
                    print_err("missing address for JLT", pc);
                }
            }
            Some(Opcode::Jlt32) => {
                if let Some(b) = program.get(pc + 1..pc + 5) {
                    if stack.len() >= 2 {
                        let a = pop(&mut stack, pc);
                        let b_val = pop(&mut stack, pc);
                        if b_val < a {
                            pc = i32::from_le_bytes(b.try_into().unwrap()) as usize;
                        } else {
                            pc += 5;
                        }
                    } else {
                        print_err("stack too short for JLT", pc);
                    }
                } else {
                    print_err("missing address for JLT", pc);
                }
            }
            Some(Opcode::Jge8) => {
                if let Some(b) = program.get(pc + 1) {
                    if stack.len() >= 2 {
                        let a = pop(&mut stack, pc);
                        let b_val = pop(&mut stack, pc);
                        if b_val >= a {
                            pc = *b as usize;
                        } else {
                            pc += 2;
                        }
                    } else {
                        print_err("stack too short for JGE", pc);
                    }
                } else {
                    print_err("missing address for JGE", pc);
                }
            }
            Some(Opcode::Jge32) => {
                if let Some(b) = program.get(pc + 1..pc + 5) {
                    if stack.len() >= 2 {
                        let a = pop(&mut stack, pc);
                        let b_val = pop(&mut stack, pc);
                        if b_val >= a {
                            pc = i32::from_le_bytes(b.try_into().unwrap()) as usize;
                        } else {
                            pc += 5;
                        }
                    } else {
                        print_err("stack too short for JGE", pc);
                    }
                } else {
                    print_err("missing address for JGE", pc);
                }
            }
            Some(Opcode::Jle8) => {
                if let Some(b) = program.get(pc + 1) {
                    if stack.len() >= 2 {
                        let a = pop(&mut stack, pc);
                        let b_val = pop(&mut stack, pc);
                        if b_val <= a {
                            pc = *b as usize;
                        } else {
                            pc += 2;
                        }
                    } else {
                        print_err("stack too short for JLE", pc);
                    }
                } else {
                    print_err("missing address for JLE", pc);
                }
            }
            Some(Opcode::Jle32) => {
                if let Some(b) = program.get(pc + 1..pc + 5) {
                    if stack.len() >= 2 {
                        let a = pop(&mut stack, pc);
                        let b_val = pop(&mut stack, pc);
                        if b_val <= a {
                            pc = i32::from_le_bytes(b.try_into().unwrap()) as usize;
                        } else {
                            pc += 5;
                        }
                    } else {
                        print_err("stack too short for JLE", pc);
                    }
                } else {
                    print_err("missing address for JLE", pc);
                }
            }
            Some(Opcode::Store) => {
                if program.len() >= pc + 1 {
                    vars[program[pc + 1] as usize] = pop(&mut stack, pc);
                    pc += 2;
                } else {
                    print_err("missing argument for STORE", pc);
                }
            }
            Some(Opcode::Load) => {
                if program.len() >= pc + 1 {
                    stack.push(vars[program[pc + 1] as usize]);
                    pc += 2;
                } else {
                    print_err("missing argument for LOAD", pc);
                }
            }
            Some(Opcode::End) => {
                process::exit(0);
            }
            Some(Opcode::PushConst0) => {
                stack.push(0);
                pc += 1;
            }
            Some(Opcode::PushConst1) => {
                stack.push(1);
                pc += 1;
            }
            Some(Opcode::PushConst2) => {
                stack.push(2);
                pc += 1;
            }
            Some(Opcode::PushConst3) => {
                stack.push(3);
                pc += 1;
            }
            Some(Opcode::PushConst4) => {
                stack.push(4);
                pc += 1;
            }
            Some(Opcode::PushConst5) => {
                stack.push(5);
                pc += 1;
            }
            Some(Opcode::PushConst6) => {
                stack.push(6);
                pc += 1;
            }
            Some(Opcode::PushConst7) => {
                stack.push(7);
                pc += 1;
            }
            Some(Opcode::PushConst8) => {
                stack.push(8);
                pc += 1;
            }
            Some(Opcode::PushConst9) => {
                stack.push(9);
                pc += 1;
            }
            Some(Opcode::Inc) => {
                let x = pop(&mut stack, pc);
                stack.push(x + 1);
                pc += 1;
            }
            Some(Opcode::Dec) => {
                let x = pop(&mut stack, pc);
                stack.push(x - 1);
                pc += 1;
            }
            None => {
                print_err("unrecognized bytecode", pc);
            }
        }
    }
}
