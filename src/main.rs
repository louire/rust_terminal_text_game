use csv::{ReaderBuilder, StringRecord};
use std::collections::{hash_map, HashMap};
use std::ops::Add;
use std::{fs, option};

const FILENAME: &str = "history.csv";
const INITIAL_TAG: &str = "INICIO";

#[derive(Clone, Debug)]
enum DataType {
    SITUATION,
    OPTION,
    ERROR,
}

#[derive(Clone, Debug)]
struct Data {
    data_type: DataType,
    tag: String,
    text: String,
    health: i32,
    options: Vec<Data>,
}

impl Data {
    fn new(record: StringRecord) -> Data {
        let data_type = record.get(0).unwrap().trim();
        let data_type = match data_type {
            "SITUACION" => DataType::SITUATION,
            "OPCION" => DataType::OPTION,
            _ => DataType::ERROR,
        };

        let tag: String = String::from(record.get(1).unwrap().trim());
        let text: String = String::from(record.get(2).unwrap().trim());

        let health = record.get(3).unwrap().trim();
        let health: i32 = health.parse().unwrap_or(0);

        let options: Vec<Data> = vec![];

        Data {
            data_type,
            tag,
            text,
            health,
            options,
        }
    }
}

fn main() {
    // Game Data
    let mut current_tag = INITIAL_TAG;
    let mut health = 100;
    let mut records: HashMap<String, Data> = HashMap::new();
    let mut last_record: String = "".to_string();

    // Read text
    let contents = fs::read_to_string(FILENAME).expect("Something went wrong reading the file");

    // Read csv
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(contents.as_bytes());

    // Get data from csv
    for result in rdr.records() {
        let record = Data::new(result.unwrap());

        match record.data_type {
            DataType::SITUATION => {
                last_record = record.tag.clone();
                records.insert(record.tag.clone(), record);
            }
            DataType::OPTION => {
                if let Some(data) = records.get_mut(&last_record) {
                    (*data).options.push(record);
                }
            }
            _ => {}
        }
    }

    // Game Loop
    loop {
        println!("Tienes {} de vida", health);

        if let Some(data) = records.get(current_tag) {
            println!("{}", data.text);

            for (i, option) in data.options.iter().enumerate() {
                println!("[{}] {}", i, option.text);
            }

            let mut selection = String::new();
            std::io::stdin().read_line(&mut selection).unwrap();
            let selection = selection.trim().parse().unwrap_or(99);

            if let Some(selection) = &data.options.get(selection) {
                current_tag = &selection.tag;
            } else {
                println!("Comando no valido");
            }

            health += data.health;

            println!("");
        } else {
            break;
        }

        // println!("Seleccion: {}", selection);
        if health <= 0 {
            println!("== Game Over ==");
            break;
        }
    }
}