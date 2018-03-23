#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid url for {}: {}", expected, url)]
    InvalidUrl { url: String, expected: String },
}
