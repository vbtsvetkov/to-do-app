use crate::state::{read_file, write_to_file};
use crate::to_do::structs::traits::create::Create;
use serde_json::json;
use std::env;

mod to_do;
mod state;

const PATH: &str = "./state.json";

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let status = &args[1];
    let title = &args[2];
    
    let mut state = read_file(PATH);
    println!("{:?}", state);
    
    state.insert(title.to_string(), json!(status));
    write_to_file(PATH, state);
    
}
