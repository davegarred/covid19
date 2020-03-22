use std::path::Path;
use serde::{Serialize,Deserialize};
use postgres::{Connection, TlsMode};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug,Serialize,Deserialize)]
struct Entry {
    date: String,
    value: String
}
#[derive(Debug,Serialize,Deserialize)]
struct Record {
    state: String,
    country: String,
    entries: Vec<Entry>
}

static INSERT_RECORD: &str = "INSERT INTO records (state, country, record_type, date, value)
                               VALUES ($1, $2, $3, $4, $5)";
#[test]
fn test() {
    let conn = Connection::connect("postgresql://stc_user:stc_pass@ec2-34-222-177-55.us-west-2.compute.amazonaws.com:5432/stc_covid", TlsMode::None).unwrap();

    let path = Path::new("../COVID-19/csse_covid_19_data/csse_covid_19_time_series/time_series_19-covid-Confirmed.csv");
    // let file = File::open("../COVID-19/csse_covid_19_data/csse_covid_19_time_series/time_series_19-covid-Confirmed.csv").unwrap();
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let headers = &rdr.headers().unwrap().clone();
    let expected_cols = headers.len();
    println!("{:?}", headers);
    for result in rdr.records() {
        let mut location_record = Record{
            state: "".to_string(),
            country: "".to_string(),
            entries: Default::default(),
        };
        for (i, value) in result.unwrap().iter().enumerate() {
            match i {
                0 => location_record.state = value.to_string(),
                1 => location_record.country = value.to_string(),
                2 | 3 => {},
                _ => {
                    if value != "0" {
                        let date = headers.get(i).unwrap().to_string();
                        let num_cases = value.parse::<i32>().unwrap();
                        conn.execute(INSERT_RECORD, &[
                            &location_record.state.as_str(),
                            &location_record.country.as_str(),
                            &"CONFIRMED",
                            &date,
                            &num_cases
                        ]).unwrap();
                        location_record.entries.push(Entry{
                            date,
                            value: value.to_string()
                        });
                    }
                }
            }
        }
        if location_record.state == "King County, WA" || location_record.state == "Washington" {
            println!("{}", serde_json::to_string_pretty(&location_record).unwrap());
        }
    }
}