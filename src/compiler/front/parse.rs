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
use crate::compiler::front::{BaseOp, Block, Node, TokenStream};
impl TokenStream {
    /// Parses the stream of tokens & turns it into an AST
    pub fn parse(&mut self) -> (Block, usize) {
        let mut program = Block(Vec::new());
        let mut tokens = self.0.iter();
        let mut times_drained = 0;
        while let Some(token) = tokens.next() {
            times_drained += 1;
            program.push(match token {
                BaseOp::Add => Node::Add,
                BaseOp::Sub => Node::Sub,
                BaseOp::Mvr => Node::Mvr,
                BaseOp::Mvl => Node::Mvl,
                BaseOp::Srt => Node::Lop({
                    let res =
                        TokenStream::new(tokens.clone().cloned().collect::<Vec<BaseOp>>()).parse();
                    for _ in 0..res.1 {
                        tokens.next();
                    }
                    res.0
                }),
                BaseOp::Out => Node::Out,
                BaseOp::Inp => Node::Inp,
                BaseOp::End => return (program, times_drained),
                BaseOp::CharCount => Node::CharCount,
                BaseOp::NewLine => Node::NewLine,
            });
        }
        (program, times_drained)
    }
}
