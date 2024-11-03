mod current_directory_cmd;
use current_directory_cmd::check_and_move_from_current_dir;

mod relative_directory_cmd;
use relative_directory_cmd::check_and_move_from_relative_dir;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print!("Zmove expects at least 1 file or directory and a target directory \n");
        return;
    }

    let files_to_move = &args[1..&args.len() - 1];
    let destination = &args[&args.len() - 1];
    let files_to_move_as_string = files_to_move.join(" ");

    if check_and_move_from_current_dir(destination, &files_to_move_as_string, files_to_move) {
        return;
    };
    check_and_move_from_relative_dir(destination, &files_to_move_as_string);
}
