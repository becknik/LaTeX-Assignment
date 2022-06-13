use std::{io::{stdin, Read, Error, Write}, fs::{File, self}, path::{PathBuf}, ops::{Add}};

fn main() {
    let path_project = get_proj_root();
    let path_entities = path_project.join("LaTeX").join("src").join("entities.tex");
    let path_header = path_project.join("LaTeX").join("src").join("header.tex");

    let mut entities: String = String::new();
    let mut entities_file: File = File::open(&path_entities).expect("Entities file not found.");
    entities_file.read_to_string(&mut entities).expect("Could not read LaTeX entities file.");

    let mut header: String = String::new();
    let mut header_file = File::open(&path_header).expect("Latex header.tex file not found in \"LaTeX\"src\" folder.");
    header_file.read_to_string(&mut header).expect("Could not read LaTeX entities file.");


    println!("This is a simple utility to prepare this repository to be used as a copy-past template for your submissions.");
    println!("Starting cleaning up repo files...");

    //remove_unnecessary_repo_folders(&path_project);

    print!("\n");
    for _i in 0..10 {
        print!("-----\t");
    }
    println!("\nLet's just start setting up the template according to your needs. If you want to do this manually consider aborting with 'Ctrl+c'");
    println!("Your input of the following lines ist print into the correct tex files.\nTheoretically you can rerun this program to fix typos or something, but nevertheless be attentive :)\n\n");

    let messages_and_tex_identifiers: Vec<(&str, &str)> = vec![
        ("Please enter semester season your course takes place: ", "semester"),
        ("Please enter the full length name of your course:", "course"),
        ("Please enter your courses name abbreviation: ", "courseShort"),
        ("Please enter your group's number (e.g. \"Group 42\"): ", "group")
    ];

    for message_and_identifier in messages_and_tex_identifiers {
        let user_input: String = get_user_input(message_and_identifier.1, message_and_identifier.0);
        replace_init_entity(&mut entities, message_and_identifier.1, &user_input);
    }

    let group_members = get_user_input("member", "Please enter the names of your groups members, all separated with commas: ");

    let mut modified_entities: Vec<String> = Vec::new();
    let mut unnecessary_entities: Vec<String> = Vec::new();

    let mut group_iter = group_members.split(",");
    for counter in 10..16 { // For exotic way of int -> char conversion: counter representing hexadecimal value :D
        let member_char: char = char::from_digit(counter, 16).unwrap();
        let member_char_uppercase = member_char.to_uppercase().to_string();
        let member_identifier = String::from("member").add(&member_char_uppercase); // Should contain \memberA..F

        let next_group_member: Option<&str> = group_iter.next();
        if next_group_member.is_some() { // Setting the entity values by user input
            let current_member_name: &str = next_group_member.unwrap();
            replace_init_entity(&mut entities, &member_identifier.clone(), current_member_name);
            let member_id_identifier: String = member_identifier.clone() + "Martricle";
            let current_member_id: String = get_user_input(&member_id_identifier, &format!("Please enter {}'s marticle number: ", current_member_name));
            replace_init_entity(&mut entities, &member_id_identifier, &current_member_id);
            let member_mail_identifier: String = member_identifier.clone() + "Mail";
            let current_member_mail: String = get_user_input(&member_mail_identifier, &format!("Please enter {}'s student mail: ", current_member_name));
            replace_init_entity(&mut entities, &member_mail_identifier, &current_member_mail);

            modified_entities.push(member_identifier.clone()); // For proving the entities are not commented out in the header file
        }
        else {
            unnecessary_entities.push(member_identifier)
        }
    }

    let mut add_percent_in_header:Vec<usize> = Vec::new();
    let mut remove_percent_in_header:Vec<usize> = Vec::new();

    let mut header_iter = header.lines();
    while let Some(line) = header_iter.next() {
        if line.contains("pdfauthor") { // TODO fix this, adjust header size according to member count, sort members lastnames alphabetically, add better fill-in pattern in get-user-name method, remove line breaks in tex header
            continue;
        }

        let beginning_of_line: Option<&str> = line.get(0..10);

        for entity in &modified_entities {
            if line.contains(entity) && beginning_of_line.unwrap().contains("%") {
                let total_index: usize = header.find(&line).unwrap() + line.find("%").unwrap();
                remove_percent_in_header.push(total_index);
            }
        }
        for entity in &unnecessary_entities {
            if line.contains(entity) && !beginning_of_line.unwrap().contains("%") {
                let total_index: usize = header.find(&line).unwrap() + line.find("\\").unwrap();
                add_percent_in_header.push(total_index);
            }
        }
    }

    let mut iter = add_percent_in_header.iter();
    let mut shifting_counter: usize = 0;
    while let Some(index) = iter.next() {
        header.insert(*index + shifting_counter, '%');
        shifting_counter += 1; // Fix adding a char shift
    }
    shifting_counter = 0;
    iter = remove_percent_in_header.iter();
    while let Some(index) = iter.next() {
        header.remove(index - shifting_counter);
        shifting_counter += 1;
    }

    let mut f = std::fs::OpenOptions::new().write(true).truncate(true).open(path_header.as_path()).unwrap();
    let result_0 = f.write(&header.as_bytes());
    f = std::fs::OpenOptions::new().write(true).truncate(true).open(path_entities.as_path()).unwrap();
    let result_1 = f.write(&entities.as_bytes());

    if result_0.is_ok() && result_1.is_ok() {
        println!("We did it!")
    }
}

