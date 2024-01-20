use names::Generator;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{exit, Command};
use std::thread;
use std::time::Duration;
use std::{cmp::min};
use walkdir::WalkDir;
// use openai::{ApiResponseOrError};
use openai::files::File as OpenAiFile;
use std::fs::{read_to_string, remove_file};
use std::io::{self};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

fn update_commit_push() {
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("failed to execute git add command");
    if !add_command.status.success() {
        println!("git add command failed");
        exit(1);
    }

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_genrator())
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
    }
}

fn name_genrator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
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

// #[tokio::main]
// async fn openai_file_reader(file: &str) -> ApiResponseOrError<()> {

//     set_key(env::var("OPENAI_KEY").unwrap());
//     let mut file = File::open(file)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     OpenAiFile::builder()
//         .file_name(contents) // local file path to upload.
//         .purpose("assistants")
//         .create()
//         .await?;
//     Ok(())
// }

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

        // Delete the created text document
        if let Err(e) = delete_diff_file() {
            eprintln!("Error deleting diff file: {}", e);
            exit(1);
        }
    } else {
        println!("No changes in 'git diff'.");
    }
    let mut downloaded = 0;

    // set_key(env::var("OPENAI_API_KEY").unwrap().to_string());

    let current_dir = print_current_dir(); //*** THIS CAN BE USED TO READ THE FILES FOR GOOGLE GEMINI */
                                           // let mut file = File::open(&current_dir)?;
                                           // let mut contents = String::new();
                                           // file.read_to_string(&mut contents)?;
                                           // let file_to_check = OpenAiFile::builder()
                                           //     .file_name(contents) // local file path to upload.
                                           //     .purpose("assistants")
                                           //     .create()
                                           //     .await?;

    // println!("what is this? {:?}", file_to_check.object);
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    update_commit_push();
    Ok(())
}
