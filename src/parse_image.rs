use regex::Regex;

pub(crate) fn parse_image(tree: &str) -> String {
    let mut result = String::new();
    let image_re = Regex::new(r"!\[(.*)\]\((.*)\)").unwrap();
    let mut words = tree.split_whitespace();
    while let Some(word) = words.next() {
        if image_re.is_match(word) {
            let captures = image_re.captures(word).unwrap();
            result
                .push_str(format!("<img src=\"{}\"/>", captures.get(2).unwrap().as_str()).as_str());
        } else {
            result.push_str(format!("{} ", word).as_str());
        }
    }
    result
}
