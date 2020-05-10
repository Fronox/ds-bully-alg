mod common;

use common::bully::*;
use common::cli_functions::*;
use common::cli_models::*;
use common::models::*;
use std::io::prelude::*;
use structopt::StructOpt;

const HELP_TEXT: &str = "
* help - prints this message\n
* list - returns list of alive processes in the system, coordinator is marked with '*'\n
* kill <id> - kills process with <id> id, <id> is an integer value\n
* reload - reloads initial file, initiates new election\n
* exit - stops the system\n
";

fn run_cli() {
    let args = Cli::from_args();
    let path = args.path;
    let content = std::fs::read_to_string(&path).expect("could not read file");
    let mut storage = Storage::new();
    read_processes(content, &mut storage);
    let mut coordinator_id = bully_election(&mut storage, true);

    loop {
        let mut input_command = String::new();
        print!(">> ");
        let _ = std::io::stdout().flush();
        match std::io::stdin().read_line(&mut input_command) {
            Ok(_) => {
                let trimmed = input_command.trim_end();
                let arguments: Vec<&str> = trimmed.split(" ").collect();
                match arguments[0] {
                    "reload" => {
                        let content = std::fs::read_to_string(&path).expect("could not read file");
                        read_processes(content, &mut storage);
                        coordinator_id = bully_election(&mut storage, true);
                    }
                    "list" => {
                        let print = pretty_print_storage(&storage, coordinator_id);
                        println!("{}", print)
                    }
                    "kill" => match arguments.get(1) {
                        Some(id) => match id.parse::<Id>() {
                            Ok(int_id) => {
                                kill_process(&mut storage, int_id);
                                if int_id == coordinator_id && !storage.is_empty() {
                                    coordinator_id = bully_election(&mut storage, true);
                                }
                            }
                            Err(_) => println!("This doesn't look like an integer, try again"),
                        },
                        None => println!("Wrong usage\nTry passing an id of a process"),
                    },
                    "help" => println!("{}", HELP_TEXT),
                    "exit" => break,
                    _ => println!("Unknown command, try 'help' for known commands"),
                }
            }
            Err(exception) => panic!(exception),
        }
    }
}

fn main() {
    run_cli();
}
