use std::process;

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

fn main() {
    let mut stack: Vec<i32> = Vec::with_capacity(1024);

    let program: Vec<u8> = vec![11, 2, 72, 105];

    let mut vars: [i32; 256] = [0; 256];

    let mut pc: usize = 0;

    while pc < program.len() {
        let raw_byte: u8 = program[pc];

        match Opcode::from_u8(raw_byte) {
            Some(Opcode::Push8) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 2) {
                    let bytes: [u8; 1] = byte_slice.try_into().unwrap();
                    let value = i8::from_le_bytes(bytes);

                    stack.push(value as i32);
                    pc += 2;
                } else {
                    print_err("malformed argument for PUSH", pc);
                }
            }
            Some(Opcode::Push32) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let value = i32::from_le_bytes(bytes);

                    stack.push(value);
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
                if stack.len() == 0 {
                    print_err("stack underflow", pc);
                }
                println!("{}", stack[stack.len() - 1]);
                pc += 1;
            }
            Some(Opcode::Add) => {
                if stack.len() >= 2 {
                    let mut a: i32 = pop(&mut stack, pc);
                    let mut b: i32 = pop(&mut stack, pc);

                    stack.push(a + b);
                } else {
                    print_err("stack too short for ADD", pc);
                }
                pc += 1;
            }
            Some(Opcode::Sub) => {
                if stack.len() >= 2 {
                    let mut a: i32 = pop(&mut stack, pc);
                    let mut b: i32 = pop(&mut stack, pc);

                    stack.push(b - a);
                } else {
                    print_err("stack too short for SUB", pc);
                }
                pc += 1;
            }
            Some(Opcode::Mul) => {
                if stack.len() >= 2 {
                    let mut a: i32 = pop(&mut stack, pc);
                    let mut b: i32 = pop(&mut stack, pc);

                    stack.push(a * b);
                } else {
                    print_err("stack too short for MUL", pc);
                }
                pc += 1;
            }
            Some(Opcode::Div) => {
                if stack.len() >= 2 {
                    let mut a: i32 = pop(&mut stack, pc);
                    let mut b: i32 = pop(&mut stack, pc);

                    stack.push(b / a);
                } else {
                    print_err("stack too short for DIV", pc);
                }
                pc += 1;
            }
            Some(Opcode::Mod) => {
                if stack.len() >= 2 {
                    let mut a: i32 = pop(&mut stack, pc);
                    let mut b: i32 = pop(&mut stack, pc);

                    stack.push(b % a);
                } else {
                    print_err("stack too short for MOD", pc);
                }
                pc += 1;
            }
            Some(Opcode::Char) => {
                let mut length = program[pc + 1];
                if stack.len() < length.into() {
                    print_err("stack underflow", pc);
                    let offset: usize = length.into();
                    pc += offset + 1;
                    continue;
                }
                let buf = length;
                let mut chars_to_print: Vec<char> = Vec::new();

                while length != 0 {
                    chars_to_print.push((pop(&mut stack, pc) as u8) as char);
                    length -= 1;
                }

                for c in chars_to_print.iter().rev() {
                    print!("{}", c);
                }

                let offset: usize = buf.into();
                pc += offset + 1;
            }
            Some(Opcode::Print8) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 2) {
                    let bytes: [u8; 1] = byte_slice.try_into().unwrap();
                    let length = i8::from_le_bytes(bytes) as usize;

                    if pc + 2 + length <= program.len() {
                        for i in 0..length {
                            print!("{}", (program[pc + 2 + (i as usize)]) as char);
                        }

                        let offset: usize = length as usize;
                        pc += 2 + offset;
                    } else {
                        print_err("unexpected end of file", pc);
                    }
                } else {
                    print_err("missing length argument for PRINT", pc);
                }
            }
            Some(Opcode::Print32) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let length = i32::from_le_bytes(bytes) as usize;

                    if pc + 5 + length <= program.len() {
                        for i in 0..length {
                            print!("{}", (program[pc + 5 + (i as usize)]) as char);
                        }

                        let offset: usize = length as usize;
                        pc += 5 + offset;
                    } else {
                        print_err("unexpected end of file", pc);
                    }
                } else {
                    print_err("missing length argument for PRINT", pc);
                }
            }
            Some(Opcode::Jump32) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    pc = new_pc;
                } else {
                    print_err("missing jumping address for JUMP", pc);
                }
            }
            Some(Opcode::Jez32) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    if stack.len() >= 1 {
                        let mut x: i32 = pop(&mut stack, pc);

                        if x == 0 {
                            pc = new_pc as usize;
                        } else {
                            pc += 5;
                        }
                    } else {
                        print_err("stack too short for JEZ", pc);
                    }
                } else {
                    print_err("missing jumping address for JEZ", pc);
                }
            }
            Some(Opcode::Jeq32) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    if stack.len() >= 1 {
                        let mut a: i32 = pop(&mut stack, pc);
                        let mut b: i32 = pop(&mut stack, pc);

                        if b == a {
                            pc = new_pc as usize;
                        } else {
                            pc += 5;
                        }
                    } else {
                        print_err("stack too short for JEZ", pc);
                    }
                } else {
                    print_err("missing jumping address for JEZ", pc);
                }
            }
            Some(Opcode::Jgt32) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    if stack.len() >= 1 {
                        let mut a: i32 = pop(&mut stack, pc);
                        let mut b: i32 = pop(&mut stack, pc);

                        if b > a {
                            pc = new_pc as usize;
                        } else {
                            pc += 5;
                        }
                    } else {
                        print_err("stack too short for JEZ", pc);
                    }
                } else {
                    print_err("missing jumping address for JEZ", pc);
                }
            }
            Some(Opcode::Jlt32) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    if stack.len() >= 1 {
                        let mut a: i32 = pop(&mut stack, pc);
                        let mut b: i32 = pop(&mut stack, pc);

                        if b < a {
                            pc = new_pc as usize;
                        } else {
                            pc += 5;
                        }
                    } else {
                        print_err("stack too short for JEZ", pc);
                    }
                } else {
                    print_err("missing jumping address for JEZ", pc);
                }
            }
            Some(Opcode::Jge32) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    if stack.len() >= 1 {
                        let mut a: i32 = pop(&mut stack, pc);
                        let mut b: i32 = pop(&mut stack, pc);

                        if b >= a {
                            pc = new_pc as usize;
                        } else {
                            pc += 5;
                        }
                    } else {
                        print_err("stack too short for JEZ", pc);
                    }
                } else {
                    print_err("missing jumping address for JEZ", pc);
                }
            }
            Some(Opcode::Jle32) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    if stack.len() >= 1 {
                        let mut a: i32 = pop(&mut stack, pc);
                        let mut b: i32 = pop(&mut stack, pc);

                        if b <= a {
                            pc = new_pc as usize;
                        } else {
                            pc += 5;
                        }
                    } else {
                        print_err("stack too short for JEZ", pc);
                    }
                } else {
                    print_err("missing jumping address for JEZ", pc);
                }
            }
            Some(Opcode::Store) => {
                if program.len() >= pc + 1 {
                    let loc = program[pc + 1] as usize;
                    vars[loc] = pop(&mut stack, pc);
                    pc += 2;
                } else {
                    print_err("missing argument for STORE", pc);
                }
            }
            Some(Opcode::Load) => {
                if program.len() >= pc + 1 {
                    let loc = program[pc + 1] as usize;
                    stack.push(vars[loc]);
                    pc += 2;
                } else {
                    print_err("missing argument for LOAD", pc);
                }
            }
            None => {
                print_err("unrecognized bytecode", pc);
            }
        }
    }
}
