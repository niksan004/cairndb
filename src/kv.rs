use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub enum Record {
    Set { key: String, val: String },
    Del { key: String },
}

impl Record {
    fn encode(&self) -> String {
        match self {
            Record::Set { key, val } => {
                format!("set {} {}", key, val)
            }
            Record::Del { key } => {
                format!("del {}", key)
            }
        }
    }

    fn decode(line: &str) -> Option<Record> {
        let split_line: Vec<&str> = line.splitn(3, ' ').collect();
        match split_line[..] {
            ["set", key, val] => Some(Record::Set {
                key: key.to_string(),
                val: val.to_string(),
            }),
            ["del", key] => Some(Record::Del {
                key: key.to_string(),
            }),
            _ => None,
        }
    }
}

pub struct KvStore {
    map: HashMap<String, String>,
    file: File,
}

// apply changes to the hashmap
fn apply(map: &mut HashMap<String, String>, record: Record) {
    match record {
        Record::Set { key, val } => {
            map.insert(key, val);
        }
        Record::Del { key } => {
            map.remove(&key);
        }
    }
}

impl KvStore {
    pub fn open_and_replay(path: impl AsRef<Path>) -> io::Result<KvStore> {
        // open file
        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(path)?;

        // replay log into hash map
        let mut map = HashMap::new();

        let reader = BufReader::new(&file);
        for line in reader.lines() {
            let line = line?;
            let Some(record) = Record::decode(&line) else {
                eprintln!("skipping corrupt line {}", line);
                continue;
            };
            apply(&mut map, record);
        }

        // create kv store
        Ok(KvStore { map, file })
    }

    // appends a record to the log
    fn append(&mut self, record: &Record) -> io::Result<()> {
        let line = format!("{}\n", record.encode());
        self.file.write_all(line.as_bytes())
    }

    pub fn set(&mut self, key: String, val: String) -> io::Result<()> {
        let record = Record::Set { key, val };
        self.append(&record)?;
        apply(&mut self.map, record);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn delete(&mut self, key: &str) -> io::Result<bool> {
        if !self.map.contains_key(key) {
            return Ok(false);
        }
        let record = Record::Del {
            key: key.to_string(),
        };
        self.append(&record)?;
        apply(&mut self.map, record);
        Ok(true)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.map.iter()
    }
}
