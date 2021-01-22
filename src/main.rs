use anyhow::Result;

fn main() {
    match hello_world() {
        Ok(s) => println!("{}", s),
        Err(s) => {dbg!(s);}
    }
}

fn hello_world() -> Result<String> {
    Ok("Hello, world!".to_string())
}
