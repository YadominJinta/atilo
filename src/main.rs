mod utils;
use clap::{App, Arg, AppSettings};


fn main() {
    let version = String::from("3.0.0-alpha");

    let matches = App::new("Atilo")
        .version(version.as_str())
        .setting(AppSettings::ArgRequiredElseHelp)
        .author("Yadomin <i@yadom.in>")
        .about("Atilo is a rootless container")
        .subcommand(
            App::new("images")
                .about("List images")
        ).subcommand(
            App::new("info")
                .about("Display system-wide information")
        ).subcommand(
            App::new("pull")
                .about("Pull an image or a repository from a registry")
                .arg(
                    Arg::new("image")
                        .about("NAME[:TAG|@DIGEST]")
                        .index(1)
                        .required(true)
                )
        ).subcommand(
            App::new("run")
                .about("Run a command in a new container")
                .arg(
                    Arg::new("image")
                        .about("Image to run")
                        .index(2)
                        .required(true)
                )
                .arg(
                    Arg::new("command")
                        .about("Command to exec")
                        .index(1)
                        .required(true)
                )
        ).subcommand(
            App::new("version")
                .about("Show the atilo version information")
        )
        .get_matches();
        
        match matches.subcommand() {
            Some(("version", _)) => {
                println!("Atilo {}, working in process", version);
            }
            Some(("pull", pull_matches)) => {
                println!("Pulling {}", pull_matches.value_of("image").unwrap())
            }
            Some(("run", run_matches)) => {
                println!("Running {} in {}", run_matches.value_of("image").unwrap(), run_matches.value_of("command").unwrap());
            }
            Some(("info", _)) => {
                utils::get_sysinfo();
            }
            Some(("images", _)) => {
                println!("Images");
            }
            _ => {
                println!("Not a valid command");
            }
        }
    
}