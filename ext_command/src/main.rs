#[macro_use]
extern crate error_chain;
extern crate regex;

use std::path::Path;
use std::process::Command;
use regex::Regex;
use std::env;

error_chain!{
    errors {
        ArgError
    }
    foreign_links {
        Io(std::io::Error);
        Regex(regex::Error);
        Utf8(std::string::FromUtf8Error);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        bail!(ErrorKind::ArgError);
    }
    println!("Execute:");

    let dest_path = Path::new(&args[1]);

    let output = Command::new("git").arg("status").arg("-s").output()?;

    if !output.status.success() {
        bail!("Command executed with failing error code");
    }

    let pattern = Regex::new(r"(M )(.*)")?;

    let _ = String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| {
            let path = cap[2].to_string();
            let src_path = Path::new(&path);
            let dest_path = dest_path.join(&path);
            println!("source:      {:?}",  src_path);
            println!("destination: {:?}", dest_path);
            Command::new("cp").arg(src_path.as_os_str()).arg(dest_path.as_os_str()).spawn()
        });


    Ok(())
}

fn main(){
    match run() {
        Err(e) => {
            match e.into() {
                ErrorKind::ArgError => println!("Usage: light-deploy PATH"),
                _ => println!("Something is wrong")
            }
        }
        _ => println!("Execution is completed")
    }

}


