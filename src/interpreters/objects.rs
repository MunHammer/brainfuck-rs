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
#[derive(Debug)]
pub enum Jump {
    /// A stack of jumps, beter for smaller programs or REPLs
    Stack(Vec<usize>),
    /// A table of jumps, better for larger programs or for speed
    Table(Vec<(usize, usize)>),
}

/// A state of a given program
#[derive(Debug)]
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
pub(crate) trait Input {
    fn read_key(&mut self) -> crate::Result<Key>;
}

#[derive(Clone)]
pub(crate) struct TermInput;

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
                let mut loop_num: usize = usize::MAX;
                for (num, op) in program.into().chars().enumerate() {
                    match op {
                        '[' => {
                            loop_num = loop_num.wrapping_add(1);
                            jump_table.push((num, 0));
                        }
                        ']' => {
                            jump_table[loop_num].1 = num;
                            loop_num = loop_num.wrapping_sub(1);
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
    /// # Errors
    /// If the ptr moves to a cell value greater than `30_000`
    pub fn mvr(&mut self, amount: usize, x: usize, y: usize) -> crate::Result<&mut Self> {
        if let (_, true) = self.ptr.overflowing_add(amount) {
            return crate::Result::Err(crate::Error::TooLargeAddress((x, y)));
        } else if self.ptr + amount > 30_000 {
            return crate::Result::Err(crate::Error::TooLargeAddress((x, y)));
        }
        self.ptr += amount;
        Ok(self)
    }
    /// Moves the pointer left
    /// # Errors
    /// If the ptr moves to a cell value less than 0
    pub fn mvl(&mut self, amount: usize, x: usize, y: usize) -> crate::Result<&mut Self> {
        if let (_, true) = self.ptr.overflowing_sub(amount) {
            return crate::Result::Err(crate::Error::NegativeAddress((x, y)));
        }
        self.ptr -= amount;
        Ok(self)
    }
    /// Starts a loop
    /// # Errors
    ///
    pub fn srt(&mut self, times: usize) -> &mut Self {
        for _ in 0..times {
            match &mut self.jumps {
                Jump::Table(_) => {
                    if self.tape[self.ptr] == 0 {
                        self.pos = match self.jumps.table_pos(self.loop_num) {
                            Ok(val) => val,
                            Err(err) => unreachable!("Error {err} shouldn't be come across, as Jump::table_pos only panics when the jump isn't a Jump::Table, which has already be checked for"),
                        }
                        .1;
                    }
                    self.loop_num += 1;
                }
                Jump::Stack(stack) => stack.push(self.pos),
            }
        }
        self
    }
    /// Ends a loop
    /// # Errors
    /// If an unmatched end is detected, only found if the `State` is using `Jump::State`
    pub fn end(&mut self, times: usize, x: usize, y: usize) -> crate::Result<&mut Self> {
        for _ in 0..times {
            match &mut self.jumps {
                Jump::Table(_) => {
                    if self.tape[self.ptr] != 0 {
                        self.pos = match self.jumps.table_pos(self.loop_num) {
                            Ok(val) => val,
                            Err(err) => unreachable!("Error {err} shouldn't be come across, as Jump::table_pos only panics when the jump isn't a Jump::Table, which has already be checked for"),
                        }.0 - 1;
                    }
                    self.loop_num -= 1;
                }
                Jump::Stack(stack) => {
                    if self.tape[self.ptr] != 0 {
                        self.pos = if let Some(val) = stack.last() {
                            *val
                        } else {
                            return Err(crate::Error::UnmatchedEnd((x, y)));
                        }
                    }
                    stack.pop();
                }
            }
        }
        Ok(self)
    }
    /// Outputs the ascii value of the current cell
    /// # Errors
    /// Only if there is an I/O error
    pub fn out(&self, times: usize) -> crate::Result<&Self> {
        self.out_inner(times, stdout())
    }
    /// Outputs the ascii value of the current cell, with a trait for testing
    /// # Errors
    /// Only if there is an I/O error
    pub(crate) fn out_inner(&self, times: usize, mut write: impl Write) -> crate::Result<&Self> {
        for _ in 0..times {
            let _ = write.write(&[self.tape[self.ptr]])?;
            write.flush()?;
        }
        Ok(self)
    }
    /// Takes input
    /// # Errors
    /// If there is an I/O error
    pub fn inp(&mut self) -> crate::Result<&mut Self> {
        self.inp_inner(TermInput, stdout())
    }
    /// Takes input, with a trait, for testing
    /// # Errors
    /// If there is an I/O error
    pub(crate) fn inp_inner(
        &mut self,
        mut read: impl Input,
        mut write: impl Write,
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
#[derive(Debug, Clone)]
pub(crate) struct FakeInput(pub(crate) std::collections::VecDeque<Key>);
#[cfg(test)]
impl Input for FakeInput {
    fn read_key(&mut self) -> crate::Result<Key> {
        match self.0.pop_front() {
            Some(key) => Ok(key),
            None => Err(crate::Error::IO(std::io::ErrorKind::UnexpectedEof)),
        }
    }
}
#[cfg(test)]
pub(crate) struct FakeOutput(pub(crate) Vec<u8>);
#[cfg(test)]
impl FakeOutput {
    pub(crate) fn string(&self) -> String {
        self.0.iter().map(|b| *b as char).collect()
    }
}
#[cfg(test)]
impl Write for FakeOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        dbg!(buf);
        for b in buf {
            dbg!(b);
            self.0.push(*b);
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    mod state {
        use super::*;
        // backspace is "\x7f"
        #[rstest]
        #[case::one(vec![Key::Char('a'), Key::Enter], b'a', "a")]
        #[case::two(vec![Key::Char('b'), Key::Char('c'), Key::Enter], b'c', "b\x7fc")]
        #[case::three(vec![Key::Char('d'), Key::Char('e'), Key::Char('f'), Key::Enter], b'f', "d\x7fe\x7ff")]
        #[case::backspace(vec![Key::Char('g'), Key::Backspace, Key::Char('h'), Key::Enter], b'h', "g\x7fh")]
        fn inp(#[case] input: Vec<Key>, #[case] expected: u8, #[case] expected_out: &str) {
            let mut state = State::new();
            let mut output = FakeOutput(Vec::new());
            state
                .inp_inner(FakeInput(input.clone().into()), &mut output)
                .unwrap();
            assert_eq!(
                state.tape[state.ptr], expected,
                "State::inp didn't take input correctly from set of inputs {input:#?}\nRecieved output: {:#?}\nExpected output: {expected:#?}",
                state.tape[state.ptr]
            );
            assert_eq!(
                output.string(),
                String::from(expected_out),
                "State::inp didn't output correctly for the user from set of inputs {input:#?}\nRecieved output: {:#?}\nExpected output: {expected_out:#?}",
                output.0
            );
        }

        #[rstest]
        #[case::once(b'j', "j", 1)]
        #[case::twice(b'k', "kk", 2)]
        fn out(#[case] cell: u8, #[case] expected: &str, #[case] times: usize) {
            let mut state = State::new();
            let mut output = FakeOutput(Vec::new());
            state.tape[state.ptr] = cell;
            state.out_inner(times, &mut output).unwrap();
            assert_eq!(
                output.string(),
                expected,
                "Didn't output the correct value(s) from cell {cell:#?}\nRecieved output: {:#?}\nExpected output: {expected:#?}",
                output.0
            );
        }

        #[rstest]
        #[case::quarter(0, 64, 64)]
        #[case::half(0, 128, 128)]
        #[case::overflow(255, 129, 128)]
        fn add(#[case] starting: u8, #[case] amount: u8, #[case] expected: u8) {
            let mut state = State::new();
            state.tape[state.ptr] = starting;
            state.add(amount);
            assert_eq!(
                state.tape[state.ptr], expected,
                "Didn't add the correct amount to the cell. Starting value: {starting:#?}. Value to add {amount:#?}\nrecieved output: {:#?}\nExpected output: {expected:#?}",
                state.tape[state.ptr]
            );
        }
        #[rstest]
        #[case::quarter(64, 64, 0)]
        #[case::half(128, 128, 0)]
        #[case::overflow(0, 128, 128)]
        fn sub(#[case] starting: u8, #[case] amount: u8, #[case] expected: u8) {
            let mut state = State::new();
            state.tape[state.ptr] = starting;
            state.sub(amount);
            assert_eq!(
                state.tape[state.ptr], expected,
                "Didn't subtract the correct amount to the cell. Starting value: {starting:#?}. Value to subtract {amount:#?}\nrecieved output: {:#?}\nExpected output: {expected:#?}",
                state.tape[state.ptr]
            );
        }
        #[rstest]
        #[case::half(30_000, 15_000, 15_000)]
        #[should_panic(expected = "Failed to move past 0")]
        #[case::undeflow_panic(0, 1, 420)]
        fn mvl(#[case] starting: usize, #[case] amount: usize, #[case] expected_cell: usize) {
            let mut state = State::new();
            state.ptr = starting;
            state.mvl(amount, 0, 0).expect("Failed to move past 0");
            assert_eq!(
                state.ptr, expected_cell,
                "Didn't move left the expected amount. Starting cell: {starting:#?}. Cells to move: {amount:#?}\nRecieved balue: {:#?}\nExpected value: {expected_cell:#?}",
                state.ptr
            );
        }
        #[rstest]
        #[case::half(0, 15_000, 15_000)]
        #[should_panic(expected = "Failed to move past 30_000")]
        #[case::overflow_panic(30_000, 1, 420)]
        fn mvr(#[case] starting: usize, #[case] amount: usize, #[case] expected_cell: usize) {
            let mut state = State::new();
            state.ptr = starting;
            state.mvr(amount, 0, 0).expect("Failed to move past 30_000");
            assert_eq!(state.ptr, expected_cell);
        }

        #[rstest]
        #[case::empty("[]", vec![(0, 1)])]
        #[case::basic("[.]", vec![(0, 2)])]
        #[case::program(">-[++++[<]>->+]<", vec![(2, 14), (7, 9)])]
        fn lop_table(#[case] program: &str, #[case] expected: Vec<(usize, usize)>) {
            let state = State::from_string(program);
            if let Jump::Table(table) = state.jumps {
                assert_eq!(table, expected);
            } else {
                unreachable!();
            }
        }
        #[rstest]
        #[case::empty("[", &[0])]
        #[case::basic("[.", &[0])]
        #[case::program(">-[++++[<>->+<", &[2, 7])]
        fn lop_stack(#[case] program: &str, #[case] expected_stack: &[usize]) {
            let mut state = State::new();
            while let Some(c) = program.chars().nth(state.pos) {
                match c {
                    '+' => {
                        state.add(1);
                    }
                    '-' => {
                        state.sub(1);
                    }
                    '>' => {
                        state.mvr(1, 0, 0).unwrap();
                    }
                    '<' => {
                        state.mvl(1, 0, 0).unwrap();
                    }
                    '[' => {
                        state.srt(1);
                    }
                    ']' => {
                        state.end(1, 0, 0).unwrap();
                    }
                    ',' => panic!("Please do not enter input to this"),
                    _ => (),
                }
                state.pos += 1;
            }
            assert_eq!(
                if let Jump::Stack(stack) = state.jumps {
                    stack
                } else {
                    unreachable!()
                },
                Vec::from(expected_stack)
            );
        }
    }
}
