extern crate reqwest;

use std::io::Read;

fn main() {
    run();
}
fn run() {
    let mut res = reqwest::get("http://localhost:8000").unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{}", res.headers());
    println!("Body:\n{}", body);

}

//extern crate url;
//
//use url::Url;
//
//fn run() -> Result<()> {
//    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";
//
//    let parsed = Url::parse(s)?;
//    println!("The path part of the URL is: {}", parsed.path());
//
//    Ok(())
//}
//
//
//fn main() {
//
//}
//extern crate url;
//
//use url::Url;
//
//fn run() -> Result<()> {
//    let full = "https://github.com/rust-lang/cargo?asdf";
//
//    let url = Url::parse(full)?;
//    let base = base_url(url)?;
//
//    assert_eq!(base.as_str(), "https://github.com/");
//    println!("The base of the URL is: {}", base);
//
//    Ok(())
//}
//
///// Returns the base of the given URL - the part not including any path segments
///// and query parameters.
//fn base_url(mut url: Url) -> Result<Url> {
//    // Clear path segments.
//    match url.path_segments_mut() {
//        Ok(mut path) => {
//            path.clear();
//        }
//        Err(_) => {
//            // Certain URLs cannot be turned into a base URL.
//            return Err(Error::from_kind(ErrorKind::CannotBeABase));
//        }
//    }
//
//    // Clear query parameters.
//    url.set_query(None);
//
//    Ok(url)
//}
//extern crate url;
//
//use url::Url;
//
//fn run() -> Result<()> {
//    let path = "/rust-lang/cargo";
//
//    let gh = build_github_url(path)?;
//
//    assert_eq!(gh.as_str(), "https://github.com/rust-lang/cargo");
//    println!("The joined URL is: {}", gh);
//
//    Ok(())
//}
//
//fn build_github_url(path: &str) -> Result<Url> {
//    // Hardcoded in our program. Caller's path will be joined to this.
//    const GITHUB: &'static str = "https://github.com";
//
//    let base = Url::parse(GITHUB).expect("hardcoded URL is known to be valid");
//    let joined = base.join(path)?;
//
//    Ok(joined)
//}
//extern crate reqwest;
//extern crate tempdir;
//
//use std::io::copy;
//use std::fs::File;
//use tempdir::TempDir;
//
//fn run() -> Result<()> {
//    // create a temp dir with prefix "example"
//    let tmp_dir = TempDir::new("example")?;
//    // make HTTP request for remote content
//    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
//    let mut response = reqwest::get(target)?;
//
//    let mut dest = {
//        // extract target filename from URL
//        let fname = response
//            .url()
//            .path_segments()
//            .and_then(|segments| segments.last())
//            .and_then(|name| if name.is_empty() { None } else { Some(name) })
//            .unwrap_or("tmp.bin");
//
//        println!("file to download: '{}'", fname);
//        let fname = tmp_dir.path().join(fname);
//        println!("will be located under: '{:?}'", fname);
//        // create file with given name inside the temp dir
//        File::create(fname)?
//    };
//    // data is copied into the target file
//    copy(&mut response, &mut dest)?;
//    // tmp_dir is implicitly removed
//    Ok(())
//}
