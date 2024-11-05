use std::io;
use std::path::Path;

pub fn check_if_file_exists(dir_to_move_to: &str, files_to_move: &[String]) -> bool {
    if dir_to_move_to.trim().is_empty() {
        return false;
    }
    // Check if the file exists
    let mut file_exists = false;
    for (_, file_name) in files_to_move.iter().enumerate() {
        let filename: String = file_name
            .chars()
            .skip(1)
            .take(file_name.len() - 2)
            .collect();
        let path = Path::new(&dir_to_move_to.to_string()).join(filename.clone());
        if path.metadata().is_ok() && path.is_file() {
            println!("{} already exists in this folder", filename);
            file_exists = true;
        }
    }
    let mut input = String::new();
    if file_exists {
        println!("Would you like to overwrite? y/N");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "y" {
                    return false;
                }
                return true;
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                return true;
            }
        }
    }
    return false;
}
