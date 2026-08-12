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
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
/// The module in which all of the command line interface logic is stored
#[cfg(feature = "cli")]
mod cli {
    use brainfuck_rs::compiler;
    #[cfg(feature = "interpret")]
    use brainfuck_rs::interpreters;
    use clap::{Parser, Subcommand, ValueEnum};
    use std::fs;
    use std::path::PathBuf;
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
    /// The main subcommands
    #[derive(Subcommand, Clone)]
    enum Commands {
        /// Builds the file into a storable format
        Build {
            /// The input to compile
            input: PathBuf,
            /// The level at which the compiler stops & emits it's current code
            #[arg(short, long, value_enum, default_value_t = Emit::Byte)]
            emit: Emit,
            /// The output file
            #[arg(short, long, default_value = "")]
            output: PathBuf,
        },
        /// Runs the file, whether it be:
        /// - Raw brainfuck,
        /// - Written memory or
        /// - Bytecode
        ///
        /// Can't run machine code (just run it directly)
        Run {
            /// The input to interpret
            input: PathBuf,
        },
    }
    /// The possible optimisation values
    #[derive(ValueEnum, Clone, Debug)]
    enum Emit {
        /// Token Stream
        Stream,
        /// Abstract Syntax Tree
        Tree,
        /// Bytecode
        Byte,
        /// Optimised Bytecode
        ByteOptimised,
        /// LLVM IR
        Llvm,
        /// Machine Code
        Machine,
        /// Optimised Machine Code
        MachineOptimised,
    }
    /// The main parsing of already parsed Command line args & running of other functions & things
    pub fn run() -> brainfuck_rs::Result<()> {
        let cli = Cli::parse();
        if cli.long_license {
            println!(
                "{}",
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/COPYING"))
            );
            return Ok(());
        } else if cli.short_license {
            println!(
                "brainfuck-rs: A brainfuck CLI interpreter & compiler\nCopyright (C) 2026  Mun_Hammer\n\nThis file is part of brainfuck-rs\n\nbrainfuck-rs is free software: you can redistribute it and/or modify\nit under the terms of the GNU General Public License as published by\nthe Free Software Foundation, either version 3 of the License, or\n(at your option) any later version.\n\nbrainfuck-rs is distributed in the hope that it will be useful,\nbut WITHOUT ANY WARRANTY; without even the implied warranty of\nMERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\nGNU General Public License for more details.\n\nYou should have received a copy of the GNU General Public License\nalong with brainfuck-rs.  If not, see <https://www.gnu.org/licenses/>.\nsee --license for the full license"
            );
            return Ok(());
        }
        match cli.command.clone() {
            None => {
                #[cfg(feature = "repl")]
                interpreters::repl::repl().unwrap();
                // interpreters::repl()?;
                #[cfg(not(feature = "repl"))]
                panic!("No valid command, Feature not inclded in compilation")
            }
            Some(Commands::Run { input: _ }) => {
                #[cfg(feature = "interpret")]
                {
                    todo!();
                }
                #[cfg(not(feature = "interpret"))]
                panic!("Feature not included in compilation")
            }
            Some(Commands::Build {
                input,
                emit,
                output,
            }) => {
                let mut output = output;
                if output == PathBuf::new() {
                    output = PathBuf::from(input.file_prefix().unwrap());
                } else {
                    output.set_extension(match emit {
                        Emit::Stream => ".bstr",
                        Emit::Byte | Emit::ByteOptimised => ".bfc",
                        _ => "",
                    });
                }
                let source = compiler::SourceProgram(fs::read_to_string(input).unwrap());
                let mut source = source.lex();
                if let Emit::Stream = emit {
                    let mut encoded = vec![1];
                    encoded.append(&mut bitcode::encode(&source).unwrap());
                    fs::write(output, encoded).unwrap();
                    return Ok(());
                }
                let source = source.parse().0;
                if let Emit::Stream = emit {
                    let mut encoded = vec![2];
                    encoded.append(&mut bitcode::encode(&source).unwrap());
                    fs::write(output, encoded).unwrap();
                }
            }
        }
        Ok(())
    }
}
fn main() -> brainfuck_rs::Result<()> {
    #[cfg(feature = "cli")]
    cli::run()?;
    #[cfg(not(feature = "cli"))]
    compile_error!("Can't run this program without CLI");
    Ok(())
}
