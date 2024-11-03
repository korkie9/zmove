use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print!("Zmove expects at least 2 arguments\n");
        return;
    }

    let files_to_move = &args[1..&args.len() - 1];
    let destination = &args[&args.len() - 1];
    let files_to_move_as_string = files_to_move.join(" ");

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
        .expect("An issue occured when trying to find the directory");
    let string_to_check = String::from_utf8_lossy(&list_to_check_cmd.stdout);

    //Move files if current directory exists
    if string_to_check.contains(destination) {
        let platform_cmd = if cfg!(target_os = "windows") {
            "move"
        } else {
            "mv"
        };
        let move_to_dir_cmd_string = format!(
            "{} {} {}",
            platform_cmd, files_to_move_as_string, destination
        );

        let move_to_dir_cmd = Command::new("sh")
            .arg("-c")
            .arg(move_to_dir_cmd_string)
            .output()
            .expect("An issue occured when moving the files");
        if move_to_dir_cmd.status.success() {
            println!("Moved {:?} to {}\n", files_to_move, destination);
        } else {
            print!("File not recognized\n");
        }
        return;
    }

    // Get availible zoxide directories
    let move_to_this_cmd_string = format!("zoxide query {}", destination.trim());
    let move_to_dir_cmd = Command::new("sh")
        .arg("-c")
        .arg(move_to_this_cmd_string.clone())
        .output()
        .expect("Failed to find to original directory");
    let dir_to_move_to = String::from_utf8_lossy(&move_to_dir_cmd.stdout);

    // Move file(s) to specified zoxide directory
    if move_to_dir_cmd.status.success() {
        let mv_cmd_string = if cfg!(target_os = "windows") {
            format!("move {} {}", files_to_move_as_string, dir_to_move_to)
        } else {
            format!("mv {} {}", files_to_move_as_string, dir_to_move_to)
        };
        let mv_cmd = Command::new("sh")
            .arg("-c")
            .arg(mv_cmd_string)
            .output()
            .expect("Failed to move files");

        if mv_cmd.status.success() {
            println!("Moved {} to {}\n", files_to_move_as_string, dir_to_move_to);
            return;
        }
        print!("File not recognized\n");
        return;
    } else {
        print!("Path not recognized\n");
    }
}
