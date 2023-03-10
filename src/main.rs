//! Simple Spelling Bee solver  
//! By Tal Zana January 2023  


use std::env;           // for command-line arguments
use std::fs;            // for file access
use std::io;            // for user input
use std::io::Write;     // for flushing print statement
use std::io::BufReader; // for reading file
use std::io::BufRead;   // for reading file
use std::io::Result;    // for reading line Result
use colored::*;         // for colored terminal output
use chrono::prelude::*; // for printing current time

const DICT_FILENAME: &str = "wordlist.txt";


// A few example tests
#[test]
fn is_valid_word_test() {
    assert!(is_valid_word("hello", "heloabc"));
    assert!(is_valid_word("world", "worldab"));

    assert!(!is_valid_word("hello", "helabcd"));
    assert!(!is_valid_word("world", "wrldabc"));
}

#[test]
fn is_pangram_test() {
    assert!(is_pangram("itinerants", "retains"));
    assert!(is_pangram("nastier", "retains"));
    assert!(is_pangram("stainer", "retains"));
    assert!(is_pangram("antistress", "retains"));

    assert!(!is_pangram("retints", "retains"));
    assert!(!is_pangram("insert", "retains"));
    assert!(!is_pangram("arena", "retains"));
    assert!(!is_pangram("eeriness", "retains"));
}


/// Struct that holds the maximum values obtained,
/// to be passed around in the recursion of run()
struct Max {
    // Mutability will be inherited by the individual fields
    pangrams: i32,
    perfect: i32,
    words: i32,
    score: i32,
    ratio: i32,
}

impl Default for Max {
    fn default() -> Self {
        Self {
            pangrams: 0,
            perfect: 0,
            words: 0,
            score: 0,
            ratio: 0,
        }
    }
}


/// Read a file into an optional Vector of individual lines
fn file_to_vector(filename: &String) -> Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    let mut dict: Vec<String> = file_reader.lines().filter_map(Result::ok).collect();

    // Filter unnecessary 3-letter words
    let length = dict.len();
    dict.retain(|w| w.len() > 3);

    // Uncomment the next line to exclude words containing the letter S
    // dict.retain(|w| !w.contains('s'));

    // Print dictionary information
    println!("\nLoaded dictionary from: {}", filename);
    println!("Dictionary has {}??words, filtered down to {} words.", length, dict.len());

    Ok(dict)
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
        io::stdout().flush().expect("Couldn???t write to stdout");

        // Read the user input and handle Result, otherwise we get a warning
        io::stdin().read_line(&mut user_input).expect("Couldn???t read from stdin");

        // Convert to lowercase in-place and remove newline
        user_input.make_ascii_lowercase();
        user_input.pop();
    }

    user_input
}


/// Return true if the dictionary word is:
/// - 4 or more characters
/// - Composed only of letters present in the target string
/// - Contains the central letter (first character in the string)
fn is_valid_word(word: &str, target: &str) -> bool {

    let mut valid = true;
    let center_letter = target.chars().nth(0).unwrap();

    for char in word.chars() {
        if !target.contains(char) || !word.contains(center_letter) {
            valid = false;
            break;
        }
    }

    valid
}


/// Return true if the word is a pangram,
/// i.e. if it contains all characters present in the target string
fn is_pangram(word: &str, target: &str) -> bool {
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
/// that can be created with the target string
fn solution<'a>(dict: &'a Vec<String>, target: &String) -> (Vec<&'a String>, i32, i32, i32) {

    // We need to use lifetimes in the signature because we are returning
    // a Vector of references to the Strings in the borrowed dict Vector

    // The center letter is the first character of the string
    // let center_letter = target.chars().nth(0).unwrap();
    let mut solution: Vec<&String> = Vec::new();
    let mut pangrams = 0;
    let mut score = 0;
    let mut perfect = 0;

    // Everything is a reference here,
    // including the words pushed onto the solution Vector
    // as well as the returned Vector itself
    for word in dict {
        if is_valid_word(word, target) {
            solution.push(word);

            // We apply the standard NYT scoring here...
            if word.len() == 4 {
                score += 1;
            } else {
                score += word.len() as i32;
            }

            // ...including bonus pangrams score
            if is_pangram(word, target) {
                pangrams += 1;
                score += 7;

                if word.len() == 7 {
                    perfect += 1;
                }
            }
        }
    }

    (solution, pangrams, perfect, score)
}


/// Pretty print the solution with highlighted pangrams
/// and some statistics
fn print_solution(solution: (Vec<&String>, i32, i32, i32), target: &String) {
    let (words, pangrams, perfect, score) = solution;
    println!("");
    for word in &words {
        if is_pangram(&word, &target) {
            if word.len() == 7 { // Perfect pangram
                print!("{}", format!("{word} ").green().bold());
            } else {             // Regular pangram
                print!("{}", format!("{word} ").red().bold());
            }
        } else {
            print!("{}", format!("{word} "));
        }
    }

    println!("\n\nWords: {} Score: {} Pangrams: {} Perfect: {}", words.len(), score, pangrams, perfect);
}


