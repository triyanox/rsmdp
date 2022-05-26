use regex::Regex;

pub(crate) fn parse_ordered_lists(tree: &str) -> String {
    let mut result = String::new();
    let ol_re = Regex::new(r"^\d+\. (.*)$").unwrap();
    let mut words = tree.split_whitespace();
    while let Some(word) = words.next() {
        if ol_re.is_match(word) {
            let captures = ol_re.captures(word).unwrap();
            result.push_str(
                format!(
                    "<ol> <li> {} </li> </ol>",
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
