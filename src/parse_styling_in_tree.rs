use crate::parse_styling::parse_styling;

pub(crate) fn parse_styling_in_tree(tree: &str) -> String {
    let mut result = String::new();
    let mut words = tree.split_whitespace();
    while let Some(word) = words.next() {
        result.push_str(format!("{} ", parse_styling(word)).as_str());
    }
    result
}
