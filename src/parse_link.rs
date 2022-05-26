use regex::Regex;

pub(crate) fn parse_link(tree: &str) -> String {
    let mut result = String::new();
    let link_re = Regex::new(r"\[(.*)\]\((.*)\)").unwrap();
    let mut words = tree.split_whitespace();
    while let Some(word) = words.next() {
        if link_re.is_match(word) {
            let captures = link_re.captures(word).unwrap();
            result.push_str(
                format!(
                    "<a href=\"{}\">{}</a>",
                    captures.get(2).unwrap().as_str(),
                    captures.get(1).unwrap().as_str()
                )
                .as_str(),
            );
        } else {
            result.push_str(format!("{} ", word).as_str());
        }
    }
    result
}
