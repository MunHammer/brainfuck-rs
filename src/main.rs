#![doc = include_str!("../README.md")]
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
use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::path::PathBuf;
mod interpreters;
mod pre_proccesser;
/// The command line argumaent parser
#[derive(Parser, Clone)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Shows the full license
    #[arg(long = "license")]
    long_license: bool,
    /// Prints the license the program is under
    #[arg(short = 'l')]
    short_license: bool,
}
#[derive(Subcommand, Clone)]
enum Commands {
    /// Builds the file into a storable format
    Build {
        /// The input to compile or interpret
        input: PathBuf,
        /// The level of optimisation & Compilation
        #[arg(short='O', long, value_enum, default_value_t = Optimisation::Byte)]
        optimisation: Optimisation,
        /// The output file
        #[arg(short, long, default_value = "")]
        output: PathBuf,
    },
    /// Builds & then runs the file
    Run {
        /// The input to compile or interpret
        input: PathBuf,
        /// The level of optimisation & Compilation
        #[arg(short='O', long, value_enum, default_value_t = Optimisation::Byte)]
        optimisation: Optimisation,
        /// The output file
        #[arg(short, long, default_value = "")]
        output: PathBuf,
    },
}
/// The possible optimisation values
#[derive(ValueEnum, Clone, Debug)]
enum Optimisation {
    /// Raw interpretation
    Raw,
    /// Convert to stream first
    Stream,
    /// Convert to AST first
    Tree,
    /// Optimise the tree
    TreeOptimised,
    /// Convert to Bytecode
    Byte,
    /// Optimise the Bytecode
    ByteOptimised,
    /// Convert to machine code
    Machine,
    /// Optimise the machine code
    MachineOptimised,
}
/// The main parsing of already parsed Command line args & running of other functions & things
fn main() {
    let cli = Cli::parse();
    if cli.long_license {
        println!(
            "{}",
            fs::read_to_string("/home/kydenj0892/Programming/RustProjects/rust_brainfuck/COPYING")
                .unwrap()
        );
        return;
    } else if cli.short_license {
        println!(
            "brainfuck-rs: A brainfuck CLI interpreter & compiler\nCopyright (C) 2026  Mun_Hammer\n\nThis file is part of brainfuck-rs\n\nbrainfuck-rs is free software: you can redistribute it and/or modify\nit under the terms of the GNU General Public License as published by\nthe Free Software Foundation, either version 3 of the License, or\n(at your option) any later version.\n\nbrainfuck-rs is distributed in the hope that it will be useful,\nbut WITHOUT ANY WARRANTY; without even the implied warranty of\nMERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\nGNU General Public License for more details.\n\nYou should have received a copy of the GNU General Public License\nalong with brainfuck-rs.  If not, see <https://www.gnu.org/licenses/>.\nsee --license for the full license"
        );
        return;
    }
    match cli.command.clone() {
        None => {
            interpreters::repl();
        }
        Some(
            Commands::Run {
                input,
                optimisation,
                output,
            }
            | Commands::Build {
                input,
                optimisation,
                output,
            },
        ) => {
            let mut output = output;
            if output == PathBuf::new() {
                output = PathBuf::from(input.file_prefix().unwrap());
            } else {
                output.set_extension(match optimisation {
                    Optimisation::Raw => ".b",
                    Optimisation::Stream => ".bstr",
                    Optimisation::Tree | Optimisation::TreeOptimised => ".btre",
                    Optimisation::Byte | Optimisation::ByteOptimised => ".bfc",
                    _ => "",
                });
            }
            let source = pre_proccesser::SourceProgram(fs::read_to_string(input).unwrap());
            if let Optimisation::Raw = optimisation {
                if let Some(Commands::Run {
                    input: _,
                    optimisation: _,
                    output: _,
                }) = cli.command
                {
                    source.interpret().unwrap();
                }
                let mut encoded = vec![0];
                encoded.append(&mut bitcode::encode(&source).unwrap());
                fs::write(output, encoded).unwrap();
                return;
            }
            let mut source = source.lex().unwrap();
            if let Optimisation::Stream = optimisation {
                if let Some(Commands::Run {
                    input: _,
                    optimisation: _,
                    output: _,
                }) = cli.command
                {
                    // source.interpret(100).unwrap();
                    todo!();
                }
                let mut encoded = vec![1];
                encoded.append(&mut bitcode::encode(&source).unwrap());
                fs::write(output, encoded).unwrap();
                return;
            }
            let source = source.parse().0;
            if let Optimisation::Stream = optimisation {
                if let Some(Commands::Run {
                    input: _,
                    optimisation: _,
                    output: _,
                }) = cli.command
                {
                    // source.interpret(100).unwrap();
                    todo!();
                }
                let mut encoded = vec![2];
                encoded.append(&mut bitcode::encode(&source).unwrap());
                fs::write(output, encoded).unwrap();
            }
        }
    }
}
