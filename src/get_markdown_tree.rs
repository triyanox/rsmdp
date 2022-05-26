use std::fs::File;
use std::io::prelude::*;

pub(crate) fn get_markdown_tree(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}
