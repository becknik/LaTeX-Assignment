use std::{io::stdin, fs::File, path::Path};

fn main() {
    let project_name = "Course_ExXX_Template";

    let mut path_exec = std::env::current_exe().expect("Failed to read execution path.");
    let mut path_project = path_exec.as_path();
    while !path_project.ends_with(project_name) && !path_project.ends_with(".exe") {
        println!("{}",path_project.display());
        let parent_dir = path_project.parent();
        match parent_dir {
            Some(path) => path_project = path,
            None => panic!("Parent dir not found. Make sure your git repo name fits the project name entity.")
        } 
    }
    let entities_path = path_exec.join("LaTeX").join("src").join("entities.tex");
    
    let mut entites = File::open(entities_path);
    
    println!("This is a simple utility to prepare this repository to be used as a copy-past template for your submissions.");
    
    let course_long = get_user_input("Please enter your full course name");
    let course_short= get_user_input("Please enter your courses abbreviation");
    
    let group_members: String = get_user_input("Please enter your groups member names, separated with commas");
    let group_iter = group_members.split(",");
    for member in group_iter {
        let members_data = get_user_input(format!("Please enter {}'s comma separated ID and student name", member.trim()).as_str()).split(",");
    }
}

fn get_user_input(message:&str) -> String {
    let mut user_input = String::new();
    let mut input_correct:bool = false;
    
    while !input_correct {
        println!("{}", message);
        stdin().read_line(&mut user_input).expect("String parsing failed.");
        
        if user_input.trim().is_empty() {
            println!("Something went wrong, try again.")
        } else {
            input_correct = true;
        }
    }
    return user_input;
}