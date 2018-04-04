use std::collections::HashMap;
use entities::Url;

pub trait Repository<'a> {
    fn get(&mut self, short: &'a str) -> Result<Url, &'a str>;
    fn save(&mut self, new_url: Url) -> Result<Url, &'a str>;
}

#[derive(Default)]
pub struct InMemory<'s> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inmemory_save() {
        let mut db = InMemory::default();
        let url = Url {
            original: "www.google.com",
            short: "xd",
            count: 0,
        };
        match db.save(url) {
            Ok(new_url) => assert_eq!(url, new_url),
            Err(_) => eprintln!("Unexpected result"),
        }
    }

    #[test]
    fn test_inmemory_save_twice() {
        let mut db = InMemory::default();
        let url = Url {
            original: "www.google.com",
            short: "xd",
            count: 0,
        };
        match db.save(url) {
            Ok(new_url) => assert_eq!(url, new_url),
            Err(_) => eprintln!("Unexpected result"),
        }
        match db.save(url) {
            Ok(_) => eprintln!("Unexpected result"),
            Err(err) => assert_eq!(err, "Already exists"),
        }
    }

    #[test]
    fn test_inmemory_get_notfound() {
        let mut db = InMemory::default();
        match db.get("xd") {
            Ok(_) => eprintln!("Unexpected result"),
            Err(err) => assert_eq!(err, "Not Found"),
        }
    }

    #[test]
    fn test_inmemory_get() {
        let mut db = InMemory::default();
        let url = Url {
            original: "www.google.com",
            short: "xd",
            count: 0,
        };
        let new_url = db.save(url).unwrap();
        match db.get("xd") {
            Ok(get_url) => assert_eq!(get_url.original, new_url.original),
            Err(__) => eprintln!("Unexpected result"),
        }
    }
}
