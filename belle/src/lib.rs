pub mod hardware;
pub use hardware::*;
pub mod config;
pub mod debugger;
pub use debugger::*;
pub mod crashdump;
pub use crashdump::*;
pub mod misc;
pub use misc::*;

#[cfg(test)]
pub mod test;

// tests
#[macro_export]
macro_rules! test_instruction {
    ($bcpu:expr, $op:ident, $arg1:expr, $arg2:expr) => {{
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
            panic!("{e}");
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
