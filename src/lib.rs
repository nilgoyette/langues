extern crate app_dirs;
extern crate rand;
extern crate reqwest;
extern crate select;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod conjugation;
mod fetcher;
mod language;
mod question;
mod tense;

use conjugation::Conjugation;
pub use fetcher::{get_base_path, get_verb};
pub use language::Language;
pub use question::Questions;
use tense::Tense;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_and_conjugation() {
        let v = get_verb("manger", Language::French);
        assert_eq!(v.root, "manger".to_string());
        assert_eq!(v.first_singular, "je mange".to_string());
        assert_eq!(v.second_singular, "tu manges".to_string());
        assert_eq!(v.third_singular, "il/elle mange".to_string());
        assert_eq!(v.first_plural, "nous mangeons".to_string());
        assert_eq!(v.second_plural, "vous mangez".to_string());
        assert_eq!(v.third_plural, "ils/elles mangent".to_string());

        let v = get_verb("comer", Language::Spanish);
        assert_eq!(v.root, "comer".to_string());
        assert_eq!(v.first_singular, "(yo) como".to_string());
        assert_eq!(v.second_singular, "(tú) comes".to_string());
        assert_eq!(v.third_singular, "(él/ella/Ud.) come".to_string());
        assert_eq!(v.first_plural, "(nosotros) comemos".to_string());
        assert_eq!(v.second_plural, "(vosotros) coméis".to_string());
        assert_eq!(v.third_plural, "(ellos/ellas/Uds.) comen".to_string());
    }
}
