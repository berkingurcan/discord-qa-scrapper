use std::fs;

fn main() {
    let dirs_to_create = vec![
        "outputs",
        "outputs/chat",
        "outputs/chat_archived_threads",
        "outputs/active_threads",
        "outputs/archived_threads",
    ];

    for dir in dirs_to_create {
        if let Err(e) = fs::create_dir_all(dir) {
            eprintln!("Error creating directory {}: {}", dir, e);
        } else {
            println!("Created directory: {}", dir);
        }
    }
}
