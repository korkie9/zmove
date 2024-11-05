use std::env;

pub fn set_destination(destination: String) -> Option<String> {
    if destination.trim() == ".." {
        if let Ok(current_dir) = env::current_dir() {
            if let Some(parent_dir) = current_dir.parent() {
                if let Some(parent_dir_str) = parent_dir.to_str() {
                    return Some(parent_dir_str.to_string());
                } else {
                    println!("Failed to convert previous directory to a UTF-8 string.");
                    return None;
                }
            } else {
                println!("No parent directory found");
                return None;
            }
        } else {
            println!("No parent directory found");
            return None;
        }
    }
    return Some(destination);
}
