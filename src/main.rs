#![allow(unused_imports)]

use std::path::PathBuf;

use clap::{Parser, Subcommand, Command};

#[derive(Parser)]
#[clap(about, version, author, long_about = None)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add numbers 
    Add { number_one: i32, number_two: i32 },
    /// Subtract numbers
    Subtract { number_one: i32, number_two: i32 },
    /// Multiply numbers
    Multiply { number_one: i32, number_two: i32 },
    /// Divide numbers
    Divide { number_one: i32, number_two: i32 },
}
    

fn main() {
     let value = Value::parse();

     match &value.command {
           Commands::Add { number_one, number_two } => {
                println!("The answer is: {}!", number_one + number_two)
           }
           Commands::Subtract { number_one, number_two } => {
                println!("The answer is: {}!", number_one - number_two)
           }
           Commands::Multiply { number_one, number_two } => {
                println!("The answer is: {}!", number_one * number_two)
           }
           Commands::Divide { number_one, number_two } => {
                println!("The answer is: {}!", number_one / number_two)
           }
      }
}