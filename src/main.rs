use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print!("Zmove expects 2 arguments\n");
        return;
    }

    let file_to_move = &args[1];
    let frag_of_dir_to_move_to = &args[2];

    // Check if current directory exists
    let list_to_check_cmd_string = if cfg!(target_os = "windows") {
        "dir"
    } else {
        "ls"
    };
    let list_to_check_cmd = Command::new("sh")
        .arg("-c")
        .arg(list_to_check_cmd_string)
        .output()
        .expect("failed to return to original directory");
    let string_to_check = String::from_utf8_lossy(&list_to_check_cmd.stdout);
    if string_to_check.contains(&args[2]) {
        let move_to_dir_cmd_string = if cfg!(target_os = "windows") {
            format!("move {} {}", &args[1], &args[2])
        } else {
            format!("mv {} {}", &args[1], &args[2])
        };

        let move_to_dir_cmd = Command::new("sh")
            .arg("-c")
            .arg(move_to_dir_cmd_string)
            .output()
            .expect("failed to return to original directory");
        if move_to_dir_cmd.status.success() {
            println!("Moved {} to {}\n", &args[1], args[2]);
        } else {
            print!("File not recognized\n");
        }
        return;
    }

    // Get availible zoxide directories
    let move_to_this_cmd_string = format!("zoxide query {}", frag_of_dir_to_move_to.trim());
    let move_to_dir_cmd = Command::new("sh")
        .arg("-c")
        .arg(move_to_this_cmd_string.clone())
        .output()
        .expect("failed to return to original directory");
    let dir_to_move_to = String::from_utf8_lossy(&move_to_dir_cmd.stdout);

    // Move file
    if move_to_dir_cmd.status.success() {
        let mv_cmd_string = if cfg!(target_os = "windows") {
            format!("move {} {}", file_to_move.trim(), dir_to_move_to)
        } else {
            format!("mv {} {}", file_to_move.trim(), dir_to_move_to)
        };
        let mv_cmd = Command::new("sh")
            .arg("-c")
            .arg(mv_cmd_string)
            .output()
            .expect("failed to return to original directory");

        if mv_cmd.status.success() {
            println!("Moved {} to {}\n", file_to_move, dir_to_move_to);
        } else {
            print!("File not recognized\n");
        }
    } else {
        print!("Path not recognized\n");
    }
}
