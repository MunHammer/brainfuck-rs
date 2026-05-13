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
//! This module defines all of the objects that are used for the front end of the compiler
//! *also for the extra functions such as `::new` that just make life easier*
use bitcode::{Decode, Encode};
/// The base operation set, used in [`TokenStream`]
#[derive(Debug, Clone, Encode, Decode, PartialEq)]
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
    /// Counts characters for error messages
    CharCount,
    /// Counts line numers for error messages
    NewLine,
}
/// The node for ASTs, used in [`Block`]
#[derive(Debug, Encode, Decode, PartialEq, Clone)]
pub enum Node {
    /// The bf Add (**ADD** to current cell), equivalent to `+`
    Add,
    /// The bf Sub (**SUB**tract from current cell), equivalent to `-`
    Sub,
    /// The bf Mvr (**M**o**V**e **R**ight), equivalent to `>`
    Mvr,
    /// The bf Mvl (**M**o**V**e **L**eft), equivalent to `<`
    Mvl,
    /// The bf Out (**OUT**put the current cell), equivalent to `.`
    Out,
    /// The bf Inp (**INP**ut), equivalent to `,`
    Inp,
    /// The bf Inp (**LO**o**P**), equivalent to `[Block]`
    Lop(Block),
    /// Extra to count amount of chars for error
    CharCount,
    /// Extra to count amount of lines for errors
    NewLine,
}
/// A group of nodes for ASTs, uses the [`Node`]
#[derive(Debug, Encode, Decode, Default, PartialEq, Clone)]
#[bitcode(recursive)]
pub struct Block(pub Vec<Node>);
/// A wrapper for a program
#[derive(Encode, Decode, Default)]
pub struct SourceProgram(pub String);
/// A stream of tokens, uses [`BaseOp`]
#[derive(Debug, Encode, Decode, PartialEq)]
pub struct TokenStream(pub Vec<BaseOp>);

// The functions & stuff go here

impl Block {
    /// pushes an element to the bottom of the current branch of the tree
    pub fn push(&mut self, node: Node) {
        self.0.push(node);
    }
    /// Creates a new instance of the [`Block`] struct
    #[must_use]
    pub fn new(vect: Vec<Node>) -> Self {
        Self(vect)
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl TokenStream {
    /// A QOL method that pushes an element to the inner vector
    pub fn push(&mut self, op: BaseOp) {
        self.0.push(op);
    }
    /// Creates a new instance of the [`TokenStream`] struct
    #[must_use]
    pub fn new(vect: Vec<BaseOp>) -> Self {
        Self(vect)
    }
}

impl SourceProgram {
    /// Creates a new instance of the [`SourceProgram`] struct
    #[must_use]
    pub fn new(source: String) -> Self {
        Self(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    fn block_push() {
        let mut manual = Block::default();
        for _ in 0..3 {
            manual.push(Node::Add);
        }
        assert_eq!(
            manual,
            Block::new(vec![Node::Add; 3]),
            "Failed to push {:#?} 3 times to {:#?}\nExpected output: {:#?}\nOutput recieved: {manual:#?}",
            Node::Add,
            Block::default(),
            Block::new(vec![Node::Add; 3]),
        );
    }
    #[rstest]
    fn stream_push() {
        let mut manual = TokenStream::default();
        for _ in 0..3 {
            manual.push(BaseOp::Add);
        }
        assert_eq!(
            manual,
            TokenStream::new(vec![BaseOp::Add; 3],),
            "Failed to push {:#?} 3 times to {:#?}\nExpected output: {:#?}\nOutput recieved: {manual:#?}",
            BaseOp::Add,
            TokenStream::default(),
            TokenStream::new(vec![BaseOp::Add; 3],),
        );
    }
}
