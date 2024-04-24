use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::zip;
use std::time::Instant;

fn read_table(path: &str) -> Lines<BufReader<File>> {
    let f = match File::open(path) {
        Ok(val) => val,
        Err(e) => panic!("No need to panic! {}", e)
    };

    let content: Lines<BufReader<File>> = BufReader::new(f).lines();
    return content;
}

fn get_headers(mut file: Lines<BufReader<File>>) -> Vec<String> {
    let head = match file.next() {
        Some(a) => a,
        None => panic!("Nothing available")
    };
    let headers = head.unwrap()
                                .split("\t")
                                .map(|x| x.to_string())
                                .collect();

    return headers;
}

fn get_records(file: Lines<BufReader<File>>, headers: Vec<String>) {
    let mut dataset: String = String::new();
    for record in file.skip(1) {
        let datapoint = match record {
            Ok(dp) => dp,
            Err(e) => panic!("Calm down please {}", e)
        };
        let datapoint: Vec<&str> = datapoint.split("\t").collect();
        let zipped = zip(&headers, datapoint);
        let mut record: String =  String::new();
        for i in zipped {
            let formatted_value = match i.1.parse::<f64>() {
                Ok(num) => num.to_string(), // If it's a number, use it as-is
                Err(_) => format!(r#""{}""#, i.1), // If not, wrap it in quotes
            };
            let entry = format!(r#""{}": {},"#, i.0, formatted_value);
            record.push_str(entry.as_str());
        }
        record.insert(0, '{');
        if !record.ends_with(",") {
            record.push('}');
        } else {
            record.pop(); // Remove the last comma
            record.push('}');
        }
        record.push(',');
        dataset.push_str(record.as_str());
    }
    dataset.insert(0, '[');
    dataset.pop();
    dataset.push(']');
    println!("{dataset}");
}

fn main() {
    let start = Instant::now();
    let headers = get_headers(read_table("src/data.txt"));
    get_records(read_table("src/data.txt"), headers);
    let duration = start.elapsed();
    println!("{:#?}", duration)
}