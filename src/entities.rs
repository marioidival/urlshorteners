#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Url {
    pub original: &'static str,
    pub short: &'static str,
    pub count: i64,
}
