use std::env;

fn main() {
    if let Err(ref e) = run() {
        println!(r#"{{"yaml2json_error": "{}"}}"#, e);
    }
}

fn run() -> Result<(), String> {

    let file = match env::args().count() {
        x if x <= 1 => Err(String::from("File required"))?,
        2 => env::args().nth(1).unwrap(),
        _ => Err(String::from("Too manu arguments"))?,
    };

    let path = std::path::Path::new(&file);

    if !path.exists() {
        Err(format!("File '{}' not exists", file))?
    }

    if !path.is_file() {
        Err(format!("File '{}' not regular file", file))?
    }

    let data = std::fs::read_to_string(path).map_err(|e| format!("{}", e))?;;

    let data: serde_yaml::Value = serde_yaml::from_str(&data).map_err(|e| format!("{}", e))?;

    println!(
        "{}",
        serde_json::to_string(&data).map_err(|e| format!("{}", e))?
    );

    Ok(())
}
