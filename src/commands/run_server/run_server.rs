use std::process::Command;

use crate::types::capacity::Capacity;

pub fn exec_server(capacity: Capacity) {
    println!("{}", capacity.as_value());
    let mut server = Command::new("java")
        .arg(capacity.as_value())
        .arg("-jar")
        .arg("server.jar")
        .arg("--nogui")
        .spawn()
        .expect("Failed to run the server");
        let ecode = server.wait().expect("failed to wait on child");
        if !ecode.success() {
            println!("Error Occurred while executing server");
        }
}
