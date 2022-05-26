pub(crate) fn replace(string: &str, reg_ex: &str, replacement: &str) -> String {
    let re = regex::Regex::new(reg_ex).unwrap();
    re.replace_all(string, replacement).to_string()
}
