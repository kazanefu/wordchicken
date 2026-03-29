use csv::ReaderBuilder;
/// this function make word list from CSV using csv::ReaderBuilder
/// argument1 is &str from CSV text
pub fn load_words(string: &str) -> Vec<String> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(string.as_bytes());

    rdr.records()
        .map(|r| r.unwrap()[0].to_string())
        .collect()
}