

use serde::{Serialize, Deserialize};
use serde_json;
use std::io::{Write, stdin, stdout};
use std::fs;

static mut ID: i32 = 100;

#[derive(Debug, Serialize, Deserialize)]
enum Priority {
    Low, Medium, High
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Pending, Done
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Todo {
    id: i32,
    status: Status,
    description: String,
    priority: Priority,
    timestamp: String
}
fn main() {

    let mut to_do_list: Vec<Todo> = Vec::new();
    let json_file = fs::File::open("todo.json").unwrap();
    serde_json::to_string(&mut to_do_list);
    let mut action = String::new();

    'myloop: loop {
    println!("\n\n::.::... Coral To-Do List Actions ...::.::");
    println!("\t1. Add new todo");
    println!("\t2. Remove one todo");
    println!("\t3. Clear all todos");
    println!("\t4. Mark Task as done");
    println!("\t5. Display pending todos.");
    println!("\t0. Quit to-do.");

    print!("\nEnter action :..: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut action).unwrap();
    let action = action.trim().parse::<u8>().unwrap();

    match action {
        1 => {
            let add_new = get_new_todo();
            to_do_list.push(add_new);
        },
        2 => {
            loop{
                let mut fetch_id = String::new();
                print!("Enter id of task: ");
                stdout().flush().unwrap();
                stdin().read_line(&mut fetch_id).unwrap();
                let fetch_id = fetch_id.trim().parse::<i32>().unwrap();
    
                if fetch_id < 100 || fetch_id > 100 + to_do_list.len() as i32 {
                    println!("Todo ID must be between 101-{}", 100+to_do_list.len());
                } else {
                    remove_one_todo(&mut to_do_list, fetch_id);
                    break;
                }
            }
        }
        3 => 
            to_do_list.clear(),
        4 => {
            loop{
            let mut fetch_id = String::new();
            print!("Enter id of task: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut fetch_id).unwrap();
            let fetch_id = fetch_id.trim().parse::<i32>().unwrap();

            if fetch_id < 100 || fetch_id > 100 + to_do_list.len() as i32 {
                println!("Todo ID must be between 101-{}", 100+to_do_list.len());
            } else {
                mark_task_as_done(&mut to_do_list, fetch_id);
                break;
            }
        }
        }
        5 => print_all_todos(&to_do_list),
        0 => break 'myloop,
        other => println!("{other} is not a valid action. Try again.")
    }
}

}

fn get_new_todo() -> Todo {

    let mut describe = String::new();
    print!("Enter your todo description: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut describe).unwrap();

    let mut prioritize = String::new();
    print!("How much priority? (1.Low 2.Medium 3.High: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut prioritize).unwrap();
    let prioritize = prioritize.trim().parse::<i32>().unwrap();
    let priority = match prioritize {
        1 => Priority::Low,
        2 => Priority::Medium,
        3 => Priority::High,
        _ => Priority::Medium
    };

    let mut time_stamp = String::new();
    loop {
    print!("Enter time stamp in 24hr format: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut time_stamp).unwrap();
    let time_stamp: u32 = time_stamp.trim().parse::<u32>().unwrap();

    if time_stamp < 100 || time_stamp > 2359 {
        println!("Time must be in range 100-2359");
    } else {
        break;
    }
    }
    Todo {
        id: unsafe {ID + 1},
        status: Status::Pending,
        description: describe,
        priority: priority,
        timestamp: time_stamp
    }

}

fn print_all_todos(v: &[Todo]) {

    println!("Here are all the pending todos");
    println!("\t{:#?}", v);   
}

fn mark_task_as_done(v: &mut [Todo], id: i32) {
    for i in v {
        if i.id == id {
            i.status = Status::Done;
        }
    }
}

fn remove_one_todo(v: &mut Vec<Todo>, Id: i32) {
    for i in 0..v.len() {
        if v[i].id == Id {
            v.remove(Id as usize);
        }
    }
}

