use std::{io::{stdin, Read, Error}, fs::{File, self}, path::{PathBuf}, ops::{Add, Deref, Index}};

fn main() {
    let path_project = get_proj_root();
    let path_entities = path_project.join("LaTeX").join("src").join("entities.tex");
    let path_header = path_project.join("LaTeX").join("src").join("header.tex");

    let mut entities: String = String::new();
    let mut entities_file: File = File::open(path_entities).expect("Entities file not found.");
    entities_file.read_to_string(&mut entities).expect("Could not read LaTeX entities file.");
    drop(entities_file);
    let mut entities_default: String = entities.clone();
    
    let mut header: String = String::new();
    let mut header_file = File::open(path_header).expect("Latex header.tex file not found in \"LaTeX\"src\" folder.");
    header_file.read_to_string(&mut header).expect("Could not read LaTeX entities file.");
    drop(header_file);

    println!("This is a simple utility to prepare this repository to be used as a copy-past template for your submissions.");
    println!("Starting cleaning up repo files...");

    //remove_unnecessary_repo_folders(&path_project); TODO

    print!("\n");
    for _i in 0..10 {
        print!("-----\t");
    }
    println!("\nLet's just start setting up the template according to your needs. If you want to do this manually consider aborting with 'Ctrl+c'");
    println!("Your input of the following lines ist print into the correct tex files.\nTheoretically you can rerun this program to fix typos or something, but nevertheless be attentive :)\n\n");
    
    let messages_and_tex_identifiers: Vec<(&str, &str)> = vec![
        ("Please enter semester season your course takes place: ", "\\semester"),
        ("Please enter the full length name of your course:", "\\course"),
        ("Please enter your courses name abbreviation: ", "\\courseShort"),
        ("Please enter your group's number (e.g. \"Group 42\"): ", "\\group")
    ];

    for message_identifier in messages_and_tex_identifiers {
        replace_init_entity(&mut entities, message_identifier.1, message_identifier.0)
    }

    let group_members = get_user_input("member", "Please enter the names of your groups members, all separated with commas: ");
    let group_iter = group_members.split(",");
    let mut counter:u32 = 10; // For exotic way of int -> char conversion: counter representing hexadecimal value :D

    for member_name in group_iter {
        let member_char: char = char::from_digit(counter, 16).unwrap();
        let member_char_uppercase = member_char.to_uppercase().to_string();

        let member_name_identifier = String::from("\\member").add(&member_char_uppercase);
        // TODO Operation die im header automatisch Auskommentiert
        replace_init_entity_with_value(&mut entities, &member_name_identifier.clone(), member_name);

        let member_id_identifier = member_name_identifier.clone() + "Martricle";
        replace_init_entity(&mut entities, &member_id_identifier, format!("Please enter {}'s marticle number: ", member_name).as_str());
        let member_mail_identifier = member_name_identifier.clone() + "Mail";
        replace_init_entity(&mut entities, &member_mail_identifier, format!("Please enter {}'s student mail: ", member_name).as_str());

        counter += 1;
    }
    
    let mut default_identifiers: Vec<&str> = Vec::new();
    let mut entities_modified_iter = entities.lines();
    let mut entities_default_iter = entities_default.lines();
    
    loop {
        let default_and_modified: (Option<&str>, Option<&str>) = (entities_default_iter.next(), entities_modified_iter.next());
        match default_and_modified.0 { // To stop when there is no line left in Lines iterator
            Some(default_line) => {
                if default_line.eq(default_and_modified.1.unwrap()) && default_line.contains("member") {
                    let line:Option<usize> = default_line.find("\\def");
                    match line { // To make sure identifiers name parsing takes place on identifier lines only
                        Some(def_index) => { 
                            let identifier:&str = default_line.get(def_index+5..default_line.find("{").unwrap()-1).unwrap();
                            default_identifiers.push(identifier)
                        },
                        None => ()
                    }
                }
            },
            None => break
        }
    }

    //println!("{:?}", &default_identifiers);

    let mut add_procent:Vec<usize> = Vec::new();
    let mut remove_procent:Vec<usize> = Vec::new();
    
    let mut header_lines = header.lines();
    println!("{}", header_lines.next().unwrap());

    for line in header.lines() {
        println!("{}", &line);
        for entity in &default_identifiers {
            if line.contains(*entity) && !line.contains("%") && !line.contains("pdftitle") {
                add_procent.push(line.find("\\").unwrap());
            } else if line.contains(*entity) && line.contains("%") && line.find("%").unwrap() > 4 {
                remove_procent.push(line.find("%").unwrap())
            }
        }
    }

    for index in add_procent {
        header.insert(index, '%');
    }
        // let author_patter = format!("textbf({})", &entity);
        // header.insert(header.find(&entity).unwrap(), '%');
        // let header_pattern = format!("{} ~|~MtNr.)", &entity);
        // header.insert(header.find(&header_pattern).unwrap(), '%');
        // let hyperref_pattern = format!("{},)", &entity);
        // header.replace(&hyperref_pattern, "");

    //println!("{}", header);

    // nicht member entitäten rausfiltern, (reuseability), auskommentieren

    // let mut header_iter = header.lines();
    // loop {
    //     match header_iter.next() {
    //         Some(line) => {
    //             for identifier in default_identifiers {
    //                 if (line.contains(identifier)) {
    //                     //
    //                 }
    //             }
    //         }
    //         None => break
    //     }
    // }
    

    // println!("{}", entities);
    // println!("{}", header);
}

