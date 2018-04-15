#[macro_use]
extern crate log;
extern crate env_logger;

fn execute_query(query: &str) {
    debug!("Executing query: {}", query);

    // then do the thing
}

fn run() -> Result<()> {
    env_logger::init()?;

    execute_query("DROP TABLE students");

    Ok(())
}

fn main() {
    run();
}
