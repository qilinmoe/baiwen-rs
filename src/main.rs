use std::fs;

use std::collections::HashSet;
use std::path::Path;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use clap::{App, Arg};


// Base of the JSON objects:
#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Container")]
    container: String,
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "PathID")]
    pathid: i64,
    #[serde(rename = "Type")]
    type_: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>>  {

    println!("--  Baiwen [v0.1.0]  --"); // Just a little title :)

        // -- Read arguments from cli...

    let matches = App::new("Baiwen")
        .version("0.1.0")
        .author("Angie <justangielical@gmail.com>")
        .about("Parses the JSON maps a certain program builds and matches a string and type to a source file")
        .arg(
            Arg::with_name("path")
                .long("path")
                .value_name("FILE")
                .help("Sets the input file to use")
                .takes_value(true)
                .required(true)
            )
        .arg(
            Arg::with_name("string")
                .long("string")
                .value_name("STRING")
                .help("Sets the string to match")
                .takes_value(true)
                .required(true),
            )
        .arg(
            Arg::with_name("type")
                .long("type")
                .value_name("TYPE")
                .help("Sets the type(s) to match, comma separated")
                .takes_value(true)
                .default_value("GameObject"),
            ).get_matches();



    let file_path = matches.value_of("path").unwrap();
    let string_to_match = matches.value_of("string").unwrap();
    let type_to_match = matches.value_of("type").unwrap();

    let user_types: Vec<&str> = type_to_match.split(',').collect();
    let valid_types = vec!["GameObject", "Mesh", "Texture2D", "Animator", "Material"];

    for user_type in &user_types {
        if !valid_types.contains(user_type) {
            println!("Error: '{}' is not a recognized type. Please use one of the following valid types:", user_type);
            for t in &valid_types {
                println!("> {}", t);
            }
            return Ok(());
        }
    }

        // -- Done with argument parsing

    println!("> Trying to match [ {} ] with type [ {} ] in [ {} ]", string_to_match, type_to_match, Path::new(file_path).file_name().and_then(|name| name.to_str()).unwrap_or(file_path));


    // Start a timer for time checking.
    let start = Instant::now();


    // Read file contents and load it as JSON.
    let file_content = fs::read_to_string(file_path)?;
    let entries: Vec<Entry> = serde_json::from_str(&file_content)?;



        // Parse entries.
    
    let mut unique_sources = HashSet::new();

    for entry in entries {
        if entry.name.contains(string_to_match) && user_types.contains(&entry.type_.as_str()) {
            unique_sources.insert(entry.source.clone());
        }
    }

    // End timer.
    let duration = start.elapsed();

        // -- Done with entry parsing.
    
    println!("> Done! ({:?})\n", duration);
    println!("> Unique sources that contain a matching name and type:");

    // Print all files that contain at least one object with matching both name and type.
    for source in &unique_sources {
        println!("  > {}", 
            Path::new(source)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(source)  // In case we can't get the file name, we'll default to the full source
        );
    }

    // Done!!
    Ok(())
}