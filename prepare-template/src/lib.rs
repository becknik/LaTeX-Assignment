mod parse {
    use std::error::Error;

	pub struct Entity {
		identifier: String,
		message: String,
		user_input:String
	}

	pub enum Entity {
		Season(String, String),
		Course(String, String),
		ShortCourse(String, String),
		Group(String, String),
		Mail(String, String),
		MNr(String, String)
	}
	impl Entity { // Wold be nice if I just could initialize a Entity::Season etc. directly with Entity::Season::new()
		pub fn new(identifier: &str) -> Result<Entity, Box<dyn Error>> {
			match identifier {
				"semester" => Ok(Entity::Season(identifier.to_owned(), "Please enter the semester season your course takes place in.".to_owned())),
				"course" => Ok(Entity::Course(identifier.to_owned(), "Please enter the full length name of your course. Your input will show up on the heading page.".to_owned())),
				"courseShort" => Ok(Entity::ShortCourse(identifier.to_owned(), "Please enter your courses name abbreviation. It will be part of this templates instance directory.".to_owned())),
				"group" => Ok(Entity::Group(identifier.to_owned(), "Please enter your group's number.".to_owned())),
				"Mail" => Ok(Entity::Mail(identifier.to_owned(), "Please enter {}'s student ID.".to_owned())),
				"Martricle" => Ok(Entity::MNr(identifier.to_owned(), "Please enter {}'s student mail.".to_owned()))
			}
		}
		// pub fn get_message(&self) -> &str {
		// }

		pub fn proof(&self, input: &str) -> Option<&str> {

		}
	}
}