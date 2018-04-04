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