// Returns the user input to the provided message by obeying the formatting rules defined for the parameterized latex identifier
fn get_user_input(identifier: &str, message: &str) -> String {

    let mut user_input: String = String::new();
    let mut identifier_semantic: String = identifier.to_owned();
    if identifier_semantic.contains("\\") {
        identifier_semantic = identifier_semantic.get(1..identifier_semantic.len()).unwrap().to_string();
    }
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
                    break;
                }
            }
            "Martricle" => {
                already_tested = true;
                if user_input.trim().len() == 7 {
                    break
                } else {
                    eprintln!("Seems like the entered student ID's length is not 8 and therefore not valid.")
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

// Replaces the entity value of the specified identifier in the entities.tex file with a user-entered value. The message specifier is for explanation for this user entered value.
fn replace_init_entity(entities_tex_file: &mut String, identifier: &str, message: &str) {
    let user_value = get_user_input(&identifier, message); // New user modified entity init gets saved in 'value'
    let start_index_identifier = entities_tex_file.find(identifier).expect(format!("Something went wrong locating {}'s declaration in \"LaTeX/src/entities.tex\" string.", identifier).as_str());
    let mut init_entity_slice: &str = entities_tex_file.get(start_index_identifier..entities_tex_file.len()).unwrap();

    let mut start_index = init_entity_slice.find("{").expect(format!("Error ocurred locating the start-position of the default initialization by searching for \"{{\" in for current entity trimmed entities.tex: {}", init_entity_slice).as_str());
    start_index += 1; // To not cut the '{'
    let end_index = init_entity_slice.find("}").expect(format!("Error ocurred locating the end-position of the default initialization by searching for \"}}\" in for current entity trimmed entities.tex: {}", init_entity_slice).as_str());

    init_entity_slice = init_entity_slice.get(start_index..end_index).expect(format!("Error ocurred trimming the slice to fit the initialization value - start index: {}, end index: {}, entity: {}",start_index, end_index, init_entity_slice).as_str());
    *entities_tex_file = entities_tex_file.replace(init_entity_slice,&user_value.trim());
}

// Replaces the entity value of the specified identifier in the entities.tex file with the parameterized value.
fn replace_init_entity_with_value(entities_tex_file: &mut String, entity: &str, value: &str) { // I don't knooow how to do this better :(
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