use std::env;
use std::process::Command;
use colored::Colorize;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return;
    } else {
        if args[1] == "-h" || args[1] == "help"{
           help();
           return;
        } else if args[1] == "-v" || args[1] == "version" {
            version();
            return;
        } else if args[1] == "off" || args[1] == "0" {
            sleep_off();
            return;
        } else if args[1] == "on" || args[1] == "1" {
            sleep_on();
            return;
        } else {
            println!("{}","Arg not found...".red());
            help();
            std::process::exit(1);
        }



    }


}
fn help() {
    println!("{}","-h, help : shows this help screen \n-v, version : shows the version of antisleep \n1, on : turns your computer's sleep functionality on \n0, off : turns the sleep functionality off".bright_yellow())
}

fn sleep_on() {
    Command::new("sh")
        .arg("-c")
        .arg("sudo pmset -b sleep 1 && sudo pmset -b disablesleep 0")
        .output()
        .expect("failed to run sleep on commands..\n run sudo pmset -b sleep 1 and sudo pmset -b disablesleep 0 in terminal");
    println!("{}","The sleep functionality has been enabled.".green())
    }


fn sleep_off() {
    Command::new("sh")
        .arg("-c")
        .arg("sudo pmset -b sleep 0 && sudo pmset -b disablesleep 1")
        .output()
        .expect("failed to run sleep on commands..\n run sudo pmset -b sleep 0 and sudo pmset -b disablesleep 1 in terminal");
    println!("{}","The sleep functionality has been disabled.".green())
}

fn version() {
    println!("{}","0.1.1".green())
}

