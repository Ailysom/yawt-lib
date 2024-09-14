#[derive(Debug)]
pub struct Error<'a> {
    pub message: &'a str,
}