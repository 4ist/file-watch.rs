use std::fs;
use std::process::Command;
use std::env;

fn main() {
    

    let dir = env::current_dir().unwrap();
    println!("file-watch.rs: pushing files in {}", dir.display());

    /*
    let file = fs::File::open("config.json")
        .expect("file should open read only");

    let app_config: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");

    let os = app_config.get("OS")
        .expect("file should have OS key");
    */

    //println!("{}", &os);}
    let git_add = vec!["/C", "git", "add", "."]; //TODO add this to a try/catch to handle being ran outside a git repo
    let git_commit = vec!["/C", "git", "commit", "-m", "\"automated commit\""];
    let git_push = vec!["/C", "git", "push"]; //TODO add this to a try/catch to handle repos without remote master

    let commands = vec![&git_add, &git_commit, &git_push];
    let iterable_commands = commands.iter();

    let enabled = true;
    if !enabled{
        println!("committing not enabled; exiting");
        return
    }
    for &command in iterable_commands{
        let output = {
            Command::new("cmd")
                    .args(&*command)
                    .status()
                    .expect("failed to execute process")
        };
    }

    
    
    //let hello = output.stdout;



}

