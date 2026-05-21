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
//! The file for the function that interprets the program from source
use super::objects::State;
use crate::compiler::SourceProgram;
impl SourceProgram {
    /// Interprets the program's source
    /// # Errors
    /// Returns an error if:
    /// The pointer moves out of bounds
    /// `<`
    pub fn interpret(&self) -> crate::Result<()> {
        let mut state = State::from_string(self.0.clone());
        let chars: Vec<char> = self.0.chars().collect();
        let mut err_pos: (usize, usize) = (0, 0);
        let mut c: char;
        while state.pos < chars.len() - 1 {
            err_pos.1 += 1;
            c = chars[state.pos];
            match c {
                '+' => {
                    state.add(1);
                }
                '-' => {
                    state.sub(1);
                }
                '<' => {
                    state.mvl(1, err_pos.0, err_pos.1)?;
                }
                '>' => {
                    state.mvr(1, err_pos.0, err_pos.1)?;
                }
                '[' => {
                    state.srt(1);
                }
                ']' => {
                    state.end(1, err_pos.0, err_pos.1)?;
                }
                '.' => {
                    state.out(1)?;
                }
                ',' => {
                    state.inp()?;
                }
                '\n' => {
                    err_pos.0 += 1;
                    err_pos.1 = 1;
                }
                _ => (),
            }
            state.pos += 1;
        }
        Ok(())
    }
}
