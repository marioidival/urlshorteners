extern crate urlshortener;

use urlshortener::{
    boundaries::InMemory,
    usecases::{GetUrl, SaveUrl}
};

fn main() {
    let mut in_memory = InMemory::default();

    println!("{:?}", SaveUrl::action(&mut in_memory, "www.google.com", "xiboga"));
    println!("{:?}", GetUrl::action(&mut in_memory, "xiboga"));
    println!("{:?}", GetUrl::action(&mut in_memory, "xiboga"));
    println!("{:?}", GetUrl::action(&mut in_memory, "xiboga"));
    println!("{:?}", GetUrl::action(&mut in_memory, "xiboga"));
    match SaveUrl::action(&mut in_memory, "www.google.com", "xiboga") {
        Ok(url) => println!("save new url: {:?}", url),
        Err(err) => println!("Error: {}", err),
    }
    println!("Hello, Clean Architecture!");
}
