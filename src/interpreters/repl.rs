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
#![cfg(feature = "repl")]
use rustyline::{
    Editor, Helper, Result,
    completion::Completer,
    highlight::Highlighter,
    hint::Hinter,
    history::{DefaultHistory, SearchDirection, SearchResult},
    validate::Validator,
};
pub fn repl() -> Result<()> {
    let mut rl: Editor<BrainfuckReplHelper, DefaultHistory> = Editor::new()?;
    rl.set_helper(Some(BrainfuckReplHelper));
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                // TODO: main repl logic
            }
            Err(error) => {
                eprintln!("error: {error:#?}");
                break;
            }
        }
    }
    Ok(())
}
struct BrainfuckReplHelper;

struct BrainfuckReplCompleter;
struct BrainfuckReplHinter;
struct BrainfuckReplHighlighter;
struct BrainfuckReplValidator;

impl BrainfuckReplHelper {
    pub(crate) fn completeness(prog: &str) -> i32 {
        prog.chars().fold(0, |acc, c| {
            acc + match c {
                '[' => 1,
                ']' => -1,
                _ => 0,
            }
        })
    }
}

impl Helper for BrainfuckReplHelper {}

impl Completer for BrainfuckReplHelper {
    type Candidate = String;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>)> {
        BrainfuckReplCompleter.complete(line, pos, ctx)
    }
    fn update(
        &self,
        line: &mut rustyline::line_buffer::LineBuffer,
        start: usize,
        elected: &str,
        cl: &mut rustyline::Changeset,
    ) {
        BrainfuckReplCompleter.update(line, start, elected, cl);
    }
}

impl Hinter for BrainfuckReplHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        BrainfuckReplHinter.hint(line, pos, ctx)
    }
}

impl Highlighter for BrainfuckReplHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
        BrainfuckReplHighlighter.highlight(line, pos)
    }
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> std::borrow::Cow<'b, str> {
        BrainfuckReplHighlighter.highlight_prompt(prompt, default)
    }
    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        BrainfuckReplHighlighter.highlight_hint(hint)
    }
    fn highlight_candidate<'c>(
        &self,
        candidate: &'c str, // FIXME should be Completer::Candidate
        completion: rustyline::CompletionType,
    ) -> std::borrow::Cow<'c, str> {
        BrainfuckReplHighlighter.highlight_candidate(candidate, completion)
    }
    fn highlight_char(&self, line: &str, pos: usize, kind: rustyline::highlight::CmdKind) -> bool {
        BrainfuckReplHighlighter.highlight_char(line, pos, kind)
    }
}

impl Validator for BrainfuckReplHelper {
    fn validate(
        &self,
        ctx: &mut rustyline::validate::ValidationContext,
    ) -> Result<rustyline::validate::ValidationResult> {
        BrainfuckReplValidator.validate(ctx)
    }
}

impl Completer for BrainfuckReplCompleter {
    type Candidate = String;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>)> {
        let mut candidates = Vec::new();
        let completeness = BrainfuckReplHelper::completeness(line);
        if completeness == 0 {
            return Ok((pos, vec![]));
        } else if completeness.is_positive() {
            candidates.push("]".repeat(completeness as usize));
        } else {
            return Ok((pos, Vec::with_capacity(0)));
        }
        if let Some(SearchResult { idx: _, entry, pos }) =
            ctx.history()
                .search("", ctx.history().len(), SearchDirection::Reverse)?
        {
            dbg!(pos);
            candidates.push(entry.into());
        }
        Ok((pos, candidates))
    }
}

impl Hinter for BrainfuckReplHinter {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        BrainfuckReplCompleter
            .complete(line, pos, ctx)
            .unwrap()
            .1
            .get(0)
            .cloned()
        // HistoryHinter::new().hint(line, pos, ctx)
    }
}

impl Highlighter for BrainfuckReplHighlighter {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
        let _ = pos;
        let mut output = String::new();
        for c in line.chars() {
            output.push_str(match c {
                '[' | ']' => "\x1b[96m", // bright cyan
                '+' => "\x1b[92m",       // bright green
                '-' => "\x1b[91m",       // bright red
                '<' => "\x1b[94m",       // bright blue
                '>' => "\x1b[33m",       // yellow
                _ => "",
            });
            output.push(c);
            output.push_str("\x1b[39m");
        }
        std::borrow::Cow::Owned(output)
    }
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> std::borrow::Cow<'b, str> {
        std::borrow::Cow::Owned(String::from("\x1b[95m>>>\x1b[0m "))
    }
    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        std::borrow::Cow::Owned(format!("\x1b[2m{hint}\x1b[0m"))
    }
    fn highlight_char(&self, line: &str, pos: usize, kind: rustyline::highlight::CmdKind) -> bool {
        match line.chars().nth(pos.saturating_sub(1)) {
            None => false,
            Some(c) => match c {
                '[' | ']' | '+' | '-' | '<' | '>' => true,
                _ => false,
            },
        }
    }
}

impl Validator for BrainfuckReplValidator {
    fn validate(
        &self,
        ctx: &mut rustyline::validate::ValidationContext,
    ) -> Result<rustyline::validate::ValidationResult> {
        let completeness = BrainfuckReplHelper::completeness(ctx.input());
        if completeness > 0 {
            Ok(rustyline::validate::ValidationResult::Incomplete)
        } else if completeness < 0 {
            Ok(rustyline::validate::ValidationResult::Invalid(Some(
                String::from("Unmatched ] character"),
            )))
        } else {
            Ok(rustyline::validate::ValidationResult::Valid(None))
        }
    }
}
