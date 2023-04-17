use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use serde::{Serialize};

#[derive(Serialize)]
struct Cluster {
    cluster_name: String,
    spark_version: String,
    node_type: String,
    spark_conf: serde_json::Value,
    num_workers: u32,
}

fn get_node_type(purpose: &str) -> &str {
    /*Takes desired purpose of cluster (string) and returns type of cluster we will use string.
    String returned is the name of the compute resource we want to use.
    Hardcoded into four sections for now.*/
    match purpose {
        "general" => "Standard_D96ds_v5",
        "memory" => "Standard_E96ds_v5",
        "storage" => "Standard_L80as_v3",
        "compute" => "Standard_F72s_v2",
        _ => "Standard_D96ds_v5", // default to general
    }
}

// make sure create_cluster returns Restul or Option to accept ?
fn create_cluster(name: &str, purpose: &str) -> Result<(), Box<dyn std::error::Error>> {
    let node_type = get_node_type(purpose);
    println!("Creating cluster '{}' with size {}", name, node_type);

    // create json file with cluster info
    let cluster = Cluster {
        cluster_name: String::from(name),
        spark_version: String::from("7.3.x-scala2.12"),
        node_type: String::from(node_type),
        spark_conf: serde_json::json!({
            "spark.speculation": true
        }),
        num_workers: 25,
    };

    let json = serde_json::to_string_pretty(&cluster)?;

    let mut file = File::create("utils/cluster.json")?;
    file.write_all(json.as_bytes())?;

    // run command line to create cluster
    let _output = Command::new("databricks")
        .arg("clusters")
        .arg("create")
        .arg("--json-file")
        .arg("utils/cluster.json")
        .output()
        .expect("failed to execute process");

    Ok(())
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
                        .required(true)
                        .index(1),

                )
                .arg(
                    Arg::with_name("optimize")
                        .help("Optimize for a specific purpose")
                        .required(false)
                        .default_value("general")
                        .index(2),

                ),
        )
        .subcommand(SubCommand::with_name("delete")
        .about("Delete an existing cluster")
        .arg(
            Arg::with_name("cluster-id")
                .help("ID of the cluster to delete")
                .required(true)
                .index(1),
        )
    )
        .subcommand(SubCommand::with_name("list").about("List all clusters"))
        .get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap();
            let _output = create_cluster(name, sub_matches.value_of("optimize").unwrap());
            println!("Cluster created!");
        }
        Some(("delete", _)) => { println!("Deleting cluster...");
            let cluster_id = matches.value_of("cluster-id").unwrap();
            println!("Deleting cluster {}...", cluster_id);
            let _output = Command::new("databricks")
                .arg("clusters")
                .arg("delete")
                .arg("--cluster-id")
                .arg(cluster_id)
                .output()
                .expect("failed to execute process");

    }


        Some(("list", _)) =>  {
            
            println!("Listing clusters...");
            let _output = Command::new("databricks")
                .arg("clusters")
                .arg("list")
                .output()
                .expect("failed to execute process");

            let output = String::from_utf8_lossy(&_output.stdout);
            println!("{}", output);

        }
        None => println!("No subcommand specified."),
        _ => unreachable!(),
    }
}