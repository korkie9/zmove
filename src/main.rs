mod current_directory_cmd;
use current_directory_cmd::check_and_move_from_current_dir;

mod relative_directory_cmd;
use relative_directory_cmd::check_and_move_from_relative_dir;

mod set_destination;
use set_destination::set_destination;

mod file_checker;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // For version query
    if args.len() == 2 && (args[1] == "-v" || args[1] == "--version") {
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.len() < 3 {
        print!("Zmove expects at least 1 file or directory and a target directory \n");
        return;
    }

    let unformated_files_to_move = &args[1..&args.len() - 1];
    let files_to_move: Vec<String> = unformated_files_to_move
        .iter()
        .map(|file| format!("'{}'", file))
        .collect();
    let files_to_move_as_string = files_to_move.join(" ");
    let destination: String;
    match set_destination(args[&args.len() - 1].clone()) {
        Some(dest) => {
            destination = format!("'{}'", Some(dest).unwrap());
        }
        None => {
            return;
        }
    }

    if check_and_move_from_current_dir(&destination, &files_to_move_as_string, &files_to_move) {
        return;
    };
    check_and_move_from_relative_dir(&destination, &files_to_move_as_string, &files_to_move);
}
