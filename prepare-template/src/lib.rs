pub fn run() -> Result<(), Box<dyn std::error::Error>> {
	println!("Start setup of identities to be used in this template instance:");
	let entities = parse::init_entities();
	let mut file_contents = mod_content::fetch_content()?;

	let mut necessary_for_move = vec![]; // TODO For creating a copy of the execution folder
	let standard_enities = entities.0;

	for mut entity in standard_enities {
		parse::set_user_input(&mut entity)?;

		mod_content::fill_in_user_input(&mut file_contents.0, &entity);

		if entity.is_necessary() {
			let necessary_values = entity.user_input().to_string();
			necessary_for_move.push(necessary_values);
		}
	}

	println!("{}", file_contents.0);

	Ok(())
}

mod parse {
	use std::fmt::Debug;

use regex::Regex;

	pub struct Entity {
		identifier: Regex,
		message: String,
		regex: Regex,
		user_input: String
	} impl Entity {
		pub fn identifier (&self) -> &Regex {
			&&self.identifier
		}

		pub fn user_input (&self) -> &str {
			&&self.user_input
		}

		pub fn is_necessary(&self) -> bool {
			self.identifier.is_match("\\courseShort")
		}

		pub fn set_input_if_valid(&mut self, user_input :String) -> bool {
			if self.regex.is_match(&user_input) {
				self.user_input = user_input;
				true
			} else {
				false
			}
		}
	} impl Debug for Entity {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.debug_struct("Entity").field("identifier", &self.identifier).field("user_input", &self.user_input).finish()
		}
	}

	pub fn set_user_input(entity: &mut Entity) -> Result<(), Box<dyn std::error::Error>> {
		loop {
			let mut user_input = String::new();
			println!("> {}", &entity.message);
			std::io::stdin().read_line(&mut user_input)?;

			if entity.regex.is_match(&user_input) {
				entity.set_input_if_valid(user_input);
				return Result::Ok(());
			} else {
				eprintln!("Your input does not match the following RegEx: {}. Please try again!\n", entity.regex.as_str())
			}
		}
	}


	pub fn init_entities() -> (Vec<Entity>, Vec<Entity>) {

		let semester = Entity {
    		identifier: Regex::new(r"\\semester\s?\{[a-zA-Z0-9\s/]*\}").unwrap(),
    		message: "Please enter the semester season your want to use this template for.".to_string(),
    		regex: Regex::new(r"([[:alpha:]]{4,14}\s\d{2,4}|[[:alpha:]]{2,14}/[[:alpha:]]{2,14}\s\d{2,4})").unwrap(),
    		user_input:String::new()
		};
		let course = Entity {
    		identifier: Regex::new(r"\\course\{[a-zA-Z0-9\s]*\}").unwrap(),
    		message: "Please enter the full length name of your course.".to_string(),
    		regex: Regex::new(r"[[:alpha:]]+").unwrap(),
    		user_input:String::new()
		};
		let short_course = Entity {
    		identifier: Regex::new(r"\\courseShort\{[a-zA-Z0-9\s]*\}").unwrap(),
    		message: "Please enter your courses name abbreviation.".to_string(),
    		regex: Regex::new(r"[[:alpha:]-\s0-9]+").unwrap(),
    		user_input:String::new()
		};
		let group = Entity {
    		identifier: Regex::new(r"\\group\{[a-zA-Z0-9\s]*\}").unwrap(),
    		message: "Please enter your group's team-number.".to_string(),
    		regex: Regex::new(r"[[:alpha:]]{5,6}\s[\d]{1,2}").unwrap(),
    		user_input:String::new()
		};

		let names = Entity {
    		identifier: Regex::new(r"member[A-F][[:^alpha:]]\{[a-zA-Z0-9\s]*\}").unwrap(),
    		message: "Please enter your group members names, all separated by commas.".to_string(),
    		regex: Regex::new(r"([A-Z][a-z]+(\s[A-Z][a-z]+)+,\s*)+").unwrap(),
    		user_input:String::new()
		};

		let id = Entity {
    		identifier: Regex::new(r"member[A-F]Martricle\{[a-zA-Z0-9\s]*\}").unwrap(),
    		message: "Please enter {}'s student ID number.".to_string(),
    		regex: Regex::new(r"\d{7}").unwrap(),
    		user_input:String::new()
		};
		let mail = Entity {
    		identifier: Regex::new(r"member[A-F]Mail\{[a-zA-Z0-9\s]*\}").unwrap(),
    		message: "Please enter {}'s public student mail.".to_string(),
    		regex: Regex::new(r"[[:alpha:]0-9]+@\..+").unwrap(),
    		user_input:String::new()
		};

		return (vec![semester, course, short_course, group], vec![names, id, mail]);
	}
}

mod mod_content {
    use std::{fs::File, error::Error, io::Read};

    use regex::Regex;

    use super::parse::Entity;

	pub fn fetch_content() -> Result<(String, String), Box<dyn Error>> {
		let exec = std::env::current_exe()?;
		let mut project_root = exec.as_path();

		while !project_root.ends_with("Course_ExXX_Template") {
			project_root = project_root.parent().unwrap();
		}

		let mut entities = File::open(project_root.join("LaTeX").join("src").join("entities.tex"))?;
		let mut header = File::open(project_root.join("LaTeX").join("src").join("header.tex"))?;

		let mut entities_content = String::new();
		entities.read_to_string(&mut entities_content)?;
		let mut header_content = String::new();
		header.read_to_string(&mut header_content)?;

		Result::Ok((entities_content, header_content))
	}

	pub fn fill_in_user_input(entites: &mut String, entity: &Entity) {
		let regex_identifier = entity.identifier();
		let identifier = regex_identifier.find(&entites).unwrap().as_str();
		println!("The identifier regex: {}",&regex_identifier);
		println!("Match to identifier regex: {}",&identifier);
		let regex_value = Regex::new(r"\{[a-zA-Z0-9\s@-*\.\}").unwrap(); // help needed with /...
		let value = regex_value.find(identifier).unwrap().range();
		entites.replace_range(value, entity.user_input());
	}
}

mod files {

}