use std::env;
use std::process::Command;
use colored::Colorize;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        // TODO: make this help better
        println!("Use antisleep 1 for sleep on | 0 for sleep off. \n use help to see this msg.")
    } else {
        if args[1] == "1" {
            println!("{}","this will ask for a password.".yellow());
            Command::new("sh")
                .arg("-c")
                .arg("sudo pmset -b sleep 1 && sudo pmset -b disablesleep 0")
                .output()
                .expect("failed to execute process");
            println!("{}","The sleep functionality has been enabled!".green())
        } else if args[1] == "0" {
            println!("{}","A password will be requested.".yellow());
            Command::new("sh")
                .arg("-c")
                .arg("sudo pmset -b sleep 0 && sudo pmset -b disablesleep 1")
                .output()
                .expect("failed to execute process");
            println!("{}","sleep has been disabled".green())
        }

    }


}
