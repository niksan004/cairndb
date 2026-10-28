use cairndb::kv::{self, KvStore};
use std::io::{self, Write};

fn main() {
    let mut store = match kv::KvStore::open_and_replay("store.db") {
        Ok(store) => store,
        Err(e) => {
            eprintln!("failed to open and replay db {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = run_repl(&mut store) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn run_repl(store: &mut KvStore) -> io::Result<()> {
    let mut input = String::new();
    loop {
        print!("cairndb> ");
        io::stdout().flush()?;

        input.clear();
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }

        let line = input.trim();
        if line.is_empty() {
            continue;
        }

        if line == ".exit" {
            break;
        }

        match execute(store, line) {
            Ok(output) => println!("{}", output),
            Err(e) => eprintln!("error: {}", e),
        }
    }

    Ok(())
}

fn execute(store: &mut KvStore, line: &str) -> io::Result<String> {
    let split_line: Vec<&str> = line.splitn(3, ' ').collect();
    match split_line[..] {
        ["set", key, val] => {
            store.set(key.to_string(), val.to_string())?;
            Ok("Ok".to_string())
        }
        ["get", key] => match store.get(key) {
            Some(value) => Ok(value.to_string()),
            None => Ok("null".to_string()),
        },
        ["del", key] => {
            if store.delete(key)? {
                Ok(format!("deleted {}", key))
            } else {
                Ok(format!("key not found {}", key))
            }
        }
        ["list"] => {
            println!(
                "{}",
                store
                    .iter()
                    .map(|(k, v)| format!("{k} = {v}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            Ok("Ok".to_string())
        }
        _ => Ok("unknown command".to_string()),
    }
}
