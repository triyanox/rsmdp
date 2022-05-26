use crate::replace;
use regex::Regex;

pub(crate) fn parse_styling(word: &str) -> String {
    let bold_re = Regex::new(r"\*\*(.*)\*\*").unwrap();
    let italic_re = Regex::new(r"\*(.*)\*").unwrap();
    let strike_re = Regex::new(r"~~(.*)~~").unwrap();
    let inline_code_re = Regex::new(r"`(.*)`").unwrap();
    let italic_with_spaces_re = Regex::new(r"_(.*)_").unwrap();
    if bold_re.is_match(word) {
        return replace::replace(word, r"\*\*(.*)\*\*", "<b>$1</b>");
    } else if italic_re.is_match(word) {
        return replace::replace(word, r"\*(.*)\*", "<i>$1</i>");
    } else if strike_re.is_match(word) {
        return replace::replace(word, r"~~(.*)~~", "<s>$1</s>");
    } else if inline_code_re.is_match(word) {
        return replace::replace(word, r"`(.*)`", "<pre><code>$1</code></pre>");
    } else if italic_with_spaces_re.is_match(word) {
        return replace::replace(word, r"_(.*)_", "<i>$1</i>");
    } else {
        return word.to_string();
    }
}
