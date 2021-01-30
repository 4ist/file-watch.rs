use std::fs;
use std::process::Command;
use std::env;
use std::io;
use config::Config;
use std::collections::HashMap;
use std::path::PathBuf;
use std::slice::Iter;

fn main() {
    println!("-----------------");
    println!("| file-watch.rs |");
    println!("-----------------");

    let current_dir = env::current_dir().unwrap();
    println!("Target directory: {}", current_dir.display());

    let mut exe_dir = env::current_exe().unwrap();
    exe_dir.pop();

    println!("Config path: {}", exe_dir.display());

    let mut config_path: String = String::from(exe_dir.to_str().expect("current exe path was null"));
    config_path.push_str("\\config");


    let mut config = Config::default();
    config.merge(config::File::with_name(&config_path)).unwrap();
    let lil_config = config.try_into::<HashMap<String, String>>().unwrap();
    println!("Config: {:?}", lil_config);
    
    let enabled = true;
    if !enabled{
        println!("committing not enabled; exiting");
        return
    }

    let os = lil_config.get("OS").expect("file should have OS key");
    let commands = get_commands(&os);
    let iterable_commands = commands.iter();

    for command in iterable_commands{
        let mut win_command = command.clone();
        win_command.insert(0, String::from("/C"));
        let _output = {
            Command::new("cmd") //Refactor this
                    .args(&*win_command)
                    .status()
                    .expect("failed to execute process")
        };
    }
    
}

fn get_commands(os: &str) -> Vec<Vec<String>>{   //TODO figure out how to return an iterable
    match &os[..] {
        "windows" => get_windows_commands(),
        //"linux" => println!("OS config = linux"),
        _ => panic!("unsupported OS config: {}", &os[..])
    }
}

macro_rules! string_vec {
    // match a list of expressions separated by comma:
    ($($str:expr),*) => ({
        // create a Vec with this list of expressions,
        // calling String::from on each:
        vec![$(String::from($str),)*] as Vec<String>
    });
}

fn get_windows_commands() -> Vec<Vec<String>> { 
    let git_add = string_vec!["git", "add", "."]; //TODO add this to a try/catch to handle being ran outside a git repo
    let git_commit = string_vec!["git", "commit", "-m", "automated commit"];
    let git_push = string_vec!["git", "push"]; //TODO add this to a try/catch to handle repos without remote master

    let commands = vec![git_add, git_commit, git_push];
    return commands;
}

