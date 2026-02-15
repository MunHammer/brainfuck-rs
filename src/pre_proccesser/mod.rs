//! The pre-proccessing that can be used for interpretation & compilation
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
use bitcode::{Encode, Decode};
/// The base operation set, used in [`TokenStream`]
#[derive(Clone, Encode, Decode)]
pub enum BaseOp {
    /// The bf Add (**ADD** to current cell), equivalent to `+`
    Add,
    /// The bf Sub (**SUB**tract from current cell), equivalent to `-`
    Sub,
    /// The bf Mvr (**M**o**V**e **R**ight), equivalent to `>`
    Mvr,
    /// The bf Mvl (**M**o**V**e **L**eft), equivalent to `<`
    Mvl,
    /// The bf Srt (**S**ta**RT** the loop), equivalent to `[`
    Srt,
    /// The bf End (**END** the loop), equivalent to `]`
    End,
    /// The bf Out (**OUT**put the current cell), equivalent to `.`
    Out,
    /// The bf Inp (**INP**ut), equivalent to `,`
    Inp,
}
/// The node for ASTs, used in [`Block`]
#[derive(Debug, Encode, Decode)]
pub enum Node {
    Add,
    Sub,
    Mvr,
    Mvl,
    Out,
    Inp,
    Lop(Block),
}
/// A group of nodes for ASTs, uses the [`Node`]
#[derive(Debug, Encode, Decode)]
#[bitcode(recursive)]
pub struct Block(Vec<Node>);
/// A wrapper for a program
#[derive(Encode, Decode)]
pub struct SourceProgram(pub String);
/// A stream of tokens, uses [`BaseOp`]
#[derive(Encode, Decode)]
pub struct TokenStream(pub Vec<BaseOp>);
impl SourceProgram {
    /// Turns the source program into a stream of tokens & removes standard starting comments
    pub fn lex(&self) -> Result<TokenStream, &str> {
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
                _ => continue,
            });
        }
        // Removes comments like:
        /*
        [This is a brainfuck comment, you can put any char in here
        but the [] (loops) have to be matched like normal.]
        +++++
        */
        while let BaseOp::Srt = tokens.first().unwrap_or(&BaseOp::Add) {
            let mut depth: u8 = 0;
            for op in tokens.clone() {
                match op {
                    BaseOp::Srt => depth += 1,
                    BaseOp::End => depth -= 1,
                    _ => (),
                }
                if depth > 0 {
                    tokens.drain(..0);
                } else if depth == 0 {
                    break;
                }
            }
            if depth != 0 {
                return Err("Unmatched [");
            }
        }
        Ok(TokenStream(tokens))
    }
}
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
                    let res = TokenStream(tokens.clone().cloned().collect::<Vec<BaseOp>>()).parse();
                    for _ in 0..res.1 {
                        tokens.next();
                    }
                    res.0
                }),
                BaseOp::Out => Node::Out,
                BaseOp::Inp => Node::Inp,
                BaseOp::End => return (program, times_drained),
            });
        }
        (program, times_drained)
    }
}
impl Block {
    /// pushes an element to the bottom of the current branch of the tree
    pub fn push(&mut self, node: Node) {
        self.0.push(node);
    }
}
