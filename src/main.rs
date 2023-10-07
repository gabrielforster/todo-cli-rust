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

    fn uncomplete(&mut self) {
        self.completed = false;
    }

    fn to_string(&self) -> String {
        format!("{} - Completed: {}", self.description, self.completed)
    }
}


fn main() {
    let mut todo = Todo::new("Commit code".to_string());
    println!("Todo: {}", todo.to_string());
    todo.complete();
    println!("Todo: {}", todo.to_string());
    todo.uncomplete();
    println!("Todo: {}", todo.to_string());
}
