use clap::{App, Arg, SubCommand};

fn get_size(size: &str) -> &str {
    /*Takes desired purpose of cluster (string) and returns type of cluster we will use string.
    String returned is the name of the compute resource we want to use.
    Hardcoded into four sections for now.*/
    match size {
        "general" => "Standard_D96ds_v5",
        "memory" => "Standard_E96ds_v5",
        "storage" => "Standard_L80as_v3",
        "compute" => "Standard_F72s_v2",
        _ => "Standard_D96ds_v5", // default to general
    }
}

fn main() {
    let matches = App::new("Cluster Management Tool")
        .version("1.0")
        .author("Alex Bzdel")
        .about("Manage clusters")
        .subcommand(
            SubCommand::with_name("create")
                .about("Create a new cluster")
                .arg(
                    Arg::with_name("name")
                        .help("Name of the new cluster")
                        .short('n')
                        .required(true),

                )
                .arg(
                    Arg::with_name("optimize")
                        .help("Optimize for a specific purpose")
                        .short('o')
                        .required(false)
                        .default_value("general"),

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