use std::process::Command;

pub fn check_and_move_from_relative_dir(destination: &str, files_to_move_as_string: &String) {
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
