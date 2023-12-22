use rand::prelude::*;
use clap::{Parser, Subcommand};

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz"; 
const SYMBOLS: &str = r"!@#%^&*()-_=+[]{}\|/?.,<>;:";
const NUMBERS: &str = "0123456789";

#[derive(Parser)]
#[command(author, version)]
struct Args
{
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Program for generating passwords.
#[derive(Subcommand)]
enum Commands
{
    /// Generate password
    Generate
    {
        /// Length of the password
        #[arg(long, default_value_t = 16)]
        length: u16,

        /// Exclude numbers
        #[arg(short, long)]
        numbers: bool,

        /// Exclude letters
        #[arg(short, long)]
        letters: bool,

        /// Exclude symbols
        #[arg(short, long)]
        symbols: bool
    },
}

#[repr(u8)]
enum Flags
{
    NUMBER,
    LETTER,
    SYMBOL
}

fn generate(length: u16, numbers: bool, letters: bool, symbols: bool) -> String
{
    let mut rng = thread_rng();
    let mut result: String = Default::default();
    let mut flags: Vec<Flags> = Vec::new();

    if numbers == false
    {
        flags.push(Flags::NUMBER);
    }

    if letters == false
    {
        flags.push(Flags::LETTER);
    }

    if symbols == false
    {
        flags.push(Flags::SYMBOL);
    }

    for _ in 0..length  
    {
        match flags.choose(&mut rng) 
        {
            Some(Flags::NUMBER) => result.push(NUMBERS.chars().choose(&mut rng).unwrap()),
            Some(Flags::LETTER) => result.push(LETTERS.chars().choose(&mut rng).unwrap()),
            Some(Flags::SYMBOL) => result.push(SYMBOLS.chars().choose(&mut rng).unwrap()),
            _ => {}
        }
    }

    return result;
}

fn main() 
{
    let args = Args::parse();

    match &args.command 
    {
        Some(Commands::Generate { length, numbers, letters, symbols }) =>
        {
            println!("{}", generate(*length, *numbers, *letters, *symbols));
        }
        None => {}
    }
}

#[test]
fn check_length()
{
    assert_eq!(generate(16, false, false, false).len(), 16);
}

#[test]
fn check_exclude_numbers()
{
    let password: String = generate(16, true, false, false);
    assert_eq!(password.find(NUMBERS), None);
}

#[test]
fn check_exclude_letters()
{
    let password: String = generate(16, false, true, false);
    assert_eq!(password.find(LETTERS), None);
}

#[test]
fn check_exclude_symbols()
{
    let password: String = generate(16, false, false, true);
    assert_eq!(password.find(SYMBOLS), None);
}
