use calamine::{open_workbook_auto, DataType, Error, Range, Reader};
use std::{collections::HashMap, fs};

fn get_keys(start: u32, range: &Range<DataType>) -> Vec<String> {
  let col = range.get_size().1 as u32;
  let mut keys: Vec<String> = vec![];
  for n in 0..col {
    keys.push(range.get_value((start, n)).unwrap().to_string());
  }
  keys
}

fn get_range(path: &str) -> Result<Range<DataType>, Error> {
  let mut workbook = open_workbook_auto(path)?;
  let sheet_name = workbook.sheet_names()[0].clone();
  let range = workbook
    .worksheet_range(&sheet_name)
    .ok_or(Error::Msg("Cannot read first sheet"))??;

  Ok(range)
}

fn parse_row(keys: &Vec<String>, range: &Range<DataType>) -> HashMap<String, String> {
  let mut line: HashMap<String, String> = HashMap::new();
  for n in 0..range.get_size().1 {
    line.insert(keys[n].to_string(), range.get((0, n)).unwrap().to_string());
  }
  line
}

pub fn parse_into_json(file: &str) {
  println!("Reading File: {}", file);
  // TODO: get key row from argument or set as constant
  let key_row: u32 = 0;
  let range = get_range(&file).ok().expect("fail to get range");
  let keys = get_keys(key_row, &range);

  let mut key_value_map: Vec<HashMap<String, String>> = Vec::new();

  let (row, col): (usize, usize) = range.get_size();
  let data_row = key_row + 1;
  for n in data_row..row as u32 {
    let rec = range.range((n, 0), (n, col as u32 - 1));
    key_value_map.push(parse_row(&keys, &rec));
  }

  // Convert hashmap to formatted json string
  let json_string = serde_json::to_vec_pretty(&key_value_map).unwrap();

  // TODO: set dynamic file names
  let file_name: &str = "excel_to_json_dictionary";
  fs::write(format!("./output/{}.json", file_name), json_string).expect("Unable to write file");
  println!("Done");
}
