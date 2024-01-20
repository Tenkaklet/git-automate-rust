use std::process::{Command, exit};
use names::Generator;
use std::thread;
use std::time::Duration;
use std::{cmp::min, fmt::Write};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;
// use openai::{ApiResponseOrError};
use openai::files::File as OpenAiFile;




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

fn print_current_dir() -> std::path::PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    return current_dir;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let total_size = get_dir_size(&current_dir); // total size is the size of the repo
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
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
    Ok(())
}

