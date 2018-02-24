extern crate select;
extern crate reqwest;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};

struct PronomsPersonels {
    je: String,
    tu: String,
    il: String,
    nous: String,
    vous: String,
    ils: String
}

// http://conjugueur.reverso.net/conjugaison-francais-verbe-dormir.html
// http://conjugueur.reverso.net/conjugaison-espagnol-verbe-dormir.html
struct Verb {
    root: String,
    francais: PronomsPersonels,
    espagnol: PronomsPersonels
}

fn get_french_verb(verb: &str) {
    get_verb(verb, "francais", "Présent", 1)
}

fn get_spanish_verb(verb: &str) {
    get_verb(verb, "espagnol", "Presente", 3)
}

fn get_verb(verb: &str, language: &str, tense: &str, pronoun_length: usize) {
    let url = format!(
        "http://conjugueur.reverso.net/conjugaison-{}-verbe-{}.html", language, verb);
    let resp = reqwest::get(&url).unwrap();
    assert!(resp.status().is_success());

    let path_to_tense = Name("p");
    let path_to_text = Class("wrap-verbs-listing").descendant(Name("li")).descendant(Name("i"));

    let document = Document::from_read(resp).unwrap();
    for node in document.find(Class("blue-box-wrap")) {
        if node.find(path_to_tense).next().unwrap().text().trim_right() == tense {
            let words: Vec<String> = node.find(path_to_text).map(|n| n.text()).collect();

            // TODO Prendre en charge les verbes impersonels. ex. pleuvoir, falloir, etc.
            for chunk in words.chunks(pronoun_length + 1) {
                print!("{:020} ", chunk[0..pronoun_length].join(""));
                println!("{}", chunk.last().unwrap());
            }
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access() {
        for &(f, e) in &[("manger", "comer"), ("boire", "beber")] {
            get_french_verb(f);  println!("");
            get_spanish_verb(e); println!("");
        }
    }
}
