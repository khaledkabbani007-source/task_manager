use std::io;

fn main() {
    println!("TASK MANAGER");

    let mut tasks: Vec<String> = vec![];

    loop {
        let mut choice = String::new();

        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Quit");
        println!("Choose an option:");
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => println!("Add task"),
            "2"=> println!("show tasks"),
            "3" => break,
            _ => println!("Invalid option"),
        }
    }
}
    s
