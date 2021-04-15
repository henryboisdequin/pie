//! Pie is an algebra calculator. It takes an input (linear equation or combining like terms) and
//! turns it into a stream of tokens to an ast to eventually the final answer. The way I am doing this (with an ast)
//!  allows for my calculator to give a reason why the algorithm came up with that solution. It will be
//! able to give in-depth steps on how it has done it which is something very few calculators do today.
//! This project can be expanded to work with more algebraic problems, word problems, geometry problems,
//! or statistical problems and graphing.

#![warn(missing_docs, rust_2018_idioms, missing_debug_implementations)]

/// Abstract syntax tree handler for Pie.
pub mod ast;
/// Token handler for Pie.
pub mod lexer;

fn run(input: String) {
    todo!();
}
