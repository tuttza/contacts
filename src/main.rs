// Standard Libs:
use std::env;
use std::io::{self, Write};
use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use std::process;

// External Libs:
extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate app_dirs;
use app_dirs::*;

const APP_INFO: AppInfo = AppInfo{name: "Contacts", author:"zach tuttle"};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Contact {
    name:   String,
    phone:  String,
    email:  String
}

impl Contact {
    fn new(name: String, phone: String, email: String ) -> Contact {
        Contact {name, phone, email}
    }

    fn create_prompt() {
        let mut name  = String::new();
        let mut phone = String::new();
        let mut email = String::new();

        println!("==[ NEW CONTACT ]==");
                    
        print!("Contact Name: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut name)
            .ok()
            .expect("Couldn't read line");    
        
        print!("Contact Phone: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut phone)
            .ok()
            .expect("Couldn't read line");
        let _ = io::stdout().flush();

        print!("Contact Email: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut email)
            .ok()
            .expect("Couldn't read line");

        let new_contact = Contact::new(name.to_string(), phone.to_string(), email.to_string());
        Contact::save(new_contact);
    }

    fn data_file_path() -> PathBuf {
        let mut contact_data_file = app_dir(AppDataType::UserData, &APP_INFO, "contacts_data").unwrap();
        contact_data_file.push("data");
        contact_data_file.set_extension("bin");

        return contact_data_file;
    }

    fn create_data_file() -> File {
        let file = File::create(Contact::data_file_path());
   
        let file = match file {
            Ok(file) => file,
            Err(error) => {
                panic!("Something happening while trying to create the contact data file: {:?}", error);
            }
        };

        return file;
    }

    fn load_data() -> Vec<Contact> {
        let contact_data = File::open(Contact::data_file_path());

        let contact_data = match contact_data {
            Ok(file) => file,
            Err(_error) => {
                println!("You have no contacts yet. Run 'contacts add' to add a contact.");
                process::exit(0);
            }
        };

        let reader = BufReader::new(contact_data);
        let decoded: Vec<Contact> = bincode::deserialize_from(reader).unwrap();

        return decoded;
    }

    fn display_count() {
        let contact_data = Contact::load_data();
        println!("You have {:?} contacts.", contact_data.len());
    }

    fn load_all() {
        let contact_data = Contact::load_data();

        println!("==[ Contacts ]==");

        for c in contact_data {
            println!("<<<<<<<<");
            print!("Name:  {}" , c.name);
            print!("Phone: {}" , c.phone);
            print!("Email: {}" , c.email);
            println!(">>>>>>>>");
            println!("");
        }
    }

    fn save(contact: Contact) {
        if fs::metadata(Contact::data_file_path()).is_ok() {
            let mut decoded_contacts = Contact::load_data();
            decoded_contacts.push(contact);

            let bytes: Vec<u8> = bincode::serialize(&decoded_contacts).unwrap();

            let mut file = Contact::create_data_file();

            let write = file.write_all(&bytes);

            match write {
                Ok(()) => { 
                    println!("Successfully saved contact.")
                }

                Err(error) => {
                    panic!("Something happening while trying to save contact to file: {:?}", error)
                }
            }
        } 
        
        else {
            
            let mut contacts: Vec<Contact> = Vec::new();
            contacts.push(contact);
   
            let bytes: Vec<u8> = bincode::serialize(&contacts).unwrap();
   
            let mut file = Contact::create_data_file();
   
            let write = file.write_all(&bytes);

            match write {
                Ok(()) => { 
                    println!("Successfully saved contact.")
                }
                
                Err(error) => {
                    panic!("Something happening while trying to save contact to file: {:?}", error)
                }
            }
        }

    }
}

fn cli_help() {
    println!("");
    println!("USAGE: contacts <command>");
    println!("");
    println!("Commands:");
    println!("\tadd   - Add a contact");
    println!("\tlist  - Displays a list of existing contacts");
    println!("\tcount - Display how many contacts you have");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    match args.len() {
        1 => {
            let cmd = &args[0].to_lowercase();
    
            match cmd.as_str() {
                "add"   => { Contact::create_prompt()  }
                "list"  => { Contact::load_all()       }
                "size"  => { Contact::display_count()  }
                _ => { 
                    println!("Command not recognized.");
                    cli_help();
                }
            }
        },

        _ => { cli_help(); }
    }

}
