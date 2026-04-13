use chrono::Local;
use rpassword::read_password;
use serde::de;
use sha2::{Digest, Sha256};
use std::{
    fmt::format,
    fs,
    io::{self},
    task,
};

mod login;
mod registrtion;
mod todo;

use crate::login::login::Status;
use login::login::Loginstatus;
use login::login::UserLogin;
use registrtion::registration::UserRegistration;
use todo::todo::{PriorityLevel, Task, TaskStatus};

enum Registration_Or_Login {
    Register,
    Login,
}

enum TaskOperations {
    CreateTask,
    UpdateTask,
    DeleteTask,
    ViewTasks,
    CompleteTask,
    Quit,
}

#[derive(Debug)]
enum TaskViews {
    TodaysAllTasks,
    DateWiseTasks,
    TodaysCompletedTasks,
    TodaysPendingTasks,
    PriorityWiseTasks,
    BackToHomePage,
}

enum PriorityWiseTasks {
    HighPriorityTasks,
    MediumPriorityTasks,
    LowPriorityTasks,
    HighToLowPriorityTasks,
    LowToHighPriorityTasks,
}
fn main() {
    let mut is_registered = false;

    let content = fs::read_to_string("registration.json").expect("unable to read json data");

    if content.trim().is_empty() {
        is_registered = false
    } else {
        let user: UserRegistration = serde_json::from_str(&content).unwrap();
        is_registered = user.is_registered;
    }

    let register_or_login = match is_registered {
        false => Registration_Or_Login::Register,
        true => Registration_Or_Login::Login,
        _ => panic!("please restart the program and login or register your account"),
    };

    match register_or_login {
        Registration_Or_Login::Register => {
            let mut username = String::new();
            let mut email = String::new();
            // let mut password  = String::new();
            println!("User Name: ");
            io::stdin().read_line(&mut username).unwrap();
            println!("Email: ");
            io::stdin().read_line(&mut email).unwrap();
            println!("Password: ");
            let password = read_password().unwrap();
            
            let mut hasher = Sha256::new();
            hasher.update(password.trim());

            let hash_password_result = hasher.finalize();

            let hex_password = hex::encode(hash_password_result);

            let user = UserRegistration::new(
                username.trim().to_string(),
                email.trim().to_string(),
                hex_password,
                true,
            );

            is_registered = user.is_registered;

            let json = serde_json::to_string_pretty(&user).expect("unable to stringify");

            fs::write("registration.json", json).expect("unable to store json data");
        }
        Registration_Or_Login::Login => {
            let user: UserRegistration =
                serde_json::from_str(&content).expect("unable to objectify");

            let mut username_or_email = String::new();
            // let mut password = String::new();

            println!("Username or Email: ");
            io::stdin().read_line(&mut username_or_email).unwrap();

            println!("Password: ");
            let password = read_password().unwrap();
            // io::stdin().read_line(&mut password).unwrap();

            let mut hasher = Sha256::new();
            hasher.update(password.trim());

            let hash_password = hasher.finalize();
            let hex_password = hex::encode(hash_password);

            let userlogin_info = UserLogin {
                username_or_email: username_or_email.trim().to_string(),
                password: hex_password,
            };

            let logged_in_status =
                userlogin_info.login(&user.username, &user.email, &user.password);

            match logged_in_status {
                Loginstatus::SuccessLogin(status) => {
                    println!("{}", status.message);

                    let mut tasks: Vec<Task> = serde_json::from_str(
                        &fs::read_to_string("tasks.json").expect("unable to read tasks data"),
                    )
                    .unwrap_or_else(|_| Vec::new());

                    let mut id = tasks.len() as u32 + 1;

                    println!("Welcome to the RI_TODO_LIST");

                    loop {
                        let task_operation_choice = home_page();

                        match task_operation_choice {
                            TaskOperations::CreateTask => {
                                let task = create_task(tasks.len() as u32 + 1);

                                let task = match task {
                                    Some(t) => t,
                                    None => {
                                        println!("Failed to create task. Please try again.");
                                        continue;
                                    }
                                };
                                tasks.push(task);
                                let user_tasks = serde_json::to_string_pretty(&mut tasks)
                                    .expect("unable to stringify");
                                fs::write("tasks.json", user_tasks)
                                    .expect("unable to write to file");
                            }
                            TaskOperations::UpdateTask => {
                                update_task(&mut tasks);
                                println!("You have chosen to update a task.");
                            }
                            TaskOperations::DeleteTask => {
                                delete_task(&mut tasks);
                                println!("You have chosen to delete a task.");
                            }
                            TaskOperations::ViewTasks => {
                                tasks_view(&tasks);
                            }
                            TaskOperations::Quit => {
                                println!("You have chosen to quit the program. Goodbye!");
                                std::process::exit(0);
                            }
                            TaskOperations::CompleteTask => {
                                println!("You have chosen to complete a task.");
                            }
                        }
                    }
                }
                Loginstatus::UserNotFound(status) => {
                    println!("{}", status.message)
                }
                Loginstatus::WrongPassword(status) => {
                    println!("{}", status.message)
                }
            };
        }
    };
}

