use std::env;
use rand::Rng;

fn print_help () {          // print help message and exit
    println!("Usage: randomstringgen [FLAGS] [SIZE]");
    println!("FLAGS:");
    println!("\t-u\t\tInclude uppercase letters");
    println!("\t-l\t\tInclude lowercase letters");
    println!("\t-n\t\tInclude numbers");
    println!("\t-p\t\tInclude punctuation");
    println!("\t-a\t\tInclude all characters");
    println!("\t-h\t\tDisplay this help message");
    // and exit
}

fn generate_string (size: usize, uppercase: bool, lowercase: bool, numbers: bool, punctuation: bool, all: bool) -> String {
    let mut string = String::new();
    let mut string_chars = String::new(); // Change the type to String

    let mut rng = rand::thread_rng();
    // println!("size {}\nuppercase {}\nlowercase {}\nnumbers {}\npunctuation {}\nall {}", size, uppercase, lowercase, numbers, punctuation, all);

    if uppercase {
        string_chars.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if lowercase {
        string_chars.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if numbers {
        string_chars.push_str("0123456789");
    }
    if punctuation {
        string_chars.push_str("!@#$%^&*()-_=+[]{};:'\",.<>/?");
    }
    if all {
        string_chars.clear();
        string_chars.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+[]{};:'\",.<>/?");
    }
    if !uppercase && !lowercase && !numbers && !punctuation && !all {
        string_chars.push_str("abcdefghijklmnopqrstuvwxyz0123456789");
    }
    
    let string_chars: Vec<char> = string_chars.chars().collect();
    for _ in 0..size {
        let index = rng.gen_range(0..string_chars.len());
        string.push(string_chars[index]);
    }
    
    string
}

fn main () {
    let args: Vec<String> = env::args().collect();
    // let argumentnamegoeshere = args.iter().any(|arg| arg == "flaggoeshere");

    // usage: defualt string generated is 12 chars long and contains only lowercase letters and numbers
    // usage: if only  integer is provided it will be used as the length of the string
    // usage: the -u flag can be used to include uppercase letters
    // usage: the -l flag can be used to include lowercase letters
    // usage: the -n flag can be used to include numbers
    // usage: the -p flag can be used to include punctuation
    // usage: the -a flag can be used to include all characters
    // usage: the -h flag can be used to display the help message
    // usage: if any flags are provided the others are assumed to be false
    // usage: if any flags are malformed just spit out the help monologue instead of letting the program panic

    let mut size = 12;
    let mut uppercase = false;
    let mut lowercase = true;
    let mut numbers = true;
    let mut punctuation = false;
    let mut all = false;

    if args.len() > 1 {
        uppercase = false;
        lowercase = false;
        numbers = false;
        punctuation = false;
        all = false;
        for arg in &args[1..] {
            match arg.as_str() {
                "-h" => {
                    print_help();
                    return;
                },
                "-u" => {
                    uppercase = true;
                },
                "-l" => {
                    lowercase = true;
                },
                "-n" => {
                    numbers = true;
                },
                "-p" => {
                    punctuation = true;
                },
                "-a" => {
                    all = true;
                },

                _ => {
                    if arg.starts_with("-") {
                        print_help();
                        std::process::exit(1);
                    } else if !arg.parse::<usize>().is_ok() {
                        print_help();
                        std::process::exit(1);
                    } else {
                        size = arg.parse::<usize>().unwrap();
                    }
                }
            }
        }
    }
    println!("{}", generate_string(size, uppercase, lowercase, numbers, punctuation, all));
}