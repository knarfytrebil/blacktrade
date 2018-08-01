#[derive(Clone, Debug, PartialEq)]
pub enum Phase {
    Validate(String),
    Run(String),
    Success(String)
}
