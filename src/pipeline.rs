use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};

use crate::*;

pub fn receive_input_from_r() -> Vec<(usize, Vec<Groups>)> {
    let mut input_json = String::new();
    io::stdin().read_to_string(&mut input_json).expect("Failed to read from stdin");

    serde_json::from_str(&input_json).expect("Failed to deserialize JSON")
}

// Function to send JSON output to R
pub fn send_output_to_r<T: Serialize>(output: T) {
    let json_data = serde_json::to_string(&output).expect("Failed to serialize data to JSON");

    // Print JSON to standard output
    print!("{}", json_data);
    io::stdout().flush().expect("Failed to flush stdout");
}