use crate::todo::{load_todos, save_todos};
use crate::cli::DeleteArgs;

pub fn handle(args: &DeleteArgs) {
    let mut todos = load_todos();

    if args.id == 0 || args.id > todos.len() {
        println!("❌ Invalid ID: {}", args.id);
        return;
    }

    let removed = todos.remove(args.id - 1); 
    save_todos(&todos);

    println!("🗑️ Deleted: {} - {}", removed.name, removed.description);
}
