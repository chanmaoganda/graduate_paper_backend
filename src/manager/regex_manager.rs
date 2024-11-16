use regex::Regex;

#[derive(Debug, Clone)]
pub struct RegexManager {
    email_regex: Regex,
    id_regex: Regex,

}

impl RegexManager {
    pub fn new() -> Self {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        let id_regex = Regex::new(r"^[0-9]{10}$").unwrap();
        Self {
            email_regex,
            id_regex,
        }
    }

    pub fn is_valid_email(&self, email: &str) -> bool {
        self.email_regex.is_match(email)
    }

    pub fn is_valid_id(&self, id: &str) -> bool {
        self.id_regex.is_match(id)
    }
}