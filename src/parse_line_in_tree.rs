use crate::parse_line;

pub(crate) fn parse_line_in_tree(tree: &str) -> String {
    let mut lines = tree.lines();
    let mut result = String::new();
    while let Some(line) = lines.next() {
        result.push_str(&parse_line::parse_line(line));
    }
    return result;
}
