use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Cluster Management Tool")
        .version("1.0")
        .author("Your Name <your.name@example.com>")
        .about("Manage clusters")
        .subcommand(
            SubCommand::with_name("create")
                .about("Create a new cluster")
                .arg(
                    Arg::with_name("name")
                        .help("Name of the new cluster")
                        .required(true),
                )
                .arg(
                    Arg::with_name("size")
                        .help("Size of the new cluster")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("edit").about("Edit an existing cluster"))
        .subcommand(SubCommand::with_name("delete").about("Delete an existing cluster"))
        .get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap();
            let size = sub_matches
                .value_of("size")
                .unwrap_or_else(|| matches.value_of("size").unwrap());
            println!("Creating cluster '{}' with size {}", name, size);
        }
        Some(("edit", _)) => println!("Editing cluster..."),
        Some(("delete", _)) => println!("Deleting cluster..."),
        None => println!("No subcommand specified."),
        _ => unreachable!(),
    }
}