mod bridge;

use palimpsest::compare;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.as_slice() {
        [command] if command == "compare" => match compare() {
            Ok(result) => {
                println!(
                    "E001 - same initial world except claim visibility: {}",
                    result.world_equal_except_mask
                );
                println!("Visible Claim: {:#?}", result.visible);
                println!("Masked Claim: {:#?}", result.masked);
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("E001 could not run: {error:?}");
                ExitCode::FAILURE
            }
        },
        [command] if command == "bridge" => print_bridge(None),
        [command, visibility] if command == "bridge" && visibility == "visible" => {
            print_bridge(Some(true))
        }
        [command, visibility] if command == "bridge" && visibility == "masque" => {
            print_bridge(Some(false))
        }
        _ => {
            eprintln!("Usage: palimpsest compare | bridge [visible|masque]");
            ExitCode::from(2)
        }
    }
}

fn print_bridge(choice: Option<bool>) -> ExitCode {
    let variants: &[bool] = match choice {
        Some(true) => &[true],
        Some(false) => &[false],
        None => &[true, false],
    };
    println!("E005 / LE PONT : DEUX DEMANDES, UN SEUL CRENEAU DE TRAVAIL");
    println!("Borin veut reparer le pont. Ciro veut renforcer l'abri.");
    println!("L'acces au Claim est la seule intervention. E001 prend les decisions.");
    for &visible in variants {
        println!();
        match bridge::run(visible) {
            Ok(trial) => bridge::print_trial(&trial),
            Err(error) => {
                eprintln!("E005 n'a pas pu s'executer: {error:?}");
                return ExitCode::FAILURE;
            }
        }
    }
    ExitCode::SUCCESS
}
