extern crate clap;

use clap::{App,SubCommand};

fn main() {
    let matches = App::new("opass")
                        .version("0.1.0")
                        .about("Simple cli password manager")
                        .subcommand(SubCommand::with_name("insert")
                                    .about("Insert a new entry"))
                        .subcommand(SubCommand::with_name("delete")
                                    .about("Delete a entry"))
                        .subcommand(SubCommand::with_name("get")
                                    .about("Get a entry")).get_matches();
    

}
