use std::fs;

const CHARS_PER_PAGE: usize = 1900;

fn main() {
    let epub_path = "./test_epub/test/OEBPS/Text";

    let files= get_file_list(epub_path);

    let page_count = count_pages(files);
    println!("{}", page_count);
}

/// Return file list in provided directory path
fn get_file_list(dir_path: &str) -> Vec<String> {
    fs::read_dir(dir_path).unwrap()
        .map(|f| f.unwrap().path().display().to_string())
        .collect()
}

/// Iterate through provided file list and return page count
fn count_pages(file_list: Vec<String>) -> usize {
    let mut char_count = 0;
    for file in file_list.iter() {
        char_count += fs::read_to_string(file)
            .expect(&format!("Unable to read the file: {}", file))
            .chars()
            .filter(|s| *s!='\n') // Exclude line breaks from char count
            .count();
    }
    char_count / CHARS_PER_PAGE
}
