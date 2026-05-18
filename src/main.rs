use std::process;

enum Opcode {
    Push = 1,
    Pop = 2,
    Top = 3,
    Add = 4,
    Sub = 5,
    Mul = 6,
    Div = 7,
    Mod = 8,
    Char = 9,
    Print = 10,
    Jump = 11,
    Jez = 12,
    Jeq = 13,
    Jgt = 14,
    Jlt = 15,
    Jge = 16,
    Jle = 17,
    Store = 18,
    Load = 19,
}

impl Opcode {
    fn from_u8(byte: u8) -> Option<Opcode> {
        match byte {
            1 => Some(Opcode::Push),
            2 => Some(Opcode::Pop),
            3 => Some(Opcode::Top),
            4 => Some(Opcode::Add),
            5 => Some(Opcode::Sub),
            6 => Some(Opcode::Mul),
            7 => Some(Opcode::Div),
            8 => Some(Opcode::Mod),
            9 => Some(Opcode::Char),
            10 => Some(Opcode::Print),
            11 => Some(Opcode::Jump),
            12 => Some(Opcode::Jez),
            13 => Some(Opcode::Jeq),
            14 => Some(Opcode::Jgt),
            15 => Some(Opcode::Jlt),
            16 => Some(Opcode::Jge),
            17 => Some(Opcode::Jle),
            18 => Some(Opcode::Store),
            19 => Some(Opcode::Load),
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

fn main() {
    let mut stack: Vec<i32> = Vec::with_capacity(1024);

    let program2: Vec<u8> = vec![
        1, 5, 0, 0, 0, 1, 5, 0, 0, 0, 13, 21, 0, 0, 0, 10, 1, 0, 0, 0, 48, 10, 1, 0, 0, 0, 49,
    ];

    let program: Vec<u8> = vec![
        // Initialize: var[0] = 0 (a), var[1] = 1 (b), var[2] = 5 (counter)
        1, 0, 0, 0, 0, // 0:  PUSH 0
        18, 0, // 5:  STORE var[0]  (a = 0)
        1, 1, 0, 0, 0, // 7:  PUSH 1
        18, 1, // 12: STORE var[1]  (b = 1)
        1, 5, 0, 0, 0, // 14: PUSH 5
        18, 2, // 19: STORE var[2]  (counter = 5)
        // Loop start (pc = 21): if counter == 0, jump to end
        19, 2, // 21: LOAD var[2]
        1, 0, 0, 0, 0, // 23: PUSH 0
        13, 63, 0, 0, 0, // 28: JEQ 62  (if counter == 0, jump to end)
        // Print var[0] (a) using TOP
        19, 0, // 33: LOAD var[0]
        3, // 35: TOP  (prints top of stack)
        2, // 36: POP
        // Compute next: temp = a + b
        19, 0, // 37: LOAD var[0]  (a)
        19, 1, // 39: LOAD var[1]  (b)
        4, // 41: ADD  → a+b on stack
        // a = b
        19, 1, // 42: LOAD var[1]
        18, 0, // 44: STORE var[0]
        // b = a+b (result still on stack from ADD)
        18, 1, // 46: STORE var[1]
        // counter -= 1
        19, 2, // 48: LOAD var[2]
        1, 1, 0, 0, 0, // 50: PUSH 1
        5, // 55: SUB  (counter - 1)
        18, 2, // 56: STORE var[2]
        // Jump back to loop start
        11, 21, 0, 0, 0, // 58: JUMP 21
        // End (pc = 63)
        1, 0, 0, 0, 0, // 63: PUSH 0  (no-op landing pad)
    ];

    let mut vars: [i32; 256] = [0; 256];

    let mut pc: usize = 0;

    while pc < program.len() {
        let raw_byte: u8 = program[pc];

        match Opcode::from_u8(raw_byte) {
            Some(Opcode::Push) => {
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
                    let mut a: i32 = 0;
                    let mut b: i32 = 0;

                    match stack.pop() {
                        Some(x) => a = x,
                        None => {
                            print_err("stack underflow", pc);
                        }
                    }

                    match stack.pop() {
                        Some(x) => b = x,
                        None => {
                            print_err("stack underflow", pc);
                        }
                    }

                    stack.push(a + b);
                } else {
                    print_err("stack too short for ADD", pc);
                }
                pc += 1;
            }
            Some(Opcode::Sub) => {
                if stack.len() >= 2 {
                    let mut a: i32 = 0;
                    let mut b: i32 = 0;

                    match stack.pop() {
                        Some(x) => a = x,
                        None => {
                            print_err("stack underflow", pc);
                        }
                    }

                    match stack.pop() {
                        Some(x) => b = x,
                        None => {
                            print_err("stack underflow", pc);
                        }
                    }

                    stack.push(b - a);
                } else {
                    print_err("stack too short for SUB", pc);
                }
                pc += 1;
            }
            Some(Opcode::Mul) => {
                if stack.len() >= 2 {
                    let mut a: i32 = 0;
                    let mut b: i32 = 0;

                    match stack.pop() {
                        Some(x) => a = x,
                        None => {
                            print_err("stack underflow", pc);
                        }
                    }

                    match stack.pop() {
                        Some(x) => b = x,
                        None => {
                            print_err("stack underflow", pc);
                        }
                    }

                    stack.push(a * b);
                } else {
                    print_err("stack too short for MUL", pc);
                }
                pc += 1;
            }
            Some(Opcode::Div) => {
                if stack.len() >= 2 {
                    let mut a: i32 = 0;
                    let mut b: i32 = 0;

                    match stack.pop() {
                        Some(x) => a = x,
                        None => {
                            print_err("stack underflow", pc);
                        }
                    }

                    match stack.pop() {
                        Some(x) => b = x,
                        None => {
                            print_err("stack underflow", pc);
                        }
                    }

                    stack.push(b / a);
                } else {
                    print_err("stack too short for DIV", pc);
                }
                pc += 1;
            }
            Some(Opcode::Mod) => {
                if stack.len() >= 2 {
                    let mut a: i32 = 0;
                    let mut b: i32 = 0;

                    match stack.pop() {
                        Some(x) => a = x,
                        None => {
                            print_err("stack underflow", pc);
                        }
                    }

                    match stack.pop() {
                        Some(x) => b = x,
                        None => {
                            print_err("stack underflow", pc);
                        }
                    }

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
                    match stack.pop() {
                        Some(x) => chars_to_print.push((x as u8) as char),
                        None => print_err("unexpected underflow", pc),
                    }
                    length -= 1;
                }

                for c in chars_to_print.iter().rev() {
                    print!("{}", c);
                }

                let offset: usize = buf.into();
                pc += offset + 1;
            }
            Some(Opcode::Print) => {
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
            Some(Opcode::Jump) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    pc = new_pc;
                } else {
                    print_err("missing jumping address for JUMP", pc);
                }
            }
            Some(Opcode::Jez) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    if stack.len() >= 1 {
                        let mut x: i32 = 0;

                        match stack.pop() {
                            Some(stack_top) => x = stack_top,
                            None => {
                                print_err("unexpected stack underflow", pc);
                            }
                        }

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
            Some(Opcode::Jeq) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    if stack.len() >= 1 {
                        let mut a: i32 = 0;
                        let mut b: i32 = 0;

                        match stack.pop() {
                            Some(stack_top) => a = stack_top,
                            None => {
                                print_err("unexpected stack underflow", pc);
                            }
                        }

                        match stack.pop() {
                            Some(stack_top) => b = stack_top,
                            None => {
                                print_err("unexpected stack underflow", pc);
                            }
                        }

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
            Some(Opcode::Jgt) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    if stack.len() >= 1 {
                        let mut a: i32 = 0;
                        let mut b: i32 = 0;

                        match stack.pop() {
                            Some(stack_top) => a = stack_top,
                            None => {
                                print_err("unexpected stack underflow", pc);
                            }
                        }

                        match stack.pop() {
                            Some(stack_top) => b = stack_top,
                            None => {
                                print_err("unexpected stack underflow", pc);
                            }
                        }

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
            Some(Opcode::Jlt) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    if stack.len() >= 1 {
                        let mut a: i32 = 0;
                        let mut b: i32 = 0;

                        match stack.pop() {
                            Some(stack_top) => a = stack_top,
                            None => {
                                print_err("unexpected stack underflow", pc);
                            }
                        }

                        match stack.pop() {
                            Some(stack_top) => b = stack_top,
                            None => {
                                print_err("unexpected stack underflow", pc);
                            }
                        }

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
            Some(Opcode::Jge) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    if stack.len() >= 1 {
                        let mut a: i32 = 0;
                        let mut b: i32 = 0;

                        match stack.pop() {
                            Some(stack_top) => a = stack_top,
                            None => {
                                print_err("unexpected stack underflow", pc);
                            }
                        }

                        match stack.pop() {
                            Some(stack_top) => b = stack_top,
                            None => {
                                print_err("unexpected stack underflow", pc);
                            }
                        }

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
            Some(Opcode::Jle) => {
                if let Some(byte_slice) = program.get(pc + 1..pc + 5) {
                    let bytes: [u8; 4] = byte_slice.try_into().unwrap();
                    let new_pc = i32::from_le_bytes(bytes) as usize;

                    if stack.len() >= 1 {
                        let mut a: i32 = 0;
                        let mut b: i32 = 0;

                        match stack.pop() {
                            Some(stack_top) => a = stack_top,
                            None => {
                                print_err("unexpected stack underflow", pc);
                            }
                        }

                        match stack.pop() {
                            Some(stack_top) => b = stack_top,
                            None => {
                                print_err("unexpected stack underflow", pc);
                            }
                        }

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

                    match stack.pop() {
                        Some(stack_top) => vars[loc] = stack_top,
                        None => print_err("stack too short for STORE", pc),
                    }
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
