use pulldown_cmark::{html::push_html, Options, Parser};

// pub fn markdown_to_html(markdown_text: String) -> String {
//     let text = parse_text_to_html(&markdown_text);
//     text
// }

pub fn parse_text_to_html(value: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(&value, options);
    let mut parsed_text = String::new();
    push_html(&mut parsed_text, parser);
    parsed_text
}
