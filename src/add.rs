use crate::cli::AddArgs;
use crate::todo::{Todo, load_todos, save_todos};

pub fn handle(args: &AddArgs) {
    let mut todos = load_todos();

    let new_todo = Todo {
        name: args.name.clone(),
        description: args.desc.clone(),      
        due_date: args.due.clone(),         
        status: args.status,
    };

    todos.push(new_todo);
    save_todos(&todos);

    println!("✅ Todo added and saved.");
}
