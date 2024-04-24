use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn read_table() {
    let f = match File::open("src/data.txt") {
        Ok(val) => val,
        Err(e) => panic!("No need to panic! {}", e)
    };

    let content = BufReader::new(f).lines().enumerate();

    let mut headers: Vec<String> = Vec::new();  // Declare headers here
    let mut dataset: String = String::new();

    for (index, record) in content {
        if index == 0 {
            let head = match record {
                Ok(res) => res,
                Err(e) => panic!("Error occurred {}", e)
            };
            headers = head.split("\t").map(|x| x.to_string()).collect();
        } else {
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
            // record.push_str("},");
            dataset.push_str(record.as_str());
        }
    }
    dataset.insert(0, '[');
    dataset.pop();
    dataset.push(']');
    println!("{dataset}")
}

fn main() {
    read_table();
}