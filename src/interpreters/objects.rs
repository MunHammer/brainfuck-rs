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
see COPYING for the full license
*/
//! This file is to declare the structs & enums & impl some basic fuctions
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
    /// The program's tape, in which all of the cells are stored
    pub tape: [u8; 30_000],
    /// The pointer to the tape's cell, with which most cell manipulation commmands will be executed on
    pub ptr: usize,
    /// The position in the program that the interpreter is at
    pub pos: usize,
    // TODO: put this in the `Jump` enum
    /// The depth at which the jump stack is at
    pub loop_num: usize,
    /// The stack or table of jumps
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

// This trait is for testing methods that use stdin
trait Input {
    fn read_key(&mut self) -> crate::Result<Key>;
}

struct TermInput;

impl Input for TermInput {
    fn read_key(&mut self) -> crate::Result<Key> {
        Ok(Term::stdout().read_key_raw()?)
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
    pub fn out(&self, times: usize) -> crate::Result<&Self> {
        self.out_inner(times, stdout())
    }
    /// Outputs the ascii value of the current cell, with a trait for testing
    pub fn out_inner<Out: Write>(&self, times: usize, mut write: Out) -> crate::Result<&Self> {
        for _ in 0..times {
            write!(write, "{}", self.tape[self.ptr] as char)?;
            write.flush()?;
        }
        Ok(self)
    }
    /// Takes input
    pub fn inp(&mut self) -> crate::Result<&mut Self> {
        self.inp_inner(TermInput, stdout())
    }
    /// Takes input, with a trait, for testing
    fn inp_inner<Inp: Input, Out: Write>(
        &mut self,
        mut read: Inp,
        mut write: Out,
    ) -> crate::Result<&mut Self> {
        let mut buf: [u8; 1] = [0];
        self.tape[self.ptr] = {
            let mut out: Option<char> = None;
            loop {
                let mut done = false;
                match read.read_key()? {
                    Key::Char(c) => {
                        if out.is_none() {
                            write!(write, "{c}")?;
                        } else {
                            write!(write, "\x7f{c}")?;
                        }
                        out = Some(c);
                    }
                    Key::Enter => {
                        done = true;
                    }
                    Key::Backspace => {
                        write!(write, "\x7f")?;
                        out = None;
                    }
                    _ => (),
                }
                write.flush()?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    mod state {
        use std::{collections::VecDeque, io};

        use super::*;
        #[derive(Debug)]
        struct FakeInput(VecDeque<Key>);
        impl Input for FakeInput {
            fn read_key(&mut self) -> crate::Result<Key> {
                match self.0.pop_front() {
                    Some(key) => Ok(key),
                    None => Err(crate::Error::IO(io::ErrorKind::UnexpectedEof.into())),
                }
            }
        }
        struct FakeOutput(String);
        impl Write for FakeOutput {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                for c in buf {
                    self.0.push(*c as char)
                }
                Ok(buf.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        // backspace is "\x7f"
        #[rstest]
        #[case::one(vec![Key::Char('a'), Key::Enter], b'a', "a")]
        #[case::two(vec![Key::Char('b'), Key::Char('c'), Key::Enter], b'c', "b\x7fc")]
        #[case::three(vec![Key::Char('d'), Key::Char('e'), Key::Char('f'), Key::Enter], b'f', "d\x7fe\x7ff")]
        #[case::backspace(vec![Key::Char('g'), Key::Backspace, Key::Char('h'), Key::Enter], b'h', "g\x7fh")]
        fn inp(#[case] input: Vec<Key>, #[case] expected: u8, #[case] expected_out: &str) {
            let mut state = State::new();
            let mut output = FakeOutput(String::new());
            state
                .inp_inner(FakeInput(input.into()), &mut output)
                .unwrap();
            assert_eq!(state.tape[state.ptr], expected);
            assert_eq!(output.0, String::from(expected_out));
        }

        #[rstest]
        #[case::once(b'j', "j", 1)]
        #[case::twice(b'k', "kk", 2)]
        fn out(#[case] cell: u8, #[case] expected: &str, #[case] times: usize) {
            let mut state = State::new();
            let mut output = FakeOutput(String::new());
            state.add(cell);
            state.out_inner(times, &mut output).unwrap();
            assert_eq!(output.0, expected);
        }
    }
}
