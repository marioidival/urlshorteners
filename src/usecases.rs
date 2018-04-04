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
