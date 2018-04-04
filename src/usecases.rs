use boundaries::Repository;
use entities::Url;

pub struct GetUrl;
pub struct SaveUrl;

impl GetUrl {
    pub fn action<'d, T: Repository<'d>>(repo: &mut T, short: &'d str) -> Result<Url, &'d str> {
        Ok(repo.get(short)?)
    }
}

impl SaveUrl {
    pub fn action<'d, T: Repository<'d>>(
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use boundaries::InMemory;

    #[test]
    fn test_save_url() {
        let mut db = InMemory::default();
        match SaveUrl::action(&mut db, "www.google.com", "xd") {
            Ok(new_url) => {
                assert_eq!(0, new_url.count);
                assert_eq!("xd", new_url.short)
            }
            Err(err) => panic!(err),
        }
    }

    #[test]
    fn test_try_save_url_twice() {
        let mut db = InMemory::default();
        let _hidden = SaveUrl::action(&mut db, "www.google.com", "xd");
        match SaveUrl::action(&mut db, "www.google.com", "xd") {
            Ok(_) => println!("expect no result"),
            Err(err) => assert_eq!(err, "Already exists"),
        }
    }

    #[test]
    fn test_get_url() {
        let mut db = InMemory::default();
        let saved = SaveUrl::action(&mut db, "www.google.com", "xd");
        match GetUrl::action(&mut db, "xd") {
            Ok(url) => {
                let v = saved.unwrap();
                assert_eq!(v.original, url.original);
                assert_eq!(v.short, url.short)
            }
            Err(err) => eprintln!("{}", String::from(err)),
        }
    }

    #[test]
    fn test_get_url_not_found() {
        let mut db = InMemory::default();
        match GetUrl::action(&mut db, "xd") {
            Ok(_) => eprintln!("expected no result!"),
            Err(err) => assert_eq!("Not Found", err),
        }
    }
}
