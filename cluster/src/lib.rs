// write a function that takes user input and sends the appropriate output to helpers/create-cluster.json

fn main() {
    // get user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // send user input to lib.rs
    lib::create_cluster(input);
}