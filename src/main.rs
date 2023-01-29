/// Returns the page count of an epub file (based on 2000 chars per page).
use std::env;
use epub::doc::EpubDoc;

const CHARS_PER_PAGE: usize = 2000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut epub_path = &args[1];

    let mut doc = EpubDoc::new(epub_path.trim()).unwrap();
    let mut spine = doc.spine.clone();

    let mut char_count = 0;
    for res_id in spine.iter_mut() {
        char_count += doc.get_resource_str(res_id).unwrap().0.chars().filter(|s| *s!='\n').count();
    }
    let page_count = char_count / CHARS_PER_PAGE;
    println!("{page_count}");
}
