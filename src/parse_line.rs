use regex::Regex;

pub(crate) fn parse_line(line: &str) -> String {
    let h1_re = Regex::new(r"^# (.*)$").unwrap();
    let h2_re = Regex::new(r"^## (.*)$").unwrap();
    let h3_re = Regex::new(r"^### (.*)$").unwrap();
    let h4_re = Regex::new(r"^#### (.*)$").unwrap();
    let h5_re = Regex::new(r"^##### (.*)$").unwrap();
    let h6_re = Regex::new(r"^###### (.*)$").unwrap();
    if h1_re.is_match(line) {
        return format!(
            "<h1> {} </h1>",
            h1_re.captures(line).unwrap().get(1).unwrap().as_str()
        );
    } else if h2_re.is_match(line) {
        return format!(
            "<h2> {} </h2>",
            h2_re.captures(line).unwrap().get(1).unwrap().as_str()
        );
    } else if h3_re.is_match(line) {
        return format!(
            "<h3> {} </h3>",
            h3_re.captures(line).unwrap().get(1).unwrap().as_str()
        );
    } else if h4_re.is_match(line) {
        return format!(
            "<h4> {} </h4>",
            h4_re.captures(line).unwrap().get(1).unwrap().as_str()
        );
    } else if h5_re.is_match(line) {
        return format!(
            "<h5> {} </h5>",
            h5_re.captures(line).unwrap().get(1).unwrap().as_str()
        );
    } else if h6_re.is_match(line) {
        return format!(
            "<h6> {} </h6>",
            h6_re.captures(line).unwrap().get(1).unwrap().as_str()
        );
    } else {
        return format!("<p> {} </p>", line);
    }
}
