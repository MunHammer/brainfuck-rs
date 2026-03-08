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
use console::{Key, Term};
use std::io::{Write, stdout};
/// An enum to decide if you use a jump table or a jump stack
pub enum Jump {
    /// A stack of jumps, beter for smaller programs or REPLs
    Stack(Vec<usize>),
    /// A table of jumps, better for larger programs or for speed
    Table(Vec<(usize, usize)>),
}

/// A state of a given program
pub struct State {
    pub tape: [u8; 30_000],
    pub ptr: usize,
    pub pos: usize,
    pub loop_num: usize,
    pub jumps: Jump,
}
impl Jump {
    /// Gets the tuple of the table at the pos position
    /// # Errors
    /// If the [`Jump`] isn't a [`Jump::Table`]
    pub fn table_pos(&self, pos: usize) -> crate::Result<(usize, usize)> {
        if let Self::Table(table) = self {
            return Ok(table[pos]);
        }
        Err(crate::Error::JumpType)
    }
}
impl State {
    /// Creates a new instance of the [`State`] struct, with [`Jump::Stack`]
    #[must_use]
    pub fn new() -> Self {
        Self {
            tape: [0; 30_000],
            ptr: 0,
            pos: 0,
            loop_num: 0,
            jumps: Jump::Stack(Vec::new()),
        }
    }
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
                Jump::Table(jump_table)
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
    pub fn mvr(&mut self, amount: usize, x: usize, y: usize) -> crate::Result<&mut Self> {
        if self.ptr + amount == 30_000 {
            return crate::Result::Err(crate::Error::TooLargeAddress((x, y)));
        }
        self.ptr += amount;
        Ok(self)
    }
    /// Moves the pointer left
    pub fn mvl(&mut self, amount: usize, x: usize, y: usize) -> crate::Result<&mut Self> {
        if self.ptr.saturating_sub(amount) == 0 {
            return crate::Result::Err(crate::Error::NegativeAddress((x, y)));
        }
        self.ptr -= amount;
        Ok(self)
    }
    /// Starts a loop
    pub fn srt(&mut self, times: usize) -> crate::Result<&mut Self> {
        for _ in 0..times {
            if self.tape[self.ptr] == 0 {
                self.pos = self.jumps.table_pos(self.loop_num)?.1;
            }
            self.loop_num += 1;
        }
        Ok(self)
    }
    /// Ends a loop
    pub fn end(&mut self, times: usize) -> crate::Result<&mut Self> {
        if let Jump::Table(_) = self.jumps {
            for _ in 0..times {
                if self.tape[self.ptr] != 0 {
                    self.pos = self.jumps.table_pos(self.loop_num)?.0 - 1;
                }
                self.loop_num -= 1;
            }
        }
        Ok(self)
    }
    /// Outputs the ascii value of the current cell
    pub fn out(&self, times: usize, capture: bool) -> crate::Result<(&Self, Option<String>)> {
        let mut out = String::new();
        for _ in 0..times {
            if capture {
                out.push(self.tape[self.ptr] as char);
            } else {
                print!("{}", self.tape[self.ptr] as char);
                stdout().flush()?;
            }
        }
        if capture {
            return Ok((self, Some(out)));
        }
        Ok((self, None))
    }
    /// Takes input
    pub fn inp(&mut self) -> crate::Result<&mut Self> {
        let mut buf: [u8; 1] = [0];
        self.tape[self.ptr] = {
            let mut out: Option<char> = None;
            loop {
                let mut done = false;
                match Term::stdout().read_key_raw()? {
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
                    print!("\x7f{val}");
                }
                stdout().flush()?;
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
        self.tape[self.ptr] = buf[0];
        Ok(self)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
