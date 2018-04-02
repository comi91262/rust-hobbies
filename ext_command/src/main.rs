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

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        bail!(ErrorKind::ArgError);
    }

    let src_path  = Path::new(&args[1]);
    let dest_path = Path::new(&args[2]);


    let output = Command::new("git").arg("status").arg("-s").output()?;
    if !output.status.success() {
        bail!("Command executed with failing error code");
    }

    let pattern = Regex::new(r"(M )(.*)")?;

    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| {
            cap[2].to_string()  //=> path
        })
        .for_each(|path| {
            let s_path = src_path.join(&path);
            let d_path = dest_path.join(&path);
            println!("{:?}", s_path.as_os_str());
           println!("{:?}", d_path.as_os_str());
            Command::new("cp").arg(s_path.as_os_str()).arg(d_path.as_os_str());
    //        let path = Path::new(&path);
    //        let a = path.join

        });


    Ok(())
}

fn main(){
    use ErrorKind::ArgError;

    match run() {
        Err(e) => {
            println!("{:?}", e);
            //println!("Usage: toy-deploy PATH1 PATH2 target");
        }
        _ => ()
    }

}


