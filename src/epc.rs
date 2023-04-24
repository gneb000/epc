/// Counts the pages in an epub file (based on 2000 chars per page).

use epub::doc::EpubDoc;
use std::env;
use std::path::Path;

const CHARS_PER_PAGE: usize = 2000;

fn count_epub_pages(epub_file: &Path) -> Option<usize> {
    return match EpubDoc::new(epub_file) {
        Err(_) => None,
        Ok(mut doc) => {
            let char_count = doc.spine.clone().iter().fold(0_usize, |acc, r| {
                acc + doc
                    .get_resource_str(r)
                    .unwrap_or((String::new(), String::new()))
                    .0
                    .chars()
                    .filter(|s| *s != '\n')
                    .count()
            });
            Some(char_count / CHARS_PER_PAGE)
        }
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = Path::new(args[1].trim());

    if !input_path.exists() || input_path.is_dir() {
        println!("epc: file not found");
        return;
    }

    if input_path.extension().unwrap_or(String::new().as_ref()) != "epub" {
        println!("epc: file not an epub");
        return;
    }

    match count_epub_pages(input_path) {
        None => println!("epc: unable to read epub contents"),
        Some(page_count) => {
            println!(
                "{} {}",
                page_count,
                input_path.file_name().unwrap().to_string_lossy().replace(".epub", "")
            );
        }
    };
}
