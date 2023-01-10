//! Simple Spelling Bee solver  
//! By Tal Zana January 2023  


use std::fs;            // for file access
use std::io;            // for user input
use std::io::Write;     // for flushing print statement
use std::io::BufReader; // for reading file
use std::io::BufRead;   // for reading file
use colored::*;         // for colored terminal output
use std::io::Result;    // for optional Result
use chrono::prelude::*; // for printing current time


/// Read a file into an optional Vector of individual lines
fn file_to_vector(filename: String) -> Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(Result::ok).collect())
}


/// Get the seven letters from the user
fn get_target_letters() -> String {
    // Define the user input variable as a String
    let mut user_input = String::new();

    // Wait for a 7-letter string
    while user_input.len() != 7 {
        // Reset the input string
        user_input = "".to_string();
        
        // Display prompt on one line
        print!("\nEnter 7 unique letters with the center letter first.\n> ");
        io::stdout().flush().expect("Couldn’t write to stdout");

        // Read the user input and handle Result, otherwise we get a warning
        io::stdin().read_line(&mut user_input).expect("Couldn’t read from stdin");

        // Convert to lowercase in-place and remove newline
        user_input.make_ascii_lowercase();
        user_input.pop();
    }

    user_input
}


/// Return true if the word only contains characters
/// which are part of the target string
fn is_valid_word(word: &String, target: &String) -> bool {
    let mut valid = true;
    for char in word.chars() {
        if !target.contains(char) {
            valid = false;
            break;
        }
    }

    valid
}


/// Return true if the word is a pangram,
/// i.e. if it contains all characters present in the target string
fn is_pangram(word: &String, target: &String) -> bool {
    let mut n = 0;
    for char in target.chars() {
        if word.contains(char) {
            n += 1;
        }
    }

    if n == 7 {
        true
    } else {
        false
    }
}


/// Return a Vector containing references to all the words in the dictionary
/// which can be created with the target string
fn solution<'a>(dict: &'a Vec<String>, target: &String) -> (Vec<&'a String>, i32) {

    // We need to use lifetimes in the signature because we are returning
    // a Vector of references to the Strings in the borrowed dict Vector

    let center_letter = target.chars().nth(0).unwrap();
    let mut solution: Vec<&String> = Vec::new();
    let mut pangrams = 0;

    // Everything is a reference here,
    // including the words pushed onto the solution Vector
    // as well as the returned Vector itself
    for word in dict {
        if word.len() > 3 && word.contains(center_letter) {
            if is_valid_word(word, target) {
                solution.push(word);
                if is_pangram(word, target) {
                    pangrams += 1;
                }
            }
        }
    }

    (solution, pangrams)
}


/// Pretty print the solution with pangrams in yellow
/// and some statistics
fn print_solution(solution: (Vec<&String>, i32), target: &String) {
    let (words, pangrams) = solution;
    println!("");
    for word in &words {
        if is_pangram(&word, &target) {
            print!("{}", format!("{word} ").bright_yellow().bold());
        } else {
            print!("{}", format!("{word} "));
        }
    }

    println!("\n\nWords: {} Pangrams: {}", words.len(), pangrams);
}


/// Main interactive loop for entering letters and printing the solution
fn main_loop(dict: &Vec<String>) {
    loop {
        // Get 7-letter target string from the user
        // Center letter should be the first element in the string
        let target = get_target_letters();

        if target == "maximum" {
            run(&mut String::new(), &dict, &mut 0, &mut 0);
        }

        let solution = solution(dict, &target);
        print_solution(solution, &target);
    }
}


fn main() {
    // Read the word dictionary from the file
    let dict = match file_to_vector("./wordlist.txt".to_string()) {
        Ok(dict) => dict,
        // Err(why) => panic!("Problem opening the dictionary file (is \"wordlist.txt\" in the current directory?) {:?}", why),
        Err(_) => Vec::<String>::new()
    };

    // Only run if the dictionary has been read properly
    if dict.is_empty() {
        println!("\nProblem opening the dictionary file (is \"wordlist.txt\" in the current directory?)");
    } else {
        main_loop(&dict);
    }
}

fn run(target: &mut String, dict: &Vec<String>, max_pangrams: &mut i32, max_words: &mut i32) {
    for a in 'a'..='z' {
        if !target.contains(a) {
            target.push(a);
            if target.len() == 7 {
                let (solution, pangrams) = solution(dict, &target);
                if pangrams > *max_pangrams {
                    println!("{} -- Pangrams: {} {:>3} {:>3}", Local::now(), target, solution.len(), pangrams);
                    *max_pangrams = pangrams;
                } else if solution.len() > *max_words as usize {
                    println!("{} -- Words:    {} {:>3} {:>3}", Local::now(), target, solution.len(), pangrams);
                    *max_words = solution.len() as i32;
                }
            } else {
                run(target, dict, max_pangrams, max_words);
            }
            target.pop();
        }
    }
} 
