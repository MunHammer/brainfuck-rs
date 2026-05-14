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
//! This file is for the lexical analysis of a program
use crate::compiler::front::{BaseOp, SourceProgram, TokenStream};
impl SourceProgram {
    /// Turns the source program into a stream of tokens
    /// Also ignores all other lines
    pub fn lex(&self) -> TokenStream {
        let mut tokens: Vec<BaseOp> = Vec::new();
        // Tokenises valid chars
        for c in self.0.chars() {
            tokens.push(match c {
                '+' => BaseOp::Add,
                '-' => BaseOp::Sub,
                '>' => BaseOp::Mvr,
                '<' => BaseOp::Mvl,
                '[' => BaseOp::Srt,
                ']' => BaseOp::End,
                '.' => BaseOp::Out,
                ',' => BaseOp::Inp,
                '\n' => BaseOp::NewLine,
                _ => BaseOp::CharCount,
            });
        }
        TokenStream(tokens)
    }
    /// Removes all starting comments from a TokenStream
    fn rm_comments(tokens: TokenStream) -> crate::Result<TokenStream> {
        // Removes comments like:
        /*
        [This is a brainfuck comment, you can put any char in here
        but the [] (loops) have to be matched like normal.]
        +++++
        */
        let mut tokens = tokens.0;
        let mut pos: (usize, usize) = (0, 0);
        let mut last_start: (usize, usize) = (0, 0);
        while let BaseOp::Srt = tokens.first().unwrap_or(&BaseOp::Add) {
            let mut depth: u8 = 0;
            for op in tokens.clone() {
                match op {
                    BaseOp::Srt => {
                        last_start = pos;
                        depth += 1;
                    }
                    BaseOp::End => {
                        if depth == 0 {
                            return Err(crate::Error::UnmatchedEnd(pos));
                        }
                        depth -= 1;
                    }
                    BaseOp::NewLine => {
                        pos.0 += 1;
                        pos.1 = 0;
                    }
                    _ => {
                        pos.1 += 1;
                        continue;
                    }
                }
                if depth > 0 {
                    tokens.drain(..0);
                } else if depth == 0 {
                    break;
                }
            }
            if depth != 0 {
                return Err(crate::Error::UnmatchedStart(last_start));
            }
        }
        Ok(TokenStream(tokens))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case::add("+++", vec!{BaseOp::Add; 3})]
    #[case::sub("---", vec![BaseOp::Sub; 3])]
    #[case::mvl("<<<", vec![BaseOp::Mvl; 3])]
    #[case::mvr(">>>", vec![BaseOp::Mvr; 3])]
    //
    #[case::out("...", vec![BaseOp::Out; 3])]
    #[case::inp(",,,", vec![BaseOp::Inp; 3])]
    #[case::srt("[[[", vec![BaseOp::Srt; 3])]
    #[case::end("]]]", vec![BaseOp::End; 3])]
    #[case::mixed(",+++++[>+++++<-]>.",
        vec![
            BaseOp::Inp,
            BaseOp::Add,
            BaseOp::Add,
            BaseOp::Add,
            BaseOp::Add,
            BaseOp::Add,
            BaseOp::Srt,
            BaseOp::Mvr,
            BaseOp::Add,
            BaseOp::Add,
            BaseOp::Add,
            BaseOp::Add,
            BaseOp::Add,
            BaseOp::Mvl,
            BaseOp::Sub,
            BaseOp::End,
            BaseOp::Mvr,
            BaseOp::Out,
        ])]
    fn lex(#[case] program: &str, #[case] expected: Vec<BaseOp>) {
        let lexed = SourceProgram::new(String::from(program)).lex();
        let manual = TokenStream::new(expected);
        assert_eq!(
            lexed, manual,
            "Lexed string {program} doesn't match expected output\nRecieved output: {lexed:#?}\nExpected output: {manual:#?}"
        );
    }
}
