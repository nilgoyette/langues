extern crate app_dirs;
extern crate reqwest;
extern crate select;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod conjugation;
mod fetcher;
mod tense;

use conjugation::Conjugation;
use tense::Tense;

#[cfg(test)]
mod tests {
    use super::*;
    use fetcher::{get_french_verb, get_spanish_verb};

    #[test]
    fn test_access_and_conjugation() {
        let v = get_french_verb("manger");
        assert_eq!(v.root, "manger".to_string());
        assert_eq!(v.first_singular, "mange".to_string());
        assert_eq!(v.second_singular, "manges".to_string());
        assert_eq!(v.third_singular, "mange".to_string());
        assert_eq!(v.first_plural, "mangeons".to_string());
        assert_eq!(v.second_plural, "mangez".to_string());
        assert_eq!(v.third_plural, "mangent".to_string());

        let v = get_spanish_verb("comer");
        assert_eq!(v.root, "comer".to_string());
        assert_eq!(v.first_singular, "como".to_string());
        assert_eq!(v.second_singular, "comes".to_string());
        assert_eq!(v.third_singular, "come".to_string());
        assert_eq!(v.first_plural, "comemos".to_string());
        assert_eq!(v.second_plural, "coméis".to_string());
        assert_eq!(v.third_plural, "comen".to_string());
    }
}