fn delete_task(tasks: &[Task]){
    
}

fn print_tasks(tasks: &Vec<&Task>) {
    for task in tasks {
        println!("-------------------------------");
        println!("Task: {}", task.task_name);
        println!("Description: {}", task.task_description);
        println!("Priority: {:?}", task.priority_level);
        println!("Status: {:?}", task.task_status);
        println!("------------------------------");
    }
}

fn tasks_priority(tasks: &[Task], priority: PriorityLevel) -> Vec<&Task> {
    let todays_date = Local::now().format("%Y-%m-%d").to_string();

    let priority_tasks: Vec<&Task> = tasks
        .iter()
        .filter(|task| (task.created_at[0..10] == todays_date) && (task.priority_level == priority))
        .collect();

    priority_tasks
}

fn tasks_view(tasks: &[Task]) {
    println!("1.Today's All tasks");
    println!("2.Today's pending tasks");
    println!("3.Today's completed tasks");
    println!("4.Priority wise tasks");
    println!("5.Date wise tasks");
    println!("6.Back to home page then write 'back'");

    println!("Please enter your choice: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("somthing went wrong while taking input of tasks views.Try Again.");

    let views_choice = match choice.trim().to_lowercase().as_str() {
        "todays all tasks" | "1" => TaskViews::TodaysAllTasks,
        "todays pending tasks" | "2" => TaskViews::TodaysPendingTasks,
        "todays completed tasks" | "3" => TaskViews::TodaysCompletedTasks,
        "priority wise tasks" | "4" => TaskViews::PriorityWiseTasks,
        "date wise tasks" | "5" => TaskViews::DateWiseTasks,
        "back to home page" | "6" => TaskViews::BackToHomePage,
        _ => {
            println!("Something went wrong");
            home_page();
            return;
        }
    };

    match views_choice {
        TaskViews::TodaysAllTasks => {
            let todays_date = Local::now().format("%Y-%m-%d").to_string();

            let todays_tasks: Vec<&Task> = tasks
                .iter()
                .filter(|task| task.created_at[0..10] == todays_date)
                .collect();

            print_tasks(&todays_tasks);
        }
        TaskViews::TodaysPendingTasks => {
            let pending_tasks: Vec<&Task> = tasks
                .iter()
                .filter(|task| task.task_status == TaskStatus::Pending)
                .collect();
            print_tasks(&pending_tasks);
        }
        TaskViews::TodaysCompletedTasks => {
            let compeleted_tasks: Vec<&Task> = tasks
                .iter()
                .filter(|task| task.task_status == TaskStatus::Completed)
                .collect();
            print_tasks(&compeleted_tasks);
        }
        TaskViews::PriorityWiseTasks => {
            println!("1.print all High Priority tasks.");
            println!("2.print all Medium Priority tasks.");
            println!("3.print all Low Priority tasks.");
            println!("4.print all High to Low Priority tasks.");
            println!("5.print all Low to High Priority tasks.");
            println!("Enter your choice: ");

            let mut priority_choice = String::new();
            io::stdin().read_line(&mut priority_choice).expect(
                "somthing went wrong while taking input of priority wise tasks view.Try Again.",
            );

            let todays_date = Local::now().format("%Y-%m-%d").to_string();

            match priority_choice.trim().to_lowercase().as_str() {
                "print all high priority tasks" | "1" => {
                    let high_priority_tasks: Vec<&Task> =
                        tasks_priority(tasks, PriorityLevel::High);
                    print_tasks(&high_priority_tasks);
                }
                "print all medium priority tasks" | "2" => {
                    let medium_priority_tasks: Vec<&Task> =
                        tasks_priority(tasks, PriorityLevel::Medium);
                    print_tasks(&medium_priority_tasks);
                }
                "print all low priority tasks" | "3" => {
                    let low_priority_tasks: Vec<&Task> = tasks_priority(tasks, PriorityLevel::Low);
                    print_tasks(&low_priority_tasks);
                }
                "print all high to low priority tasks" | "4" => {
                    let high_priority_tasks: Vec<&Task> =
                        tasks_priority(tasks, PriorityLevel::High);
                    let medium_priority_tasks: Vec<&Task> =
                        tasks_priority(tasks, PriorityLevel::Medium);
                    let low_priority_tasks: Vec<&Task> = tasks_priority(tasks, PriorityLevel::Low);

                    print_tasks(&high_priority_tasks);
                    print_tasks(&medium_priority_tasks);
                    print_tasks(&low_priority_tasks);
                }
                "print all low to high priority tasks" | "5" => {
                    let high_priority_tasks: Vec<&Task> =
                        tasks_priority(tasks, PriorityLevel::High);
                    let medium_priority_tasks: Vec<&Task> =
                        tasks_priority(tasks, PriorityLevel::Medium);
                    let low_priority_tasks: Vec<&Task> = tasks_priority(tasks, PriorityLevel::Low);

                    print_tasks(&low_priority_tasks);
                    print_tasks(&medium_priority_tasks);
                    print_tasks(&high_priority_tasks);
                }
                _ => {
                    println!("Wrong choice you have made.Try Again");
                    return;
                }
            }
        }
        TaskViews::DateWiseTasks => {
            let mut tasks_date = String::new();
            let mut tasks_month = String::new();
            let mut tasks_year = String::new();

            println!("Enter the date (DD): ");
            io::stdin()
                .read_line(&mut tasks_date)
                .expect("unable to read date");

            println!("Enter the month (MM): ");
            io::stdin()
                .read_line(&mut tasks_month)
                .expect("unable to read month");

            println!("Enter the year (YYYY): ");
            io::stdin()
                .read_line(&mut tasks_year)
                .expect("unable to read year");

            let date = format!(
                "{:0>2}-{:0>2}-{}",
                tasks_year.trim(),
                tasks_month.trim(),
                tasks_date.trim(),
            );

            let date_wise_tasks: Vec<&Task> = tasks
                .iter()
                .filter(|task| task.created_at[0..10] == date)
                .collect();

            print_tasks(&date_wise_tasks);
        }
        TaskViews::BackToHomePage => {
            home_page();
        }
    };
}

fn home_page() -> TaskOperations {
    println!("what you want to do?");
    println!("If You want to back one step, just type \"back\" in any section.");
    println!("-----------------------------");
    println!("1.Create Task");
    println!("2.Update Task");
    println!("3.Delete Task");
    println!("4.View Tasks");
    println!("5.Complete Task");
    println!("6.Quit");

    println!("Please enter your choice: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("unable to read your choice");

    let task_operation_choice = match choice.trim().to_lowercase().as_str() {
        "create task" | "1" => TaskOperations::CreateTask,
        "update task" | "2" => TaskOperations::UpdateTask,
        "delete task" | "3" => TaskOperations::DeleteTask,
        "view tasks" | "4" => TaskOperations::ViewTasks,
        "complete task" | "5" => TaskOperations::CompleteTask,
        "quit" | "6" => TaskOperations::Quit,
        _ => {
            println!("Invalid choice, try again");
            home_page()
        }
    };
    task_operation_choice
}

fn update_task(tasks: &mut Vec<Task>) {
    println!("What you want to update?");

    println!("1.Task Name");
    println!("2.Task Description");
    println!("3.Task Priority");
    println!("Back to home page: Type 'back'");

    let mut choose_fields_to_update = String::new();

    io::stdin()
        .read_line(&mut choose_fields_to_update)
        .expect("unable to read choose fields");

    match choose_fields_to_update.trim().to_lowercase().as_str() {
        "task name" | "1" => {
            let mut prev_task_name = String::new();
            let mut new_task_name = String::new();

            println!("Enter the previous task name:");
            io::stdin()
                .read_line(&mut prev_task_name)
                .expect("unable to read previous task name");

            println!("Enter the updated task name:");
            io::stdin()
                .read_line(&mut new_task_name)
                .expect("rewrite the new updated task name");

            let now = Local::now();

            if let Some(task) = tasks
                .iter_mut()
                .find(|task| prev_task_name.trim().to_lowercase() == task.task_name)
            {
                task.task_name = new_task_name.trim().to_lowercase();
                task.updated_at = now.format("%Y-%m-%d %H:%M:%S").to_string();
            } else {
                println!("There is no such task.");
            }

            let user_tasks = serde_json::to_string(&tasks).expect("unable to stringify");
            fs::write("tasks.json", user_tasks).expect("unable save data in tasks.json file");
        }
        "task description" | "2" => {}
        "task priority" | "3" => {}
        "back" => {
            home_page();
        }
        _ => {
            println!("Somthing went wrong in update section.Try Again...");
        }
    };
}

fn create_task(id: u32) -> Option<Task> {
    let mut task_name = String::new();
    let mut task_description = String::new();
    let mut priority_level = String::new();

    println!("Enter task name or back to home page then write 'back': ");
    io::stdin()
        .read_line(&mut task_name)
        .expect("unable to read task name");

    if task_name.trim().to_lowercase().as_str() == "back" {
        home_page();
        return None;
    }

    println!("Enter task description: ");
    io::stdin()
        .read_line(&mut task_description)
        .expect("rewrite the task description again.");

    println!("Enter the priority level:");
    io::stdin()
        .read_line(&mut priority_level)
        .expect("unable to read priority level");

    let priority = match priority_level.trim().to_lowercase().as_str() {
        "high" => Some(PriorityLevel::High),
        "medium" => Some(PriorityLevel::Medium),
        "low" => Some(PriorityLevel::Low),
        _ => {
            println!("Priority can only be high medium and low.try again.");
            None
        }
    };

    let priority = match priority {
        Some(p) => p,
        None => return None,
    };

    let now = Local::now();
    Some(Task {
        _id: id,
        task_name: task_name.trim().to_lowercase(),
        priority_level: priority,
        task_status: TaskStatus::Pending,
        task_description: task_description.trim().to_lowercase(),
        remainder: String::from("Remainder"),
        created_at: now.clone().format("%Y-%m-%d %H:%M:%S").to_string(),
        updated_at: now.format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}
