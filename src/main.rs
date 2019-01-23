extern crate clap;
extern crate crypto;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate bincode;


use clap::{App,SubCommand,Arg,ArgGroup};
use bincode::{serialize,deserialize};

mod passwords;



fn main() {

    let matches = App::new("opass")
                        .version("0.1.0")
                        .about("Simple cli password manager")
                        .arg(Arg::with_name("Password")
                              .short("p")
                              .long("password")
                              .value_name("STRING")
                              .help("set password")
                              .takes_value(true)
                              .required(true))
                        .arg(Arg::with_name("Database path")
                              .short("d")
                              .long("path")
                              .value_name("PATH")
                              .help("set path to password database")
                              .takes_value(true)
                              .required(true))
                        .arg(Arg::with_name("new")
                             .short("n"))
                        .subcommand(SubCommand::with_name("insert")
                                    .about("Insert a new entry")
                                    .arg(Arg::with_name("name")
                                         .short("n")
                                         .long("name")
                                         .takes_value(true)
                                         .required(true))
                                    .subcommand(SubCommand::with_name("key")
                                                .arg(Arg::with_name("key")
                                                     .short("k")
                                                     .long("key")
                                                     .takes_value(true)
                                                     .required(true)))
                                    .subcommand(SubCommand::with_name("set")
                                           .about("a set of username and password")
                                           .arg(Arg::with_name("Username")
                                                .short("u")
                                                .takes_value(true)
                                                .required(true))
                                           .arg(Arg::with_name("Password")
                                                .short("p")
                                                .takes_value(true)
                                                .required(true))))
                        .subcommand(SubCommand::with_name("delete")
                                    .about("Delete a entry"))
                        .subcommand(SubCommand::with_name("get")
                                    .about("Get a entry")).get_matches();
  
    let pass = matches.value_of("Password").unwrap();
    let path = matches.value_of("Database path").unwrap();

    let mut mybank = match matches.value_of("new") {
        Some(p) => {
            passwords::Entries::read(path.to_string(),pass.to_string())
        },
        None => {
            passwords::Entries::new(pass.to_string())
        }
    };
    match matches.subcommand_name() {
        Some("insert") => {
            let name = matches.value_of("name");
            match matches.subcommand_name() {
                Some("set") => {
                    let password = matches.value_of("Password");
                    let username = matches.value_of("Username");
                    print!("{:?}:{:?}:{:?}",name,password,username);
                },
                Some("key") => {
                },
                _ => {},
            }
        },
        Some("delete") => {
        },
        Some("get") => {
        }
        _ =>{
        },
    }
    
    mybank.write(path.to_string());
}
