/*
brainfuck-rs: A brainfuck CLI interpreter & compiler
Copyright (C) 2026  Mun_Hammer

This file is a part of brainfuck-rs

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
see ../COPYING for the full license
*/
#[cfg(feature = "repl")]
use rustyline::error::ReadlineError;
use std::fmt;
/// The standard brainfuck error type
/// Has runtime & syntax errors
#[derive(Debug)]
pub enum Error {
    /// A standard IO error, just a wrapper for [`std::io::Error`]
    IO(std::io::Error),
    /// When something request for the program to stop
    Stop,
    /// When the pointer moves to a negative memory address that doesn't exist
    /// Example:
    /// ```rust
    /// use brainfuck-rs::interpreters
    /// fn main() {
    ///     let state = interpreters::ProgramState::from_string("[-]+[>[-]+]");
    ///     let error = compiler::SourceProgram::
    /// }
    /// ```
    NegativeAddress((usize, usize)),
    /// When the pointer moves to a memory address >= `30_000`
    /// Example:
    /// `[-]+[[-]>[-]+]`
    TooLargeAddress((usize, usize)),
    /// When a [ is unmatched
    UnmatchedStart((usize, usize)),
    /// When a ] is unmatched
    UnmatchedEnd((usize, usize)),
    /// When something uses the wrong [`crate::interpreters::Jump`] type
    /// E.G: a function expects a table & gets a stack
    JumpType,
}
/// A wrapper for Result<T, [`brainfuck_rs::Error`]>
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IO(err) => write!(f, "{err}"),
            Self::NegativeAddress(pos) => {
                write!(
                    f,
                    "Pointer moved to a negative address at {} {}",
                    pos.0, pos.1
                )
            }
            Self::TooLargeAddress(pos) => {
                write!(
                    f,
                    "Pointer moved to a memory address >= 30_000 at {} {}",
                    pos.0, pos.1
                )
            }
            Self::UnmatchedStart(pos) => write!(f, "Unmatched [ at {} {}", pos.0, pos.1),
            Self::UnmatchedEnd(pos) => write!(f, "Unmatched ] at {} {}", pos.0, pos.1),
            Self::JumpType => write!(f, "Wrong Jump type"),
            Self::Stop => write!(f, "Program was requested to stop"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

#[cfg(feature = "repl")]
impl From<ReadlineError> for Error {
    fn from(error: ReadlineError) -> Self {
        match error {
            ReadlineError::Io(io_err) => Error::from(io_err),
            ReadlineError::Eof
            | ReadlineError::Interrupted
            | ReadlineError::Signal(rustyline::error::Signal::Interrupt) => Error::Stop,
            err => panic!(
                "Unexpected error: {err} found, please notify your distributor of the program"
            ),
        }
    }
}
