/*
brainfuck-rs: A brainfuck CLI interpreter & compiler
Copyright (C) 2026  Mun_Hammer

This file is part of brainfuck-rs

brainfuck-rs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

brainfuck-rs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with brainfuck-rs.  If not, see <https://www.gnu.org/licenses/>.
see ../../COPYING for the full license
*/
use crate::pre_proccesser::SourceProgram;
use console::{Key, Term};
use std::io::{Read, Write};
pub fn repl() {
    println!(
        "brainfuck-rs  Copyright (C) 2026  Mun_Hammer\nThis program comes with ABSOLUTELY NO WARRANTY; for details type `show w'.\nThis is free software, and you are welcome to redistribute it\nunder certain conditions; type `show c' for details.\nEnter \"Reset\" or \"Clear\" to reset the program's state"
    );
    //              tape          ptr    pos    loop stack
    let mut state: ([u8; 30_000], usize, usize, Vec<usize>) = ([0; 30_000], 0, 0, Vec::new());
    let mut out: bool;
    loop {
        let mut inp: String = String::new();
        out = false;
        if state.3.is_empty() {
            print!(">>> ");
        } else {
            print!("... ");
        }
        match std::io::stdout().flush() {
            Ok(()) => (),
            Err(err) => eprintln!("{err}"),
        }
        match std::io::stdin().read_line(&mut inp) {
            Ok(()) => (),
            Err(err) => eprintln!("{err}"),
        }
        inp = String::from(inp.trim_end());
        if inp == "exit" || inp == "quit" {
            break;
        } else if inp == "show w" {
            println!(
                "This program is distributed in the hope that it will be useful,\nbut WITHOUT ANY WARRANTY; without even the implied warranty of\nMERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\nGNU General Public License for more details."
            );
        } else if inp == "show c" {
            println!("IN PROGRESS");
        } else if inp == "clear" || inp == "reset" {
            state = ([0; 30_000], 0, 0, Vec::new());
            println!("Tape, pointer & loop stacks have all been reset");
        }
        let inpe: Vec<char> = inp.chars().collect();
        state.2 = 0;
        while let Some(val) = inpe.get(state.2) {
            let op = val;
            match op {
                '+' => state.0[state.1] = state.0[state.1].wrapping_add(1),
                '-' => state.0[state.1] = state.0[state.1].wrapping_sub(1),
                '>' => {
                    state.1 += 1;
                    if state.1 == 30_000 {
                        eprintln!(
                            "Pointer moved out of bounds, pointer location moved back to zero"
                        );
                        state.1 = 0;
                        break;
                    }
                }
                '<' => {
                    if state.1 == 0 {
                        eprintln!(
                            "Pointer moved out of bounds, pointer location moved back to zero"
                        );
                        state.1 = 0;
                        break;
                    }
                    state.1 -= 1;
                }
                '.' => {
                    out = true;
                    print!("{}", state.0[state.1] as char);
                    std::io::stdout().flush().unwrap();
                }
                ',' => {
                    let mut buf: [u8; 1] = [0];
                    state.0[state.1] = {
                        let mut out: Option<char> = None;
                        loop {
                            let mut done = false;
                            match Term::stdout().read_key().unwrap() {
                                Key::Char(c) => {
                                    out = Some(c);
                                }
                                Key::Enter => {
                                    done = true;
                                }
                                Key::Backspace => {
                                    print!("\x7f");
                                    out = None;
                                }
                                _ => (),
                            }
                            if let Some(val) = out {
                                print!("\x7f{}", val);
                            }
                            match std::io::stdout().flush() {
                                Ok(()) => (),
                                Err(err) => eprintln!("{err}"),
                            }
                            if done && out.is_some() {
                                break;
                            }
                        }
                        match out {
                            Some(val) => val,
                            None => {
                                unreachable!();
                            }
                        }
                        .encode_utf8(&mut buf);
                        buf[0]
                    };
                    match std::io::stdin().read_exact(&mut buf) {
                        Ok(()) => (),
                        Err(err) => {
                            eprintln!("{err}");
                        }
                    }
                    state.0[state.1] = buf[0];
                }
                '[' => {
                    if state.0[state.1] != 0 {
                        state.3.push(state.2);
                    } else {
                        let mut depth: u16 = 1;
                        while depth > 0 {
                            state.2 += 1;
                            match match inpe.get(state.2) {
                                Some(val) => val,
                                None => break,
                            } {
                                '[' => depth += 1,
                                ']' => depth -= 1,
                                _ => (),
                            }
                        }
                    }
                }
                ']' => {
                    if state.0[state.1] > 0 {
                        state.2 = *state.3.last().unwrap();
                        continue;
                    } else if state.3.is_empty() {
                        eprintln!("Unmatched ], breaking interpretation");
                        break;
                    }
                    state.3.pop();
                }
                _ => (),
            }
            state.2 += 1;
        }
        if out {
            println!();
        }
    }
}
struct ProgramState {
    tape: [u8; 30_000],
    ptr: usize,
    pos: usize,
    loop_num: usize,
    jumps: Vec<(usize, usize)>,
}
impl ProgramState {
    /// Using an object that can be turned into a string, makes a [`ProgramState`] with a jump table
    pub fn from_string<S: Into<String>>(program: S) -> Self {
        Self {
            loop_num: 0,
            tape: [0; 30_000],
            pos: 0,
            ptr: 0,
            jumps: {
                let mut jump_table: Vec<(usize, usize)> = Vec::new();
                let mut depth: usize = 0;
                for (num, op) in program.into().chars().enumerate() {
                    match op {
                        '[' => {
                            depth += 1;
                            jump_table.push((num, 0));
                        }
                        ']' => {
                            depth -= 1;
                            let length = jump_table.len() - depth - 1;
                            jump_table[length].1 = num;
                        }
                        _ => (),
                    }
                }
                jump_table
            },
        }
    }
    /// Adds to the current cell
    pub fn add(&mut self, amount: u8) -> &mut Self {
        self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(amount);
        self
    }
    /// Subtracts from the current cell
    pub fn sub(&mut self, amount: u8) -> &mut Self {
        self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub(amount);
        self
    }
    /// Moves the pointer right
    pub fn mvr(&mut self, amount: usize) -> Result<&mut Self, String> {
        if self.ptr + amount == 30_000 {
            return Err(String::from("Cannot move past tape bounds of 30_000 cells"));
        }
        self.ptr += amount;
        Ok(self)
    }
    /// Moves the pointer left
    pub fn mvl(&mut self, amount: usize) -> Result<&mut Self, String> {
        if self.ptr.saturating_sub(amount) == 0 {
            return Err(String::from("Cannot move to negative tape addresses"));
        }
        self.ptr -= amount;
        Ok(self)
    }
    /// Starts a loop
    pub fn srt(&mut self, times: usize) -> &mut Self {
        for _ in 0..times {
            if self.tape[self.ptr] == 0 {
                self.pos = self.jumps[self.loop_num].1;
            }
            self.loop_num += 1;
        }
        self
    }
    /// Ends a loop
    pub fn end(&mut self, times: usize) -> &mut Self {
        for _ in 0..times {
            if self.tape[self.ptr] != 0 {
                self.pos = self.jumps[self.loop_num].0 - 1;
            }
            self.loop_num -= 1;
        }
        self
    }
    /// Outputs the ascii value of the current cell
    pub fn out(&self, times: usize, capture: bool) -> (&Self, Option<String>) {
        let mut out = String::new();
        for _ in 0..times {
            if capture {
                out.push(self.tape[self.ptr] as char);
            } else {
                print!("{}", self.tape[self.ptr] as char);
            }
        }
        if capture {
            return (self, Some(out));
        }
        (self, None)
    }
    /// Takes input
    pub fn inp(&mut self) -> Result<&mut Self, String> {
        let mut buf: [u8; 1] = [0];
        match std::io::stdin().read_exact(&mut buf) {
            Ok(()) => (),
            Err(err) => {
                return Err(err.to_string());
            }
        }
        self.tape[self.ptr] = buf[0];
        Ok(self)
    }
}
impl SourceProgram {
    pub fn interpret(&self) -> Result<(), String> {
        let mut state = ProgramState::from_string(self.0.clone());
        let chars: Vec<char> = self.0.chars().collect();
        let mut c: char;
        while state.pos < chars.len() - 1 {
            c = chars[state.pos];
            match c {
                '+' => {
                    state.add(1);
                }
                '-' => {
                    state.sub(1);
                }
                '<' => {
                    state.mvl(1)?;
                }
                '>' => {
                    state.mvr(1)?;
                }
                '[' => {
                    state.srt(1);
                }
                ']' => {
                    state.end(1);
                }
                '.' => {
                    state.out(1, false);
                }
                ',' => {
                    state.inp()?;
                }
                _ => (),
            }
            state.pos += 1;
        }
        Ok(())
    }
}
