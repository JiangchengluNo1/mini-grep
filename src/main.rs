use clap::{Parser};
use std::fs;
use colored::*;
/// Simple program to read a file and print its content
#[derive(Parser)]
#[command(version, about , long_about = "minigrep is for personal praticing program.")]
struct Args {

    /// The file to read
    #[arg(short = 'f', long)]
    file: String,
    /// the content to search for
    #[arg(short = 'q', long)]
    query: String,
    /// The above and behind number of the lines of aim content
    #[arg(short = 'n' , long, default_value_t = 5)]
    number: usize,
}

fn main() {
    let args = Args::parse();
    if is_available(args.query.clone()) == false {
        println!("{}", "Please input a valid query".red());
        return;
    }
    run(args);
    }

fn run(args: Args) {
    let content = fs::read_to_string(args.file.clone()).expect("No such file or could not read file");
    let lines = content.lines();
    let mut line_number = 1;
    let mut target_line_number:i32 = -1;
    let mut previous_lines = vec![];
    let mut find_flag = false;
    for line in lines {
        let query = args.query.clone();
        if !find_flag{
            if line.contains(&query) {
                if previous_lines.len()!=0{
                    for (offset_line_number,pre_line) in previous_lines.iter().enumerate(){
                        println!("{}: {}", (line_number-args.number+offset_line_number).to_string().blue().bold(),pre_line);
                    }
                }
                find_flag = true;
                target_line_number = line_number as i32;
                print!("{}:", target_line_number.to_string().blue().bold());
                let parts: Vec<&str> = line.split(&query).collect();
                    for (i, part) in parts.iter().enumerate() {
                        print!("{}", part);
                        if i < parts.len() - 1 {
                            print!("{}", query.green().bold());
                        }
                    }
                    println!();
            }else{
                if args.number != 0{
                    if previous_lines.len() < args.number{
                        previous_lines.push(line);
                    }else{
                        previous_lines.remove(0);
                        previous_lines.push(line);
                    }
                }
            }
        }else{
            if line_number <= (target_line_number as usize) + args.number {
                println!("{}: {}", line_number.to_string().blue().bold(), line);
            }
        }
        line_number += 1;
    }
}

fn is_available(query:String) -> bool {
    if query.len() == 0 {
        return false;
    }
    for c in query.chars() {
        if c.is_alphabetic() {
            return true;
        }
    }
    false
}