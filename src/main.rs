use std::process::Command;

fn main() {
    let code = "C:/Users/moklet gaming 1/AppData/Local/Programs/Microsoft VS Code/bin/code.cmd"; // change with your actual code comand directory

    let project = "C:/Project/next-portfolio"; // change with your actual project directory

    Command::new(code).arg(project).spawn().expect("Canot Open");
}
