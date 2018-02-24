
use reqwest;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

use {Conjugation, Tense};

pub fn get_french_verb(verb: &str) -> Tense {
    get_verb(verb, "francais", "Présent", 1)
}

pub fn get_spanish_verb(verb: &str) -> Tense {
    get_verb(verb, "espagnol", "Presente", 3)
}

// http://conjugueur.reverso.net/conjugaison-francais-verbe-dormir.html
// http://conjugueur.reverso.net/conjugaison-espagnol-verbe-dormir.html
fn get_verb(verb: &str, language: &str, tense: &str, pronoun_length: usize) -> Tense {
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

            let mut tense = Tense::with_capacity(1 + words.len() / pronoun_length + 1);
            tense.push((Conjugation::Root, verb.to_string()));

            for chunk in words.chunks(pronoun_length + 1) {
                let pronoun = chunk[0..pronoun_length].join("");
                let verb = chunk.last().unwrap().clone();
                tense.push((Conjugation::from_string(pronoun.trim_right()), verb));
            }
            return tense;
        }
    }

    panic!("Le temps de verbe {} n'a pas été trouvé dans la page {}.", tense, url);
}
