pub mod hardware;
pub use hardware::*;
pub mod consts_enums;
pub use consts_enums::*;
pub mod config;
pub use config::*;
pub mod debugger;
pub use debugger::*;
pub mod crashdump;
pub use crashdump::*;
pub mod misc;
pub use misc::*;
// tests
#[macro_export]
macro_rules! test_instruction {
    ($bcpu:expr, $op:ident, $arg1:expr, $arg2:expr) => {{
        use Argument::*;
        use Instruction::*;

        let bcpu = &mut $bcpu;

        let parse_argument = |arg: &str| -> Argument {
            match arg {
                s if s.starts_with("&r") => {
                    let val: i16 = s[2..].parse().expect("Invalid register pointer");
                    RegPtr(val)
                }
                s if s.starts_with("&$") => {
                    let val: i16 = s[2..].parse().expect("Invalid memory pointer");
                    MemPtr(val)
                }
                s if s.starts_with("$") => {
                    let val: i16 = s[1..].parse().expect("Invalid memory address");
                    MemAddr(val)
                }
                s if s.parse::<i16>().is_ok() => {
                    let val: i16 = s.parse().expect("Invalid literal");
                    Literal(val)
                }
                s if s.starts_with("r") => {
                    let val: i16 = s[1..].parse().expect("Invalid register");
                    Register(val)
                }
                _ => panic!("moo"),
            }
        };

        let ins = match stringify!($op).to_uppercase().as_str() {
            "HLT" => HLT,
            "ADD" => ADD(parse_argument($arg1), parse_argument($arg2)),
            "JO" => JO(parse_argument($arg1)),
            "POP" => POP(parse_argument($arg1)),
            "DIV" => DIV(parse_argument($arg1), parse_argument($arg2)),
            "RET" => RET,
            "LD" => LD(parse_argument($arg1), parse_argument($arg2)),
            "ST" => ST(parse_argument($arg1), parse_argument($arg2)),
            "JMP" => JMP(parse_argument($arg1)),
            "JZ" => JZ(parse_argument($arg1)),
            "CMP" => CMP(parse_argument($arg1), parse_argument($arg2)),
            "MUL" => MUL(parse_argument($arg1), parse_argument($arg2)),
            "PUSH" => PUSH(parse_argument($arg1)),
            "INT" => INT(parse_argument($arg1)),
            "MOV" => MOV(parse_argument($arg1), parse_argument($arg2)),
            "NOP" => NOP,
            _ => panic!("cheep cheep"),
        };

        if let Err(e) = bcpu.execute_instruction(&ins) {
            eprintln!("oops: {}", e);
        }
        bcpu
    }};

    ($bcpu:expr, $op:ident, $arg1:expr) => {{
        test_instruction!($bcpu, $op, $arg1, "");
    }};

    ($bcpu:expr, $op:ident) => {{
        test_instruction!($bcpu, $op, "", "");
    }};
}

#[test]
fn add_with_immediate() {
    let mut bcpu = CPU::new();
    test_instruction!(bcpu, add, "r0", "3");
    assert_eq!(bcpu.int_reg[0], 3);
}

#[test]
fn add_with_register() {
    let mut bcpu = CPU::new();
    bcpu.int_reg[0] = 5;
    bcpu.int_reg[2] = 12;

    test_instruction!(bcpu, add, "r0", "r2");
    assert_eq!(bcpu.int_reg[0], 17);
}

#[test]
fn add_with_rptr() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = 21;
    bcpu.int_reg[2] = 33;

    bcpu.memory[33] = Some(42);

    test_instruction!(bcpu, add, "r0", "&r2");
    assert_eq!(bcpu.int_reg[0], 63);
}

#[test]
fn add_with_mptr() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = 444;

    bcpu.memory[33] = Some(42);

    bcpu.memory[42] = Some(60);

    test_instruction!(bcpu, add, "r0", "&$33");
    assert_eq!(bcpu.int_reg[0], 504);
}

#[test]
fn add_with_negative_register() {
    let mut bcpu = CPU::new();

    bcpu.int_reg[0] = -42;

    bcpu.uint_reg[1] = 42;

    test_instruction!(bcpu, add, "r5", "r0");
    assert_eq!(bcpu.uint_reg[1], 0);
    test_instruction!(bcpu, add, "r5", "r0");
    assert_eq!(bcpu.uint_reg[1], 65494);
}
