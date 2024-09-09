#[derive(Clone)]
pub struct Password {
    pub pass: String,
}

impl Password {
    pub fn new(pass: String) -> Self {
        Self { pass }
    }
}
