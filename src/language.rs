#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Language {
    French,
    Spanish
}

impl Language {
    pub fn name(&self) -> &str {
        match *self {
            Language::French => "francais",
            Language::Spanish => "espagnol"
        }
    }

    pub fn present_tense(&self) -> &str {
        match *self {
            Language::French => "Présent",
            Language::Spanish => "Presente"
        }
    }

    pub fn pronoun_length(&self) -> usize {
        match *self {
            Language::French => 1,
            Language::Spanish => 3
        }
    }
}
