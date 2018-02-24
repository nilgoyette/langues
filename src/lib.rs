extern crate select;
extern crate reqwest;

mod conjugation;
mod fetcher;

use conjugation::Conjugation;

type Tense = Vec<(Conjugation, String)>;

#[cfg(test)]
mod tests {
    use super::*;
    use fetcher::{get_french_verb, get_spanish_verb};

    #[test]
    fn test_access_and_conjugation() {
        let v = get_french_verb("manger");
        assert_eq!(v[0], (Conjugation::Root, "manger".to_string()));
        assert_eq!(v[1], (Conjugation::Je, "mange".to_string()));
        assert_eq!(v[2], (Conjugation::Tu, "manges".to_string()));
        assert_eq!(v[3], (Conjugation::Il, "mange".to_string()));
        assert_eq!(v[4], (Conjugation::Nous, "mangeons".to_string()));
        assert_eq!(v[5], (Conjugation::Vous, "mangez".to_string()));
        assert_eq!(v[6], (Conjugation::Ils, "mangent".to_string()));

        let v = get_spanish_verb("comer");
        assert_eq!(v[0], (Conjugation::Root, "comer".to_string()));
        assert_eq!(v[1], (Conjugation::Je, "como".to_string()));
        assert_eq!(v[2], (Conjugation::Tu, "comes".to_string()));
        assert_eq!(v[3], (Conjugation::Il, "come".to_string()));
        assert_eq!(v[4], (Conjugation::Nous, "comemos".to_string()));
        assert_eq!(v[5], (Conjugation::Vous, "coméis".to_string()));
        assert_eq!(v[6], (Conjugation::Ils, "comen".to_string()));
    }
}
