
extern crate csv;
use std::collections::HashMap;


/// Read some CSV symbol file.
pub fn parse_symbol_file(path: &str) -> HashMap<u32, String> {
    let mut map = HashMap::new();
    let mut r = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path).unwrap();

    for record in r.records() {
        let data = record.unwrap();
        let fields: Vec<&str> = data.iter().collect();
        let addr = u32::from_str_radix(fields[0], 16).unwrap();
        map.insert(addr, fields[1].to_string());
    }
    map
}
