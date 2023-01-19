#![allow(non_camel_case_types)]

use clap::{Parser, Subcommand, Command};
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
     } else {
          let mut equation = String::new();
          println!("Enter an equation: ");
          std::io::stdin().read_line(&mut equation).expect("Failed to read line");
          println!("{}", parse_equation(&equation.trim()));
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
               Some("*") => result *= parse_number(tokens),
               Some("/") => result /= parse_number(tokens),
               Some("+") | Some("-") => return result,
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

/* 
fn parse_expression(tokens: &mut SplitWhitespace) -> i32 {
     let mut result = parse_term(tokens);

     loop {
           let token = tokens.next(); 
           match token {
                Some("+") => result += parse_term(tokens),
                Some("-") => result -= parse_term(tokens),
                _ => break,
           }
     }

     result
}

fn parse_term(tokens: &mut SplitWhitespace) -> i32 {
     let mut result = parse_factor(tokens);

     loop {
           let token = tokens.next(); 
           match token {
                Some("*") => result *= parse_factor(tokens),
                Some("/") => result /= parse_factor(tokens),
                _ => break,
           }
     }

     result
}

fn parse_factor(tokens: &mut SplitWhitespace) -> i32 {
     let token = tokens.next().unwrap();
     match token {
          "+" => parse_factor(tokens),
          "-" => -parse_factor(tokens),
          "*" => parse_factor(tokens) * parse_factor(tokens),
          "/" => parse_factor(tokens) / parse_factor(tokens),
          _ => token.parse().unwrap(),
     }
}
*/