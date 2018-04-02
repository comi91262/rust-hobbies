#[macro_use]
extern crate error_chain;
extern crate regex;

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

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

fn run() -> Result<()> {
    let output = Command::new("git").arg("status").arg("-s").output()?;

    let args: Vec<String> = env::args().collect();
    
    if args.len() != 4 {
       // bail!("");
       bail!(ErrorKind::ArgError);
    }


//    if !output.status.success() {
//        bail!("Command executed with failing error code");
//    }
//
//    let pattern = Regex::new(r"(?x)
//                               ([0-9a-fA-F]+) # commit hash
//                               (.*)           # The commit message")?;
//
//    String::from_utf8(output.stdout)?
//        .lines()
//        .filter_map(|line| pattern.captures(line))
//        .map(|cap| {
//                 Commit {
//                     hash: cap[1].to_string(),
//                     message: cap[2].trim().to_string(),
//                 }
//             })
//        .take(5)
//        .for_each(|x| println!("{:?}", x));
//
//
//    println!("{:?}", args);
    println!("{:?}", output);
    Ok(())
}

fn main(){
    match run() {
        Err(ArgError) => {
            println!("Usage: toy-deploy PATH1 PATH2 target");
        }
        _ => println!("success")
    }

}


