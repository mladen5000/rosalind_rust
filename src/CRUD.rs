#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    status: Status,
}
#[derive(Debug)]
enum Status {
    Todo,
    InProgress,
    Done,
}

fn create_task(id: u32, description: String) -> Task {
    Task {
        id: id.to_owned(),
        description: description.to_owned(),
        status: Status::Todo,
    }
}

fn read_tasks(tasks: &mut Vec<Task>) {
    for task in tasks.iter() {
        println!("{:?}", task);
    }
}

fn update_task(
    tasks: &mut Vec<Task>, // necessary to modify the vector
    id: u32,
    new_description: Option<String>,
    new_status: Option<Status>,
) {
    for task in tasks.iter_mut() {
        //iter_mut is necessary to modify *elements* of the vector
        if task.id == id {
            if let Some(desc) = new_description {
                task.description = desc
            }
            if let Some(stat) = new_status {
                task.status = stat
            }
            return;
        }
    }
}

fn delete_task(tasks: &mut Vec<Task>, id: u32) {
    let mut pos_to_remove: Option<usize> = None;
    for (pos, task) in tasks.iter_mut().enumerate() {
        if task.id == id {
            pos_to_remove = Some(pos);
            break;
        }
    }
    if let Some(pos) = pos_to_remove {
        tasks.remove(pos);
    }
}

fn main() {
    let mut tasks = Vec::new();
    loop {
        println!("What would you like to do?");
        println!("1: Create Task");
        println!("2: Read Tasks");
        println!("3: Update Task");
        println!("4: Delete Task");
        println!("5: Exit");

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        match choice.trim().as_ref() {
            "1" => {
                let mut input_id = String::new();
                println!("Enter task id");
                std::io::stdin()
                    .read_line(&mut input_id)
                    .expect("Failed to read input_id");

                let id = input_id
                    .trim()
                    .parse::<u32>()
                    .expect("Please enter a valid number");

                let mut input_description = String::new();
                println!("Enter task description");
                std::io::stdin()
                    .read_line(&mut input_description)
                    .expect("Failed to read input_id");

                //Get id and description from user, then call create_task
                let new_task = create_task(id, input_description);
                tasks.push(new_task)
            }
            "2" => read_tasks(&mut tasks),
            "3" => {
                let mut input_id = String::new();
                println!("Enter task id");
                std::io::stdin()
                    .read_line(&mut input_id)
                    .expect("Failed to read input_id");

                let id = input_id
                    .trim()
                    .parse::<u32>()
                    .expect("Please enter a valid number");

                let mut new_description = String::new();
                println!("Enter task description");
                std::io::stdin()
                    .read_line(&mut new_description)
                    .expect("Failed to read input_id");

                let mut input_status = String::new();
                println!("Enter task status");
                std::io::stdin()
                    .read_line(&mut input_status)
                    .expect("Failed to read input_id");

                let new_status = match input_status.trim().to_ascii_lowercase().as_str() {
                    "todo" => Status::Todo,
                    "in progress" => Status::InProgress,
                    "done" => Status::Done,
                    _ => panic!("Not a valid status"),
                };

                // Get id, description, and status
                update_task(&mut tasks, id, Some(new_description), Some(new_status));
            }
            "4" => {
                let mut input_id = String::new();
                println!("Enter task id");
                std::io::stdin()
                    .read_line(&mut input_id)
                    .expect("Failed to read input_id");

                let id = input_id
                    .trim()
                    .parse::<u32>()
                    .expect("Please enter a valid number");
                // Get id
                delete_task(&mut tasks, id);
            }
            "5" => break,
            _ => println!("Invalid choice, please try again"),
        }
    }
}
