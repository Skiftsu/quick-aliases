use serde_json::Value;
use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    process::{Command, Stdio},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.get(1).is_none() {
        print_help();
        return;
    }

    let conf_file = init();
    let mut json = open_file(conf_file.clone()).expect("Failed to deserialize json");

    let first_arg = args[1].as_str();
    match first_arg {
        "rm" => {
            if args.get(2).is_none() {
                println!(
                    "Specify the name of the alias to be remove. Example: quick-aliases rm name"
                );
                return;
            }
            let name = args[2].clone();

            if remove_alias(&mut json, &name) {
                println!("The alias “{}” has been removed", name);
            } else {
                println!("This alias does not exist");
            }
            save_file(&json, conf_file).expect("Error when saving a value to the config");
        }
        "ls" => {
            println!("\x1b[1;4;3mAliases list:\x1b[0m");
            print_aliases(&json);
        }
        "rma" => {
            remove_all_aliases(&mut json);
            save_file(&json, conf_file).expect("Error when saving a value to the config");
            println!("All aliases have been removed");
        }
        "add" => {
            if args.get(2).is_none() {
                println!(
                    "You didn't specify the alias name and command. For example: quick-aliases add docker-compose dcompose"
                );
                return;
            } else if args.get(3).is_none() {
                println!(
                    "You didn't specify the alias command. For example: quick-aliases add docker-compose dcompose"
                );
                return;
            }
            let name = args[2].clone();
            let command = args
                .iter()
                .skip(3)
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join(" ");
            if add_alias(&mut json, &name, &command) {
                println!(
                    "The new alias has been added. Name: {} Command: {}",
                    name, command
                );
            } else {
                println!("This alias already exists");
            }
            save_file(&json, conf_file).expect("Error when saving a value to the config");
        }
        "help" => {
            print_help();
        }
        _ => {
            if !execute_alias(&first_arg.to_string(), &json) {
                println!("Unrecognized command. quick-aliases help - list of commands");
            }
        }
    }
}

fn init() -> PathBuf {
    // Config folder
    let conf_dir = env::home_dir()
        .unwrap()
        .join(".config")
        .join("quick-aliases");
    if !conf_dir.exists() {
        fs::create_dir_all(conf_dir.clone()).expect("Failed to create folder");
    }

    // Config file
    let conf_file = conf_dir.join("aliases.json");
    if !conf_file.exists() {
        fs::File::create(conf_file.clone()).expect("Failed to create conf file");
    }

    return conf_file;
}

fn open_file(conf_file: PathBuf) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(conf_file.clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let json_object: Value = match serde_json::from_str(&contents) {
        Ok(json_object) => json_object,
        Err(_) => {
            let empty_json = Value::Object(serde_json::Map::new());
            save_file(&empty_json, conf_file.clone()).unwrap();
            empty_json
        }
    };
    Ok(json_object)
}

fn save_file(json: &Value, conf_file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(conf_file)?;
    serde_json::to_writer_pretty(file, json)?;
    Ok(())
}

fn add_alias(json: &mut Value, name: &String, command: &String) -> bool {
    if let Some(obj) = json.as_object_mut() {
        obj.insert(name.clone(), Value::String(command.clone()))
            .is_none()
    } else {
        false
    }
}

fn remove_alias(json: &mut Value, name: &String) -> bool {
    if let Some(obj) = json.as_object_mut() {
        obj.remove(name.as_str()).is_some()
    } else {
        false
    }
}

fn remove_all_aliases(json: &mut Value) {
    if let Some(obj) = json.as_object_mut() {
        obj.clear();
    }
}

fn print_aliases(json: &Value) {
    if let Some(obj) = json.as_object() {
        for (key, value) in obj {
            let command = String::from(value.as_str().unwrap());
            println!("Name: {}, Command: {}", key, command);
        }
    }
}

fn execute_alias(name: &String, json: &Value) -> bool {
    if let Some(obj) = json.as_object() {
        if let Some(result) = obj.get(name) {
            let command = String::from(result.as_str().unwrap());
            println!("Execute command: {}", command);

            let mut child = Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .spawn()
                .unwrap();

            if let Some(ref mut stdout) = child.stdout {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    println!("{}", line.unwrap());
                }
            }

            child.wait().expect("failed to wait on child");

            return true;
        }
    }
    return false;
}

fn print_help() {
    println!(
        "
\x1b[37;44;1m Quick aliases v0.1 \x1B[0m

The aliases are stored in the json file by path:
$HOME/.config/quick-aliases/aliases.json

\x1b[1;3;4mUsage:\x1b[0m
    \x1b[1madd\x1b[0m [name] [comand] - add new alias
    \x1b[1mrm\x1b[0m [name] - remove alias
    \x1b[1mrma\x1b[0m - remove all aliases
    \x1b[1mls\x1b[0m - aliases list
    \x1b[1mhelp\x1b[0m - print this message
"
    );
}
