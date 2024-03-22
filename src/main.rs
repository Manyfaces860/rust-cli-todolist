use std::io;
use std::collections::HashMap;
use colored::*;

#[derive(Debug)]
struct List {
    tasks : Vec<String>,
    completed : Vec<String>
}

fn main() {
    println!("{}","welcome to To Do App".bright_purple());
    println!("{}","\n Helpfull Commands".cyan());
    println!("{}","For showing TodoList : use SHOW <owner_name>".green());
    println!("{}","For marking completed tasks : use MARK <owner_name> <task_name>".yellow());
    println!("{}","For Quitting between Tasks : use BREAK for quitting between task which will take you to creating another Todolist".white());
    println!("{}","For Quitting between Naming : use BREAK for quitting between naming to quit the program".red());


    let mut todolist : HashMap<String, List> = HashMap::new();
    let mut name_input : String = String::new();
    let mut task_input : String = String::new();
    
    loop {
        
        print!("your name : ");
        name_input = take_input(name_input);
    
        if name_input == "BREAK" {
            return ()
        } else if name_input == "COMMAND" {
            run_commands(&mut todolist , &name_input);
            name_input = empty_input(name_input);
            continue;
        }
    
        loop {

            let contains_key  = todolist.contains_key(&name_input);
            println!("add tasks: ");
            task_input = take_input(task_input);

            if task_input == "BREAK" {
                break;
            } else if task_input == "COMMAND" {
                run_commands(&mut todolist , &name_input);
                task_input = empty_input(task_input);
                continue;
            }
            
            add_tasks_for_user(&task_input , &name_input , contains_key , &mut todolist);
            
            task_input = empty_input(task_input);
            
        }

        name_input = empty_input(name_input);
        task_input = empty_input(task_input);
}
}

fn take_input(mut input_to_return : String) -> String {
    io::stdin().read_line(&mut input_to_return).expect("no input was here!");
    String::from(input_to_return.trim())
}

fn empty_input(mut input_to_return : String) -> String {
    input_to_return = String::from("");
    input_to_return
}

fn add_tasks_for_user(task_input : &String , name_input : &String , key_bool : bool , mut todolist : &mut HashMap<String, List>) {

        if key_bool {

            let user_tasks = todolist.get_mut(name_input).unwrap();
            user_tasks.tasks.push(task_input.clone());
            
        } else {

            todolist.insert(name_input.clone(), List { tasks : vec![task_input.clone()] , completed : vec![]});

        }

}

fn mark_completed_task(mut todolist : &mut HashMap<String, List> , command : &String) {
    let command_parameters : Vec<&str> = command.split(' ').collect();
    let mut owner : &&str = &"";
    let mut task = "";

    match command_parameters.get(2) {
        Some(value) => {
            task = *value;
        },
        None => {
            println!("user or task was not passed!");
            return ();
        }
    }

    match command_parameters.get(1) {
        Some(value) => owner = value,
        None => {
            println!("no user was passed!");
            return ();
        } 
    }
        if todolist.contains_key(*owner) {
            let a = owner.to_string();
            match todolist.get_mut(&a) {
                Some(value) => {
                    let mut new_list = List {
                        tasks : vec![],
                        completed : vec![] 
                    };
                    
                    for i in &value.tasks {
                        if task.to_string() != *i {
                            new_list.tasks.push(i.clone());
                        } else {
                            println!("dodo");
                            new_list.completed.push(task.to_string())
                        }
                    }

                    *value = new_list;
                    println!("new list: {:?}", value);
                },
                None => return ()
            }
        } else {
            println!("you dont have a todolist!");
            return ();
        }
}


fn run_commands(mut todolist : &mut HashMap<String, List> , ownerr : &String) {
    println!("{}","You can write Commands now: ".blue());

    let mut command = String::new();
    command = take_input(command);

    if command.contains("SHOW") {

        println!("{:?}",todolist.get(ownerr));
    
    } else if command.contains("MARK") {

        mark_completed_task(todolist, &command)
    
    }

}