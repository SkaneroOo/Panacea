use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;
use crate::vm::VM;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

fn numlen(mut num: usize) -> usize {
    if num == 0 {
        return 1;
    }
    let mut ret = 0;
    while num != 0 {
        ret += 1;
        num /= 10;
    }
    return ret;
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![]
        }
    }

    pub fn run(&mut self) {
        println!("Panacea REPL");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout buffer");

            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("Quitting REPL");
                    std::process::exit(0);
                }
                ".history" => {
                    let len = self.command_buffer.len();
                    let space = numlen(len) + 1;
                    for i in 0..len {
                        println!("{}{}-> {}", i+1, " ".repeat(space-numlen(i+1)), self.command_buffer[i])
                    }
                }
                ".program" => {
                    println!("Printing instructions from current VM instance:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of program");
                }
                ".registers" => {
                    println!("Printing registers content:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of registers")
                }
                ".run" => {
                    self.vm.run();
                }
                ".run_once" => {
                    self.vm.run_once();
                }
                _ => {
                    match self.parse_hex(buffer) {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte);
                            }
                        },
                        Err(_e) => {
                            println!("Invalid command or hex string!")
                        }
                    }
                }
            }
        }
    }

    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_str in split {
            match u8::from_str_radix(&hex_str, 16) {
                Ok(res) => {
                    results.push(res);
                },
                Err(e) => {
                    return Err(e)
                }
            }
        }
        Ok(results)
    }
}