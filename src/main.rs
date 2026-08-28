use std::io;
struct Task {
    name: String,
    completed: bool,
}

fn main() {
    println!("TASK MANAGER");

    let mut tasks: Vec<Task> = vec![];

    loop {
        let mut choice = String::new();

        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Quit");
        println!("Choose an option:");
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {

            "1" =>{ println!("Add task");

        let mut input = String::new();

        println!("Enter a task");

        io::stdin().read_line(&mut input).unwrap();

        let task = Task {
            name: input.trim().to_string(),
            completed: false,
        };
        tasks.push(task);

        println!(" Task added!");
        }

            "2"=> {println!("show tasks");
        
        for (index, task) in tasks.iter().enumerate()
        {
            let status = if task.completed { "[✓]" } else { "[ ]" };
            println!("{}. {} {}", index + 1, status, task.name);
        }
        }
          "3" => {
              let mut input = String::new();

              println!("Enter task number to complete:");
              io::stdin().read_line(&mut input).unwrap();

              let number = match input.trim().parse::<usize>() {
                Ok(number) => number,
                Err(_) => {
                    println!("Please enter a number");
                    continue;
                }
              };
              
              if number == 0  ||number > tasks.len() {
                println!("Invalid task number.");
                continue;
                }
                let  index = number - 1;
                tasks[index].completed = true;
                print!("Task completed!");
            
              
          }

           "4" => {
    let mut input = String::new();

    println!("Enter task number to delete:");

    io::stdin().read_line(&mut input).unwrap();

    let number = match input.trim().parse::<usize>() {
        Ok(number) => number,
        Err(_) => {
            println!("Please enter a number");
            continue;
        }
    };

    if number == 0 || number > tasks.len() { 

        println!("Invalid task number.");
        continue;
    }
    let index = number - 1;

    tasks.remove(index);

    println!("Task deleted!");
}
           "5" => {break;}
            _ =>                
             println!("Invalid option"),
        }
    }
}
    
