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
mod parse_styling;
mod parse_styling_in_tree;
mod replace;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut markdown_tree = get_markdown_tree::get_markdown_tree(filename);

    markdown_tree = parse_line_in_tree::parse_line_in_tree(&markdown_tree);
    markdown_tree = parse_code::parse_code(&markdown_tree);
    markdown_tree = parse_styling_in_tree::parse_styling_in_tree(&markdown_tree);
    markdown_tree = parse_inline_code::parse_inline_code(&markdown_tree);
    markdown_tree = parse_image::parse_image(&markdown_tree);
    markdown_tree = parse_link::parse_link(&markdown_tree);

    let mut html = String::from(
        "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>Markdown</title>\n</head>\n<body>\n",
    );

    html.push_str(&markdown_tree);
    html.push_str("\n</body>\n</html>");

    let new_name = filename.split(".").next().unwrap();

    let mut html_file = File::create(format!("{}.html", new_name)).expect("file not found");
    html_file
        .write_all(html.as_bytes())
        .expect("something went wrong writing the file");

    println!("File created: {}.html", new_name);
}
