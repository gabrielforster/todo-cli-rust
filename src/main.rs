struct Todo {
    description: String,
    completed: bool,
}

impl Todo {
    fn new(description: String) -> Self {
        Self {
            description,
            completed: false,
        }
    }
    
    fn complete(&mut self) {
        self.completed = true;
    }

    fn to_string(&self) -> String {
        format!("{}", self.description)
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command = args[1].clone();

    if command == "list" {
        let current_todos: Vec<Todo> = std::fs::read_to_string("todos")
            .expect("Something went wrong reading the file")
            .lines()
            .map(|line| Todo::new(line.to_string()))
            .collect();

        for (index, todo) in current_todos.iter().enumerate() {
            println!("{} -> {}", index, todo.to_string());
        }
    }

    if command == "done" {
        let mut current_todos: Vec<Todo> = std::fs::read_to_string("todos")
            .expect("Something went wrong reading the file")
            .lines()
            .map(|line| Todo::new(line.to_string()))
            .collect();

        let done_index: usize = args[2].parse().expect("Error parsing index");

        current_todos[done_index].complete();

        let mut file_content = String::new();

        for todo in current_todos.iter() {
            file_content.push_str(&todo.to_string());
            file_content.push_str("\n");
        }

        std::fs::write("todos", file_content).expect("Error writing file");
    }

    if command == "new" {
        let mut current_todos: Vec<Todo> = std::fs::read_to_string("todos")
            .expect("Something went wrong reading the file")
            .lines()
            .map(|line| Todo::new(line.to_string()))
            .collect();

        let new_todo = Todo::new(args[2].clone());

        current_todos.push(new_todo);

        let mut file_content = String::new();

        for todo in current_todos.iter() {
            file_content.push_str(&todo.to_string());
            file_content.push_str("\n");
        }

        std::fs::write("todos", file_content).expect("Error writing file");
    }
}
