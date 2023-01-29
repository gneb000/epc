/// Returns the page count of an epub file (based on 2000 chars per page).

use std::io::stdin;
use epub::doc::EpubDoc;

const CHARS_PER_PAGE: usize = 2000;

fn count_pages(epub_path: &str) -> usize {
    let mut doc = EpubDoc::new(epub_path).unwrap();
    let mut spine = doc.spine.clone();

    let mut char_count = 0;
    for res_id in spine.iter_mut() {
        let content = doc.get_resource_str(res_id).unwrap().0;
        char_count += content.chars().filter(|s| *s!='\n').count();
    }
    char_count / CHARS_PER_PAGE
}

fn main() {
    let mut input_stream = String::new();
    stdin().read_line(&mut input_stream).expect("Invalid input");
    let page_count = count_pages(&input_stream.trim());
    println!("{}", page_count);
}
