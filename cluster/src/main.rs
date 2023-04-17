// write main function that calls helpers/create-cluster.json

fn main() {
    // get user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // send user input to lib.rs
    lib::create_cluster(input);
}