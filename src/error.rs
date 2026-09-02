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
//! The errors for this crate
#[cfg(feature = "repl")]
use rustyline::error::ReadlineError;
use thiserror::Error;
/// The standard brainfuck error type
/// Has runtime & syntax errors
#[derive(Error, Debug)]
pub enum Error {
    /// A standard IO error, just a wrapper for [`std::io::Error`]
    #[error("mischellaneous I/O error")]
    Io(#[from] std::io::Error),
    /// When something request for the program to stop
    #[error("program was requested to stop")]
    Stop,
    /// When the pointer moves to a negative memory address that doesn't exist
    #[error("pointer moved to a negative address at line {0}, character {1}")]
    NegativeAddress(usize, usize),
    /// When the pointer moves to a memory address >= `30_000`
    /// Example:
    /// `[-]+[[-]>[-]+]`
    #[error("pointer moved to a memory address >= 30_000 at line {0}, character {1}")]
    TooLargeAddress(usize, usize),
    /// When a [ is unmatched
    #[error("unmatched [ at line {0}, character {1}")]
    UnmatchedStart(usize, usize),
    /// When a ] is unmatched
    #[error("unmatched ] at line {0}, character {1}")]
    UnmatchedEnd(usize, usize),
    /// When something uses the wrong [`crate::interpreters::Jump`] type
    /// E.G: a function expects a table & gets a stack
    #[error("wrong type of Jump was used")]
    JumpType,
}
/// A wrapper for Result<T, [`Error`]>
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "repl")]
impl From<ReadlineError> for Error {
    fn from(error: ReadlineError) -> Self {
        match error {
            ReadlineError::Io(io_err) => Error::from(io_err),
            ReadlineError::Eof | ReadlineError::Interrupted => Error::Stop,
            #[cfg(unix)]
            ReadlineError::Signal(rustyline::error::Signal::Interrupt) => Error::Stop,
            err => panic!(
                "Unexpected error: {err} found, please notify your distributor of the program"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    const LINE: usize = 69;
    const CHAR: usize = 420;
    const POS: (usize, usize) = (LINE, CHAR);
    #[rstest]
    #[case::jump_type(Error::JumpType, "Wrong Jump type")]
    // I don't know how I should test the `Error::Io`
    // TODO: Implement testing for `Error::Io`
    #[case::stop(Error::Stop, "Program was requested to stop")]
    #[case::negative_address(Error::NegativeAddress(POS), &format!("Pointer moved to a negative address at line {LINE}, character {CHAR}"))]
    #[case::too_large_address(Error::TooLargeAddress(POS), &format!("Pointer moved to a memory address >= 30_000 at line {LINE}, character {CHAR}"))]
    #[case::unmatched_start(Error::UnmatchedStart(POS), &format!("Unmatched [ at line {LINE}, character {CHAR}"))]
    #[case::unmatched_end(Error::UnmatchedEnd(POS), &format!("Unmatched ] at line {LINE}, character {CHAR}"))]
    fn display(#[case] error: Error, #[case] expected: &str) {
        assert_eq!(
            format!("{error}"),
            String::from(expected),
            "Failed to write correct output\nRecieved output: {error:#?}\nExpected output: {expected:#?}"
        );
    }

    #[rstest]
    #[case(
        crate::Error::IO(std::io::ErrorKind::NotFound),
        std::io::Error::new(std::io::ErrorKind::NotFound, "")
    )]
    fn from_io(#[case] expected: crate::Error, #[case] error: std::io::Error) {
        assert_eq!(expected, crate::Error::from(error));
    }

    #[cfg(feature = "repl")]
    #[rstest]
    #[case::io(
        crate::Error::IO(std::io::ErrorKind::NotFound),
        ReadlineError::Io(std::io::Error::from(std::io::ErrorKind::NotFound))
    )]
    #[case::stop(crate::Error::Stop, ReadlineError::Eof)]
    #[should_panic = "Unexpected error: Signal(Resize) found, please notify your distributor of the program"]
    #[case::unkown_error(
        crate::Error::Stop,
        ReadlineError::Signal(rustyline::error::Signal::Resize)
    )]
    fn from_rustyline(#[case] expected: crate::Error, #[case] error: ReadlineError) {
        assert_eq!(expected, Error::from(error));
    }
}
