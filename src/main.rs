use std::io;

fn list_todos(todos: &Vec<String>) {
    for (idx, todo) in todos.iter().enumerate() {
        println!("{}: {}", idx + 1, todo);
    }
}

fn add_todo(todos: &mut Vec<String>, string_vec: &Vec<&str>) {
    let new_todo = string_vec[1..].join(" ").trim().to_string();
    todos.push(new_todo);
}

fn complete_todo(todos: &mut Vec<String>, idx: Result<usize, std::num::ParseIntError>) -> &mut Vec<String> {
    match idx {
        Ok(i) => { todos.remove(i - 1); todos },
        Err(_) => { todos },
    }
}

fn main() {
    println!("Add your todos here!");

    let mut todos: Vec<String> = vec![];

    let add_keyword = String::from("ADD");
    let complete_keyword = String::from("COMPLETE");
    let list_keyword = String::from("LIST");

    loop {
        println!("either ADD [todo],  COMPLETE [index] or LIST");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line.");

        let split_string: Vec<&str> = input.split(" ").collect();
        let command = split_string.get(0);

        match command {
            Some(s) if s.trim() == &list_keyword => list_todos(&todos),
            Some(s) if s.trim() == &add_keyword => add_todo(&mut todos, &split_string),
            Some(s) if s.trim() == &complete_keyword => {
                complete_todo(&mut todos, split_string.get(1).unwrap().trim().parse::<usize>());
            },
            Some(s) => {
                println!("Missed command: {}", s)
            },
            None => println!("Error. "),
        }

        list_todos(&todos);
    }
}
