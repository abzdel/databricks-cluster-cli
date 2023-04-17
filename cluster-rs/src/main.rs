// use clap with app and arg
extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Databricks Cluster Manager")
        .version("1.0")
        .author("Alex Bzdel")
        .about("A command-line tool for managing Databricks clusters")
        .arg(Arg::with_name("name")
             .short('n')
             .long("name")
             .value_name("NAME")
             .help("Sets the name of the cluster")
             .required(true))
        .arg(Arg::with_name("size")
             .short('s')
             .long("size")
             .value_name("SIZE")
             .help("Sets the size of the cluster")
             .required(true))
        .get_matches();

    let name = matches.value_of("name").unwrap();
    let size = matches.value_of("size").unwrap();

    println!("Creating cluster {} with size {}...", name, size);
}
