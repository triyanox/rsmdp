pub(crate) fn parse_code(tree: &str) -> String {
    let mut result = String::new();
    let mut words = tree.split_whitespace();
    while let Some(word) = words.next() {
        if word == "```" {
            result.push_str("<pre><code>");
            while let Some(word) = words.next() {
                if word == "```" {
                    result.push_str("</code></pre>");
                    break;
                } else {
                    result.push_str(format!("{} ", word).as_str());
                }
            }
        } else {
            result.push_str(format!("{} ", word).as_str());
        }
    }
    result
}
