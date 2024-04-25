use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader, Lines};
use std::iter::{zip, Peekable};
use std::time::{Instant, Duration};

fn init_file() -> File {
    let file = File::create("src/dummy.json");
    match file {
        Ok(file) => file,
        Err(e) => panic!("Error: {}", e)
    }
}

fn read_table(path: &str) -> Peekable<Lines<BufReader<File>>> {
    let f: File = match File::open(path) {
        Ok(val) => val,
        Err(e) => panic!("No need to panic! {}", e)
    };
    let content: Peekable<Lines<BufReader<File>>> = BufReader::new(f).lines().peekable();
    return content;
}

fn get_headers(file: &mut Peekable<Lines<BufReader<File>>>) -> Vec<String> {
    // Peek at the first item which does not consume it
    let line: &String = match file.peek() {
        Some(line) => match line {
            Ok(line) => line,
            Err(e) => panic!("CORE OVERDRIVE: {}", e)
        },
        None => panic!("No headers available"),
    };

    // Clone the line string to create an owned version of the headers
    let headers: Vec<String> = line.split('\t')
                      .map(|x| x.to_string())
                      .collect();

    headers
}

fn get_records(file: Peekable<Lines<BufReader<File>>>, headers: Vec<String>, mut output: File) {
    match output.write(b"[") {
        Ok(size) => size,
        Err(e) => panic!("This was not supposed to happen: {}", e)
    };

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
            let entry: String = format!(r#""{}": {},"#, i.0, formatted_value);
            record.push_str(entry.as_str());
        }
        record.insert(0, '{');
        if record.ends_with(",") {
            record.pop();
            record.push('}');
        } else {
            record.push('}');
        }
        record.push(',');
        match output.write_all(record.as_bytes()) {
            Ok(size) => size,
            Err(e) => panic!("This was not supposed to happen: {}", e)
        };
        // dataset.push_str(record.as_str());
    }
}

fn main() {
    let start: Instant = Instant::now();
    let mut table = read_table("src/data.txt");
    let headers = get_headers(&mut table);
    let res = init_file();
    get_records(table, headers, res);
    let duration: Duration = start.elapsed();
    println!("{:#?}", duration)
}