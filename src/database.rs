use std::{collections::HashMap, fs};

pub struct Database {
    contents: HashMap<String, String>,
}

impl Database {
    // create new database
    pub fn new() -> Database {
        let contents = Database::load();

        Database { contents }
    }

    // commit contents into database file
    fn commit(contents: &str, err: Option<&str>) {
        fs::write("kv.db", contents).expect(match err {
            None => "unable to save db",
            Some(e) => e
        });
    }

    // load contents from database file
    // creates a new database file if it does not exists
    fn load() -> HashMap<String, String> {
        let file_contents = match fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(_) => {
                let default = "foo\tbar\n";

                Database::commit(default, Some("unable to create db"));

                String::from(default)
            }
        };

        println!("loading: {} \n done loading", file_contents);

        let mut contents = HashMap::new();

        for entry in file_contents.lines() {
            let (key, value) = entry.split_once("\t").expect("db malformed");

            contents.insert(key.to_owned(), value.to_owned());
        }

        // let hash = HashMap::from(
        //     ..file_contents
        //     .lines()
        //     .map(|entry| entry
        //         .split_once("\t")
        //         .expect("db malformed")
        //     )
        // );

        return contents;
    }

    // get item by key
    pub fn get(&self, key: &str) -> &String {
        self.contents
            .get(&key.to_owned())
            .expect(format!("key '{}' not found", key).as_str())
    }

    // set item by key
    // overrides item if key already exists
    pub fn set(&mut self, key: &str, value: &str) {
        self.contents.insert(key.to_owned(), value.to_owned());

        let mut file_contents = String::new();

        for (key, value) in &self.contents {
            let mut entry = key.clone();
            entry.push_str("\t");
            entry.push_str(value.as_str());
            entry.push_str("\n");

            file_contents.push_str(entry.as_str());
        }

        println!("setting: {} \n done setting", file_contents);

        Database::commit(file_contents.as_str(), None);
    }
}
