/// Counts the pages in an epub file (based on 2000 chars per page).
use epub::doc::EpubDoc;
use std::path::Path;

use clap::Parser;

/// epub page counter (epc): counts pages in epub file based on char count
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to epub file
    #[arg(value_name = "EPUB_FILE")]
    file: String,
    /// char count per page
    #[arg(short, long, default_value_t = 2000)]
    chars_per_page: usize,
}

fn count_epub_pages(epub_file: &Path, chars_per_page: usize) -> Option<usize> {
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
            Some(char_count / chars_per_page)
        }
    };
}

fn main() {
    let args = Args::parse();

    let epub_file = Path::new(&args.file);
    if !epub_file.exists() || epub_file.is_dir()
        || epub_file.extension().unwrap_or(String::new().as_ref()) != "epub" {
        println!("epc: file not found or not an epub");
        return;
    }

    let chars_per_page: usize = args.chars_per_page;
    if chars_per_page == 0 {
        println!("epc: chars per page must be higher than 0");
        return;
    }

    match count_epub_pages(epub_file, chars_per_page) {
        None => println!("epc: unable to read epub contents"),
        Some(page_count) => {
            println!(
                "{} {}",
                page_count,
                epub_file.file_name().unwrap().to_string_lossy().replace(".epub", "")
            );
        }
    };
}