// Returns the user input to the provided message by obeying the formatting rules defined for the parameterized latex identifier
fn get_user_input(identifier: &str, message: &str) -> String {

    let mut user_input: String = String::new();
    let mut identifier_semantic: String = identifier.to_owned();
    for c in 'A'..'F' {
        if identifier_semantic.contains(c) && 6 < identifier_semantic.len() {
            identifier_semantic = identifier_semantic.get(7..identifier_semantic.len()).unwrap().to_string();
            break;
        }
    }

    loop {
        println!("> {}", message);
        stdin().read_line(&mut user_input).unwrap();
        if user_input.contains("\n") { // Maybe remove this? -> TODO read the trim() docs
            user_input = user_input.get(0..user_input.len()-1).unwrap().to_owned();
        }
        user_input = user_input.trim().to_owned();

        let mut already_tested: bool = false; // Not very elegant...
        match identifier_semantic.as_str() {
            "Mail" => {
                already_tested = true;
                if !user_input.contains("@") {
                    eprintln!("Seems like the student mail does not contain a '@'.")
                } else if !user_input.contains(".") {
                    eprintln!("Your specified mail address does not contain a '.'")
                } else if user_input.is_empty() {
                    eprintln!("The email field can't be left blank.")
                } else {
                    break
                }
            }
            "Martricle" => {
                already_tested = true;
                if user_input.trim().len() == 7 {
                    break
                } else {
                    eprintln!("Seems like the entered student ID's length is not 7 and therefore not valid.")
                }
            }
            "courseNameShort" => {
                already_tested = true;
                if user_input.is_empty() {
                    eprintln!("Leaving the short course name empty is not allowed (HAHA!).");
                }
                break;
            }
            "group" => {
                already_tested = true;
                if user_input.is_empty() {
                    eprintln!("The groups name must be filled in.")
                } else if !user_input.contains(" ") {
                    eprintln!("Your group does not contain a space, make sure you typed correctly.")
                } else {
                    break;
                }
            }
            _default => {
                if !already_tested {
                    break;
                }
                eprintln!("Please try again!\n")
            }
        }
    }
    return user_input;
}

// Replaces the entity value of the specified identifier in the entities.tex file with the parameterized value.
fn replace_init_entity(entities_tex_file: &mut String, entity: &str, value: &str) { // I don't knooow how to do this better :(
    let start_index_identifier = entities_tex_file.find(entity).expect(format!("Something went wrong locating {}'s declaration in \"LaTeX/src/entities.tex\" string.", entity).as_str());
    let mut init_entity_slice: &str = entities_tex_file.get(start_index_identifier..entities_tex_file.len()).unwrap();

    let mut start_index = init_entity_slice.find("{").expect(format!("Error ocurred locating the start-position of the default initialization by searching for \"{{\" in for current entity trimmed entities.tex: {}", init_entity_slice).as_str());
    start_index += 1; // To not cut the '{'
    let end_index = init_entity_slice.find("}").expect(format!("Error ocurred locating the end-position of the default initialization by searching for \"}}\" in for current entity trimmed entities.tex: {}", init_entity_slice).as_str());

    init_entity_slice = init_entity_slice.get(start_index..end_index).expect(format!("Error ocurred trimming the slice to fit the initialization value - start index: {}, end index: {}, entity: {}",start_index, end_index, init_entity_slice).as_str());
    *entities_tex_file = entities_tex_file.replace(init_entity_slice,&value.trim());
}

// For running this program in the cargo folder while developing
fn get_proj_root() -> PathBuf {
    let mut path_exec = std::env::current_exe().expect("Failed to read execution path.");

    while !path_exec.ends_with("Course_ExXX_Template") { // Navigates to the repos root when starting from cargo build directory
        path_exec = path_exec.parent().expect("Failed to determine the repos root.").to_path_buf();
    }
    return path_exec;
}

fn remove_unnecessary_repo_folders(project_path: &PathBuf) {
    let remove_operations:Vec<Result<(),Error>> = vec![
        fs::remove_dir_all(project_path.join(".git")),
        fs::remove_dir_all(project_path.join("prepare-template")),
        fs::remove_file(project_path.join("README.md"))
    ];

    for operation in remove_operations {
        if operation.is_err() {
            println!("A cleanup operation terminated with an error: {}", operation.unwrap_err())
        }
    }
    println!("Cleanup process finished! :>")
}