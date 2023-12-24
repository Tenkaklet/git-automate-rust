use std::process::{Command, exit};
use names::Generator;

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
        .arg("master")
        .output()
        .expect("failed to execute git push command");

    println!("{}",push_command.status.success());
    

    if !push_command.status.success() {
        println!("git push command failed");
        exit(1);
    }

    println!("git push command success");

}

fn name_genrator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}


fn main() {
    update_commit_push();
}
