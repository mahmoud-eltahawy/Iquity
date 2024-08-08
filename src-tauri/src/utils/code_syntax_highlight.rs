use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

#[derive(Debug)]
pub struct CodeBlock {
    begin: usize,
    end: usize,
    content: String,
    lang: String,
}

pub fn code_syntax_highlight(source: &str) -> String {
    let mut source = source.chars().collect::<Vec<_>>();
    let codes = extract_code_blocks(&source);
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];
    for code in codes {
        let syntax = ps
            .find_syntax_by_extension(&code.lang)
            .unwrap_or(ps.find_syntax_by_extension("sh").unwrap());
        let html = highlighted_html_for_string(&code.content, &ps, syntax, theme).unwrap();
        source.splice(code.begin..(code.end + 3), html.chars().collect::<Vec<_>>());
    }

    String::from_iter(source)
}

pub fn extract_code_blocks(source: &[char]) -> Vec<CodeBlock> {
    let mut all_blocks = Vec::<CodeBlock>::new();
    let mut code_block = Vec::<char>::new();
    let mut code_lang = Vec::<char>::new();
    let mut code_began = None::<usize>;
    let mut lang_done = false;
    for (i, three_chars) in source.windows(3).enumerate() {
        let three_chars_are_seperator = three_chars.iter().all(|x| *x == '`');
        match code_began {
            Some(begin) => {
                if three_chars_are_seperator {
                    let block = CodeBlock {
                        begin,
                        end: i,
                        content: String::from_iter(code_block.iter()),
                        lang: String::from_iter(code_lang.iter()).trim().to_string(),
                    };
                    all_blocks.push(block);
                    code_block.clear();
                    code_lang.clear();
                    code_began = None;
                    lang_done = false;
                }
                if i.checked_sub(begin).is_some_and(|i| i >= three_chars.len()) {
                    let c = three_chars[0];
                    if lang_done {
                        code_block.push(c);
                    } else {
                        code_lang.push(c);
                        lang_done = c == '\n';
                    }
                }
            }
            None => {
                if three_chars_are_seperator {
                    code_began = Some(i);
                }
            }
        }
    }
    all_blocks
}