/// Print a line with info about the letter combination
/// - `max`: struct containing the maximum values associated with the letters
/// - `reason`: string indicating the reason for the new max value (one of the 5 fields of Max)
fn print_max(max: &Max, word: &str, reason: &ColoredString) {
    println!("\r{:<38} -- {:<8}{:>6}{:>4}{:>4}{:>6} ({:>2}) -- {:<10}",
        Local::now(),
        word,
        max.words, max.pangrams, max.perfect, max.score, max.ratio,
        reason);
}


/// Recursive function that prints the maximum word count and pangram value
/// for all valid letter combinations.
/// Runs when the user inputs 'maximum'.
/// We need to pass all these references in order to track maximum values.
fn run(
    target: &mut String,
    dict: &Vec<String>,
    max: &mut Max
) {

    // Depth of recursion is equal to the length of the string
    let depth = target.len();

    // We run through all the letters of the alphabet
    for a in 'a'..='z' {

        // Append the letter to the string in one of three cases:
        // - The string is empty
        // - It's the second letter and it's different than the first (central) letter
        // - It's any other letter not already found in the string and its value
        //   is larger than the previous letter in the string
        // This avoids testing equivalent permutations e.g. abcdefg and acdefgb
        if depth == 0 ||
            depth == 1 && !target.contains(a) ||
            (depth > 1 && !target.contains(a) && target.chars().nth(depth-1).unwrap() < a) {

            // Print some idea of the function's progress
            if depth > 2 {
                print!("\r{}{}{}....", target.chars().nth(0).unwrap(), target.chars().nth(1).unwrap(), target.chars().nth(2).unwrap());
            }
            
            // Add the letter to the string
            target.push(a);

            // We are looking for a 7-letter string so we stop at 6
            if depth == 6 {

                // Get the solution for the string
                let (solution, pangrams, perfect, score) = solution(dict, &target);
                let words = solution.len() as i32;
                let ratio = if words > 0 { pangrams * 100 / words } else { 0 };

                // Print new maximums if it is the case
                if pangrams > max.pangrams {
                    print_max(&max, &target, &"pangrams".red());
                    max.pangrams = pangrams;
                }
                if perfect > max.perfect {
                    print_max(&max, &target, &"perfect".green());
                    max.perfect = perfect;
                }
                if words > max.words {
                    print_max(&max, &target, &"words".normal());
                    max.words = words;
                }
                if ratio > max.ratio && words > 9 { // 50% ratio with 2 words isn't fun
                print_max(&max, &target, &"ratio".yellow());
                    max.ratio = ratio;
                }
                if score > max.score {
                    print_max(&max, &target, &"score".bright_blue());
                    max.score = score;
                }

            } else {
                // If the string is shorter than 7 letters, recurse
                run(target, dict, max);
            }

            // Remove the last letter of the string
            // before continuing to the next iteration of the a..z for loop
            target.pop();
        }
    }
}


/// Main interactive loop for entering letters and printing the solution
fn main_loop(dict: &Vec<String>) {
    loop {
        // Get 7-letter target string from the user
        // Center letter should be the first element in the string
        let target = get_target_letters();

        // If the string is 'maximum', call the maximum searching function
        // Otherwise, display the solution
        if target == "maximum" {
            run(&mut String::new(), &dict, &mut Max::default());
        } else {
            print_solution(solution(dict, &target), &target);
        }
    }
}


fn main() {
    // Build the full path to the wordlist.txt file,
    // expected to be in the executable directory
    let mut dict_path = env::current_exe().expect("Executable not found.");
    dict_path.pop();
    dict_path.push(DICT_FILENAME);

    // dict_path is a PathBuf type
    // Convert it to a string
    let dict_path_str = dict_path.into_os_string().into_string().expect("Couldn't convert dictionary path to string.");

    // Load the dictionary
    let dict = match file_to_vector(&dict_path_str) {
        Ok(dict) => dict,
        Err(_) => Vec::<String>::new()
    };

    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Only run if the dictionary has been read properly,
    // then check for command-line arguments
    if dict.is_empty() {
        println!("\nProblem opening the dictionary file (is \"wordlist.txt\" in the current directory?)");
    } else {
        if args.len() > 1 {
            match args[1].as_str() {
                // Execute the recursive search function for maxium values
                "run" => {
                    println!("");
                    run(&mut String::new(), &dict, &mut Max::default());
                },
                _ => println!("\nUnrecognised argument."),
            }
        } else {
            main_loop(&dict);
        }
    }
}
