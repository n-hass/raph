use crossterm::style::Stylize;
use crossterm::terminal::ClearType;
use dialoguer::{theme::ColorfulTheme, Select};
use std::env;
use std::fs::{read_to_string, write};
use std::path::PathBuf;
use regex::Regex;

use clap::{crate_version, Arg, Command};
use std::process::{Command as SystemCommand, exit};

use std::io::stdout;
use crossterm::{cursor, terminal, QueueableCommand};

#[tokio::main]
async fn main() {
    let matches = Command::new("raph")
    .about("🦀 AWS Profile Handler and Executor")
    .version(crate_version!())
    .bin_name("raph")
    .arg(Arg::new("profile")
        .help("Specifies the AWS profile to use")
        .required(false)
        .index(1))
    .arg(Arg::new("command")
        .help("An 'aws' command to execute with the specified AWS profile, without affecting the current shell's environment")
        .required(false)
        .index(2)
        .num_args(1..))
    .get_matches();

    let home_dir = env::var("HOME").expect("HOME not set");
    match matches.get_one::<String>("profile") {
        Some(profile) => {
            let config_profiles = read_aws_profiles(&home_dir).expect("Error reading AWS config file");
            if verify_profile(&config_profiles, profile).is_err() {
                eprintln!("🦀 Error: Profile '{}' not found", profile);
                exit(5);
            }

            match matches.contains_id("command") { 
                true => {
                    let command_args = matches.get_many::<String>("command")
                        .expect("Error parsing aws command")
                        .collect::<Vec<&String>>();
                    execute_command_with_profile(profile, &command_args);
                    exit(0);  // Exit with code 0 after running a command because no shell profile switch required
                },
                false => {
                    match write_to_config(&home_dir, profile) {
                        Ok(_) => println!("🦀 Profile switched to {}", profile),
                        Err(err) => eprintln!("🦀 Error: {}", err),
                    }
                    exit(1);  // Exit with code 1 for profile switch
                },
            }
        },
        None => {
            // let default_profile_choice: String = std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".into());
            
            let default_profile_choice: String = match std::env::var("AWS_PROFILE") {
                Ok(profile) => {
                    if profile == "" {
                        "default".into()
                    } else {
                        profile
                    }
                },
                Err(_) => "default".into(),
            };

            let profile = match prompt_profile_choice(&home_dir, &default_profile_choice) {
                Ok(profile) => profile,
                Err(err) => {
                    eprintln!("🦀 Error: {}", err);
                    exit(5);
                },
            };

            match write_to_config(&home_dir, &profile) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("🦀 Error: {}", err);
                    exit(6);
                },
            }
            
            let _ = stdout()
                .queue(cursor::MoveUp(2)).unwrap()
                .queue(terminal::Clear(ClearType::CurrentLine)).unwrap();
            
            println!("🦀 AWS profile: {}", profile.green().bold());

            exit(1);  // Exit with code 1 for profile switch
        },
    }
    

}

fn read_aws_profiles(home_dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let config_path = PathBuf::from(home_dir).join(".aws/config");
    let data = read_to_string(config_path).expect("AWS config could not be read");

    let profile_regex = Regex::new(r"\[profile .*\]").expect("Invalid regex");
    let brackets_removal_regex = Regex::new(r"(\[profile )|(\])").expect("Invalid regex");

    let matches = profile_regex.find_iter(data.as_str()).collect::<Vec<_>>();
    
    if matches.is_empty() {
        println!("No profiles found.");
        println!("Refer to this guide for help on setting up a new AWS profile:");
        println!("https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-getting-started.html");
        return Err("No profiles found.".into());
    }

    let mut profiles: Vec<String> = matches.iter()
        .map(|mat| brackets_removal_regex.replace_all(mat.as_str(), "").into_owned())
        .collect();

    profiles.push("default".into());

    Ok(profiles)
}

fn prompt_profile_choice(home_dir: &str, default_profile_choice: &str) -> Result<String, Box<dyn std::error::Error>> {
    let profiles = read_aws_profiles(home_dir).expect("Error reading AWS profiles");
    
    let default_profile_index = profiles.iter().position(|profile| profile == &default_profile_choice).expect("Default profile not found in config");
    
    println!("🦀 AWS Profile Handler");
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a profile")
        .default(default_profile_index)
        .items(&profiles)
        .interact()?;

    Ok(profiles.get(selection).expect("selection is in profile list").to_string())
}

fn execute_command_with_profile(profile: &str, command_args: &Vec<&String>) {
    // let mut command = command_args.concat();

    let mut command_args = command_args.clone();

    let prefix = "aws".to_string();
    if command_args.first().expect("command args should be >=1") != &"aws" {
        command_args.insert(0, &prefix);
    }

    // temp override command_args to ["echo", "$AWS_PROFILE"]

    let echo = "echo".to_string();
    let aws_profile = "$AWS_PROFILE".to_string();
    command_args = vec![&echo, &aws_profile];

    let mut command = SystemCommand::new(&command_args[0]);
    command
        .args(&command_args[1..])
        .env("AWS_PROFILE", profile);
    match command.spawn() {
        Ok(mut child) => {
            match child.wait() {
                Ok(status) => println!("Command exited with status: {}", status),
                Err(e) => eprintln!("Failed to wait for command: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }
}

fn verify_profile(profile_config: &Vec<String>, profile_choice: &str) -> Result<(), String> {
    if profile_choice == "default" {
        return Ok(());
    }

    if profile_config.contains(&profile_choice.to_string()) {
        return Ok(());
    }

    Err(format!("Profile {} not found", profile_choice))
}

fn write_to_config(home_dir: &str, profile_choice: &str) -> Result<(), std::io::Error> {
    let profile_choice = if profile_choice == "default" {
        String::new()
    } else {
        profile_choice.to_string()
    };

    let path = PathBuf::from(home_dir).join(".raph");
    write(path, profile_choice)
}