#![allow(non_camel_case_types)]

use clap::{Parser, Subcommand};
use std::str::SplitWhitespace;


#[derive(Parser)]
#[clap(about, version, author, long_about = None)]
struct Value {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// calculate numbers 
    calc { 
     #[clap(name = "EQUATION")]
     equation: String, 
     },
}


fn main() {
     let value = Value::parse();

     if let Some(Commands::calc { equation }) = value.command {
          println!("{}", parse_equation(&equation))
     } 
}

fn parse_equation(equation: &str) -> i32 {
     let mut tokens = equation.split_whitespace();
     let mut result = parse_token(&mut tokens);
     
     while let Some(token) = tokens.next() {
          match token {
               "+" => result += parse_token(&mut tokens),
               "-" => result -= parse_token(&mut tokens),
               _ => break,
          } 
     }

     result
}

fn parse_token(tokens: &mut SplitWhitespace) -> i32 {
     let mut result = parse_number(tokens);

     loop {
          let token = tokens.next();
          match token {
               Some("*") => result *= parse_token(tokens),
               Some("/") => result /= parse_token(tokens),
               Some("+") => result += parse_token(tokens),
               Some("-") => result -= parse_token(tokens),
               _ => break,
          }
     }
     result
}

fn parse_number(tokens: &mut SplitWhitespace) -> i32 {
     let token = tokens.next().unwrap();
     match token {
          "+" => parse_number(tokens),
          "-" => -parse_number(tokens),
          _ => token.parse().unwrap(),
     }
}
