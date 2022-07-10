extern crate termion;

use termion::{color};
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod get_markdown_tree;
mod parse_code;
mod parse_image;
mod parse_inline_code;
mod parse_line;
mod parse_line_in_tree;
mod parse_link;
mod parse_ordred_lists;
mod parse_styling;
mod parse_styling_in_tree;
mod replace;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: rsmdp <file>");
        return;
    }
    if &args[1] == "--help" {
        println!("{}* Create the file in the current directory" , color::Fg(color::Green));
        println!("{}-> Usage: rsmdp <file>" , color::Fg(color::Black));
        println!("-> Example: rsmdp README.md");
        println!("{}* Create the file in a specific directory" , color::Fg(color::Green));
        println!("{}-> Usage: rsmdp <file> <directory>" , color::Fg(color::Black));
        println!("-> Example: rsmdp README.md /Desktop");
        return;
    }
    let filename = &args[1];

    let mut markdown_tree = get_markdown_tree::get_markdown_tree(filename);

    markdown_tree = parse_line_in_tree::parse_line_in_tree(&markdown_tree);
    markdown_tree = parse_code::parse_code(&markdown_tree);
    markdown_tree = parse_styling_in_tree::parse_styling_in_tree(&markdown_tree);
    markdown_tree = parse_inline_code::parse_inline_code(&markdown_tree);
    markdown_tree = parse_image::parse_image(&markdown_tree);
    markdown_tree = parse_link::parse_link(&markdown_tree);
    markdown_tree = parse_ordred_lists::parse_ordered_lists(&markdown_tree);
    
    let new_name = filename.split(".").next().unwrap();


    let mut html = String::from(
        "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>Markdown</title>\n</head>\n<body>\n"
    );

    html.push_str(&markdown_tree);
    html.push_str("\n</body>\n</html>");

    let lightening_unicode = String::from("\u{1F5F2}");

    let mut html_file;
    if args.len() == 3 {
        let write_path = &args[2];
        html_file = File::create(format!("{}/{}.html", write_path, new_name)).expect("file not found");
        html_file
        .write_all(html.as_bytes())
        .expect("something went wrong writing the file");
    } else {
        html_file = File::create(format!("{}.html", new_name)).expect("file not found");
        html_file
        .write_all(html.as_bytes())
        .expect("something went wrong writing the file");
    }

    println!(" {}  {}File created: {}.html", lightening_unicode, color::Fg(color::Green), new_name);
}
