use std::env;
use std::fs;

fn main() {
    let args: Vec<string> = env::args().collect();

    let (query, file_path) = read_configs(&args);
}


fn read_configs(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = %args[2];

    (query, file_path);
}
