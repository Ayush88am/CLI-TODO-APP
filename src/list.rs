use crate::todo::load_todos;

pub fn handle() {
    let todos = load_todos();

    if todos.is_empty() {
        println!();
        println!("📭 No todos found.");
        println!();

    } else {
        println!("📋 Your Todos:");
        println!();
        for (i, todo) in todos.iter().enumerate() {
            let status_icon = if todo.status { "✅" } else { "❌" };
            println!(
                "{}. {} - {} (due {}) [{}]",
                i + 1,
                todo.name,
                todo.description,
                todo.due_date,
                status_icon
            );
            println!();
            println!();

        }
    }
}
