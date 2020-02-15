use std::io;

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
            Some(s) if s == &list_keyword => {
                println!("{:?}", todos);
            },
            Some(s) if s == &add_keyword => {
                let mut new_todo = String::new();

                for string in split_string[1..].iter() {
                  new_todo.push_str(&string.to_string());
                  new_todo.push_str(" ");
                }

                todos.push(new_todo);
            },
            Some(s) if s == &complete_keyword => {
                let idx = match split_string.get(1).unwrap().trim().parse::<usize>() {
                    Ok(num) =>  num,
                    Err(e) => { println!("{}", e); continue },
                };

                todos.remove(idx - 1);
            },
            _ => continue,
        }

        println!("{:?}", todos);
    }
}
