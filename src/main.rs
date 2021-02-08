use std::fs;
use std::process::Command;
use std::env;
use std::io;
use std::fmt;
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
    let app_config = config.try_into::<HashMap<String, String>>().unwrap();
    println!("Config: {:?}", app_config);
    
    let enabled = true;
    if !enabled{
        println!("committing not enabled; exiting");
        return
    }

    let os = app_config.get("OS").expect("file should have OS key");

    let mut commands = get_commands(&os);
    for command in &mut commands{
        //println!("{}", command);
        command.status().expect("failed to execute process");
    }
}

fn get_commands(os: &str) -> Vec<Command>{
    //TODO panic if bad config
    let app_commands = get_git_commands();
    match &os[..] {
        "windows" => get_windows_commands(app_commands),
        //"linux" => println!("OS config = linux"),
        _ => {
            //panic!("unsupported OS config: {}", &os[..])
            let empty_commands: Vec<Command> = vec![];
            empty_commands
        }
    }    
}

fn get_windows_commands(app_commands: Vec<Vec<String>>) -> Vec<Command>{
    let mut windows_commands: Vec<Command> = vec![];
    
    for app_command in app_commands.iter(){
        let mut win_command = Command::new("cmd");

        let mut win_command_args = app_command.clone();
        win_command_args.insert(0, String::from("/C"));
        win_command.args(&*win_command_args);

        windows_commands.push(win_command)
    }
    return windows_commands; 
}

macro_rules! string_vec {
    // match a list of expressions separated by comma:
    ($($str:expr),*) => ({
        // create a Vec with this list of expressions,
        // calling String::from on each:
        vec![$(String::from($str),)*] as Vec<String>
    });
}

fn get_git_commands() -> Vec<Vec<String>> { 
    let git_add = string_vec!["git", "add", "."]; //TODO add this to a try/catch to handle being ran outside a git repo
    let git_commit = string_vec!["git", "commit", "-m", "automated commit"];
    let git_push = string_vec!["git", "push"]; //TODO add this to a try/catch to handle repos without remote master

    let commands = vec![git_add, git_commit, git_push];
    return commands;
}

