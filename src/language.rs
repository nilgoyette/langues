
use Conjugation;

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

    pub fn get_pronoun(&self, conjugation: Conjugation) -> &str {
        match *self {
            Language::French => {
                match conjugation {
                    Conjugation::Root => "::",
                    Conjugation::FirstSingular => "je",
                    Conjugation::SecondSingular => "tu",
                    Conjugation::ThirdSingular => "il",
                    Conjugation::FirstPlural => "nous",
                    Conjugation::SecondPlural => "vous",
                    Conjugation::ThirdPlural => "ils"
                }
            }
            Language::Spanish => {
                match conjugation {
                    Conjugation::Root => "::",
                    Conjugation::FirstSingular => "yo",
                    Conjugation::SecondSingular => "tú",
                    Conjugation::ThirdSingular => "él",
                    Conjugation::FirstPlural => "nosotros",
                    Conjugation::SecondPlural => "vosotros",
                    Conjugation::ThirdPlural => "ellos"
                }
            }
        }
    }
}
