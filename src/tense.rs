
use {Conjugation, Language};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tense {
    pub language: Language,
    pub root: String,
    pub first_singular: String,
    pub second_singular: String,
    pub third_singular: String,
    pub first_plural: String,
    pub second_plural: String,
    pub third_plural: String
}

impl Tense {
    pub fn new(language: Language) -> Tense {
        Tense {
            language,
            root: String::new(),
            first_singular: String::new(),
            second_singular: String::new(),
            third_singular: String::new(),
            first_plural: String::new(),
            second_plural: String::new(),
            third_plural: String::new()
        }
    }

    pub fn get_pronoun(&self, conjugation: Conjugation) -> &str {
        self.language.get_pronoun(conjugation)
    }

    pub fn get(&self, conjugation: Conjugation) -> &str {
        match conjugation {
            Conjugation::Root => &self.root,
            Conjugation::FirstSingular => &self.first_singular,
            Conjugation::SecondSingular => &self.second_singular,
            Conjugation::ThirdSingular => &self.third_singular,
            Conjugation::FirstPlural => &self.first_plural,
            Conjugation::SecondPlural => &self.second_plural,
            Conjugation::ThirdPlural => &self.third_plural
        }
    }

    pub fn set(&mut self, conjugation: Conjugation, verb: &str) {
        match conjugation {
            Conjugation::Root => self.root = verb.to_string(),
            Conjugation::FirstSingular => self.first_singular = verb.to_string(),
            Conjugation::SecondSingular => self.second_singular = verb.to_string(),
            Conjugation::ThirdSingular => self.third_singular = verb.to_string(),
            Conjugation::FirstPlural => self.first_plural = verb.to_string(),
            Conjugation::SecondPlural => self.second_plural = verb.to_string(),
            Conjugation::ThirdPlural => self.third_plural = verb.to_string(),
        }
    }
}
