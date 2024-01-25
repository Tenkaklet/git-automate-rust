use dotenv::dotenv;
use openai::set_key;
use reqwest::{self, Error};
use reqwest::header::CONTENT_TYPE;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::cmp::min;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::{read_to_string, remove_file};
use std::io::{self};
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command};
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use serde_json::{self, json};

fn read_additions_removals(diff_content: &str) -> (Vec<&str>, Vec<&str>) {
    let mut additions = Vec::new();
    let mut removals = Vec::new();

    // Split the diff content into lines
    let lines: Vec<&str> = diff_content.lines().collect();

    for line in lines {
        // Check if the line is an addition or removal
        if line.starts_with('+') {
            additions.push(&line[1..]); // Remove the '+' prefix
        } else if line.starts_with('-') {
            removals.push(&line[1..]); // Remove the '-' prefix
        }
    }

    (additions, removals)
}

// Assuming the function signature is something like this:
async fn name_generator() -> Result<(), reqwest::Error> {
    // Read the content of the text document
    if let Ok(read_diff) = read_diff_from_file() {
        // Here you should implement the logic to get the commit message
        // For now, I'll just return a static string
        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", api_key).parse().unwrap(),
        );

        
        let body = json!({
            "model": "gpt-3.5-turbo-instruct",
            "messages": [
                    {
                        "role": "system",
                        "content": "You are a helpful GitHub assistant. Generate a commit message based on the following Git diff"
                    },
                    {
                        "role": "user",
                        "content": read_diff.to_string()
                    }
                ]
        });

        // Assuming you want to send the request here
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        // Handle the response here
        // For now, just ignore it
    }

    Ok(())
}



async fn get_commit_message(diff_content: &str) -> Result<String, reqwest::Error> {
    // Here you should implement the logic to get the commit message
    // For now, I'll just return a static string
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", api_key).parse().unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("failed to build client");

    let body = json!({
        "model": "gpt-3.5-turbo-instruct",
        "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful GitHub assistant. Generate a commit message based on the following Git diff"
                },
                {
                    "role": "user",
                    "content": diff_content.to_string()
                }
            ]
    });

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .json(&body)
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;

    let text = json["choices"][0]["message"]["content"].as_str().unwrap();

    Ok(text.to_string())
}



async fn update_commit_push() {
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("failed to execute git add command");

    if !add_command.status.success() {
        println!("git add command failed");
        exit(1);
    }

    let commit = get_commit_message("diff").await;
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit) // Convert commit_message to String
        .output()
        .expect("failed to execute git commit command");

    if !commit_command.status.success() {
        println!("git commit command failed");
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .expect("failed to execute git push command");

    if !push_command.status.success() {
        eprintln!("Command executed with errors:");
        eprintln!("{}", String::from_utf8_lossy(&push_command.stderr));
        exit(1);
    };
}

fn get_dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| fs::metadata(entry.path()).ok())
        .filter(|metadata| metadata.is_file())
        .map(|metadata| metadata.len())
        .sum()
}

fn write_diff_to_file(diff_text: &str) -> io::Result<()> {
    let mut file = File::create("git_diff.txt")?;
    file.write_all(diff_text.as_bytes())?;
    Ok(())
}

fn read_diff_from_file() -> io::Result<String> {
    let diff_text = read_to_string("git_diff.txt")?;
    Ok(diff_text)
}

fn delete_diff_file() -> io::Result<()> {
    remove_file("git_diff.txt")?;
    Ok(())
}

// ** this reads the git diff

fn print_current_dir() -> std::path::PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    return current_dir;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run 'git diff' command
    let git_diff_output = Command::new("git").arg("diff").output();

    // Check if the 'git diff' command was successful
    let output = match git_diff_output {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error running 'git diff': {}", e);
            exit(1);
        }
    };

    // Check if 'git diff' command produced any output
    if !output.stdout.is_empty() {
        // Write the output to a text document
        let diff_text = String::from_utf8_lossy(&output.stdout);
        if let Err(e) = write_diff_to_file(&diff_text) {
            eprintln!("Error writing diff to file: {}", e);
            exit(1);
        }

        // Read the content of the text document
        if let Ok(read_diff) = read_diff_from_file() {
            println!("Git Diff Content:\n{}", read_diff);
        } else {
            eprintln!("Error reading diff from file");
            exit(1);
        }
    } else {
        println!("No changes in 'git diff'.");
    }

    let mut downloaded = 0;
    let total_size = get_dir_size(&print_current_dir());
    let mut file = fs::File::open("git_diff.txt").expect("Unable to open file");
    let mut diff_content = String::new();
    dotenv().unwrap();
    set_key(env::var("OPENAI_KEY").unwrap());

    //let current_dir = print_current_dir(); //*** THIS CAN BE USED TO READ THE FILES FOR GOOGLE GEMINI */
    let _file = fs::File::open("git_diff.txt").expect("Unable to open file");
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    while downloaded < total_size {
        let new = min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(12));
    }

    pb.finish_with_message("downloaded");
    println!("Git Automation complete, Gracias!");
    update_commit_push();
    Ok::<(), std::io::Error>(()).expect("Unable to write data");

    let (additions, removals) = read_additions_removals(&diff_content);

    // Print the additions
    println!("Additions:");
    for addition in additions {
        println!("{}", addition);
    }

    // Print the removals
    println!("Removals:");
    for removal in removals {
        println!("{}", removal);
    }
    Ok(())
}
