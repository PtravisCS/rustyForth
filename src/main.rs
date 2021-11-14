/*
* Title             : rustyPForth
* Purpose           : A FORTH interpreter written in Rust. Intended to implement a modified FORTH
* syntax that improves string handling in the language at a slight cost to memory efficiency and speed.
*
* Created           : 6/12/2021
* Author            : Paul Travis
* Last Modified     : 6/12/2021
* Last Modified By  : Paul Travis
*
* Dependancies      : None
*/

#[macro_use]
extern crate clap;
use clap::App;
use std::collections::HashMap;
use std::vec::Vec;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Word {

    word_type: String,
    pointer: fn(&mut Vec<String>),
    defenition: String 

}

fn dup(stack: &mut Vec<String>) {

    stack.push(stack.last().unwrap().to_string());

}

fn print_stack(stack: &mut Vec<String>) {

    for i in 0..stack.len() {

        println!("[{}]: {}", i, stack.get(i).unwrap());

    }

}

fn load_file(file_path: String) -> Vec<String> {

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut file_text: Vec<String> = Vec::new();

    for (index, line) in reader.lines().enumerate() {

        file_text.push(line.unwrap());

    }

    return file_text;

}

fn parse_line(line: String, stack: &mut Vec<String>, words: HashMap<String, fn(&mut Vec<String>)>) -> Result<(), String> {

    let instructions = line.split(" ").collect();

    for instruction in instructions {

        if words.contains_key(instruction) {

            let word = words.get(instruction).unwrap();
            
            if (word.word_type == "function") {

                let func = word.pointer;
                func(stack);

            } else if (word.word_type == "user_defined") {

                parse_line(word.definition, stack, words);
                
            }

        } else {

            stack.push(instruction);            

        }

    }

}

fn str_is_numeric(str: String) -> bool {

    for ch in str.chars() {

        if !ch.is_numeric() {

            return false;

        }

    }

    return true;

}

fn main() {

    //Load the Clap config and then process the user input operators
    let yaml = load_yaml!("opts.yml");
    let matches = App::from_yaml(yaml).get_matches();

    //Get the path to the input file
    let file_path = matches.value_of("file").unwrap();

    //Check if interactive mode was invoked
    let interactive: bool = matches.is_present("interactive");

    let file: Vec<String> = load_file(file_path.to_string());

    let mut stack: Vec<String> = Vec::new();

    let mut words: HashMap<String, Word> = HashMap::new();

    let struct_dup = Word {

        word_type: "function",
        pointer: dup,
        defenition: ""

    };

    words.insert("dup".to_string(), struct_dup);

    //let func = words.get("dup").unwrap();

    //func(&mut stack);

}
