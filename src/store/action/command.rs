#[derive(Clone, Debug)]
pub enum Phase {
    Validate(String),
    Run(String),
    Success(String)
}
