

use serde::{Serialize, Deserialize};
use serde_json;
use std::io::{stdin, stdout, Read, Write};
use std::fs::{self, OpenOptions};
use std::num::ParseIntError;


static mut ID: i32 = 100;

#[derive(Debug, Serialize, Deserialize)]
enum Priority {
    Low, Medium, High
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Pending, Done
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: i32,
    status: Status,
    description: String,
    priority: Priority,
    timestamp: String
}
fn main() {

    let mut to_do_list: Vec<Todo> = Vec::new();
    

    'myloop: loop {
    println!("\n\n::.::... Coral To-Do List Actions ...::.::");
    println!("\t1. Add new todo");
    println!("\t2. Remove one todo");
    println!("\t3. Clear all todos");
    println!("\t4. Mark Task as done");
    println!("\t5. Display pending todos.");
    println!("\t0. Quit to-do.");

    let action = get_input().unwrap();

    match action {
        1 => {
            let new = get_new_todo();
            to_do_list.push(new);
            println!("\n _____ todo added _____\n");
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
        5 => get_and_print_all_todos(),
        0 => break 'myloop,
        other => println!("{other} is not a valid action. Try again.")
    }
}

}

fn get_input() -> Result<i32, ParseIntError> {
    let mut action = String::new();
    print!("\nEnter action :...: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut action).unwrap();
    let action = action.trim().parse::<i32>();
    action
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
    let time_stamp= time_stamp.trim().parse::<u32>().unwrap();

    if time_stamp < 100 || time_stamp > 2359 {
        println!("Time must be in range 100-2359");
    } else {
        break;
    }
    }
    let new = Todo {
        id: unsafe {ID + 1},
        status: Status::Pending,
        description: describe,
        priority: priority,
        timestamp: time_stamp
    };
{
    // Serialize the new todo and store the string in a json file
    let mut json_string = serde_json::to_string(&new).unwrap();
    json_string.push_str("\n");
    // let mut file = File::open("todo.json").unwrap();
    let mut file = OpenOptions::new().append(true).open("todo.json").unwrap();
    // file.write_all(json_string.as_bytes()).unwrap();
    
    // let reader = BufReader::new(file);
    file.write(json_string.as_bytes()).unwrap();
}
    new

}

fn get_and_print_all_todos() {
/* 
    println!("Here are all the pending todos");
    println!("\t{:#?}", v);
*/
    let mut contents = String::new();
    let mut file = fs::OpenOptions::new().read(true).open("todo.json").unwrap();
    fs::File::read_to_string(&mut file, &mut contents).unwrap();
    let deser = serde_json::from_str::<Todo>(&contents).unwrap();
    println!("\n*-.._*_.. HISTORY .._*_..-*");
    println!("{:#?}", deser);
}

fn mark_task_as_done(v: &mut [Todo], id: i32) {
    for i in v {
        if i.id == id {
            i.status = Status::Done;
        }
    }
}

fn remove_one_todo(v: &mut Vec<Todo>, id: i32) {
    for i in 0..v.len() {
        if v[i].id == id {
            v.remove(id as usize);
        }
    }
}

