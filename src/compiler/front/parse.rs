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
//! This file is for the parsing of token streams
use crate::compiler::front::{BaseOp, Block, Node, TokenStream};
impl TokenStream {
    /// Parses the stream of tokens & turns it into an AST
    pub fn parse(&mut self) -> (Block, usize) {
        let mut program = Block::new(Vec::new());
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
#[cfg(test)]
mod tests {
    use super::{Block, Node};
    use rstest::rstest;
    #[rstest]
    #[case::add("+++", Block::new(vec![Node::Add; 3]))]
    #[case::sub("---", Block::new(vec![Node::Sub; 3]))]
    #[case::mvr(">>>", Block::new(vec![Node::Mvr; 3]))]
    #[case::mvl("<<<", Block::new(vec![Node::Mvl; 3]))]
    #[case::out("...", Block::new(vec![Node::Out; 3]))]
    #[case::inp(",,,", Block::new(vec![Node::Inp; 3]))]
    #[case::lop("[...]", Block::new(vec![Node::Lop(Block::new(vec![Node::Out; 3]))]))]
    #[case::mixed(",+++++[>+++++<-]>.",
            Block::new(vec![
                Node::Inp,
                Node::Add,
                Node::Add,
                Node::Add,
                Node::Add,
                Node::Add,
                Node::Lop(Block::new(vec![
                    Node::Mvr,
                    Node::Add,
                    Node::Add,
                    Node::Add,
                    Node::Add,
                    Node::Add,
                    Node::Mvl,
                    Node::Sub,
                ])),
                Node::Mvr,
                Node::Out,
            ]))]
    fn parsing(#[case] program: &str, #[case] expected: Block) {
        let parsed = crate::compiler::SourceProgram::new(String::from(program))
            .lex()
            .parse()
            .0;
        assert_eq!(
            parsed, expected,
            "Parsed string {program} doesn't match expected output\nReceived output: {parsed:#?}\nExpected output: {expected:#?}"
        );
    }
}
