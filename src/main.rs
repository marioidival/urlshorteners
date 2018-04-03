use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct Url {
    original: &'static str,
    short: &'static str,
    count: i64,
}

trait Repository<'a> {
    fn get(&mut self, short: &'a str) -> Result<Url, &'a str>;
    fn save(&mut self, new_url: Url) -> Result<Url, &'a str>;
}

struct InMemory<'s> {
    items: HashMap<&'s str, Url>,
}

impl<'a, 's> Repository<'a> for InMemory<'s> {
    fn get(&mut self, short: &'a str) -> Result<Url, &'a str> {
        match self.items.get_mut(short) {
            Some(url) => {
                url.count = url.count + 1;
                Ok(*url)
            }
            None => Err("Not Found"),
        }
    }

    fn save(&mut self, new_url: Url) -> Result<Url, &'a str> {
        match self.get(new_url.short) {
            Ok(_url) => Err("Already exists"),
            Err(_e) => {
                self.items.insert(new_url.short, new_url);
                Ok(new_url)
            }
        }
    }
}

fn get_url<'d, T: Repository<'d>>(repo: &mut T, short: &'d str) -> Result<Url, &'d str> {
    match repo.get(short) {
        Ok(url) => Ok(url),
        Err(err) => Err(err),
    }
}

fn save_url<'d, T: Repository<'d>>(
    repo: &mut T,
    original: &'static str,
    short: &'static str,
) -> Result<Url, &'d str> {
    let new_url = Url {
        original: original,
        short: short,
        count: 0,
    };
    Ok(repo.save(new_url)?)
}

fn main() {
    let mut in_memory = InMemory {
        items: HashMap::new(),
    };

    println!("{:?}", save_url(&mut in_memory, "www.google.com", "xiboga"));
    println!("{:?}", get_url(&mut in_memory, "xiboga"));
    println!("{:?}", get_url(&mut in_memory, "xiboga"));
    println!("{:?}", get_url(&mut in_memory, "xiboga"));
    println!("{:?}", get_url(&mut in_memory, "xiboga"));
    match save_url(&mut in_memory, "www.google.com", "xiboga") {
        Ok(url) => println!("save new url: {:?}", url),
        Err(err) => println!("Error: {}", err),
    }
    println!("Hello, Clean Architecture!");
}
