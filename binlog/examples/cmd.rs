use std::process::Command;

fn main(){
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("ls")
                // .arg("-c")
                // .arg("echo hello")
                .output()
                .expect("failed to execute process")

        // Command::new("ls")
        //     .env("PATH", "/bin")
        //     // .spawn()
        //     .output()
        //     .expect("ls command failed to start")
    };

    let hello = output.stdout;

    println!("{:?}",  String::from_utf8_lossy(&hello));

}
