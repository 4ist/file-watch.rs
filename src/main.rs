use std::fs;
use std::process::Command;


fn main() {
    println!("Hello, world!");
    //let data = fs::read_to_string("config.json").expect("Unable to read file");
    // println!("{}", data);

    //let json: serde_json::Value = serde_json::from_str(&data);

    let file = fs::File::open("config.json")
        .expect("file should open read only");

    let app_config: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");

    let os = app_config.get("OS")
        .expect("file should have OS key");

    //println!("{}", &os);}
    
    let commands = vec!["git add .", "git commit -m \"automated commit \"", "git push"];
    let iterable_commands = commands.iter();

    for command in iterable_commands{
        let output = {
            Command::new("cmd")
                    .args(&["/C", &command])
                    .status()
                    .expect("failed to execute process")
        };
    }

    
    
    //let hello = output.stdout;



}

