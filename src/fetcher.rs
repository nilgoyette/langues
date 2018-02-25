
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use app_dirs::{AppDataType, AppInfo, get_app_root};
use reqwest;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use serde_json;

use {Conjugation, Tense};

const APP_INFO: AppInfo = AppInfo{name: "Langues", author: "nilgoyette"};

pub fn get_french_verb(verb: &str) -> Tense {
    get_verb(verb, "francais", "Présent", 1)
}

pub fn get_spanish_verb(verb: &str) -> Tense {
    get_verb(verb, "espagnol", "Presente", 3)
}

// http://conjugueur.reverso.net/conjugaison-francais-verbe-dormir.html
// http://conjugueur.reverso.net/conjugaison-espagnol-verbe-dormir.html
fn get_verb(verb: &str, language: &str, tense: &str, pronoun_length: usize) -> Tense {
    if let Some(tense) = already_downloaded(verb, language) {
        return tense;
    }

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

            let mut tense = Tense::new();
            tense.set(Conjugation::Root, verb);

            for chunk in words.chunks(pronoun_length + 1) {
                let pronoun = chunk[0..pronoun_length].join("");
                let verb = chunk.last().unwrap().clone();
                tense.set(Conjugation::from_string(pronoun.trim_right()), &verb);
            }

            // Serialize it before returning
            let mut path = get_app_root(AppDataType::SharedData, &APP_INFO).unwrap();
            path.push(language);
            path.push(verb.to_string() + ".json");

            println!("Writing {} to {:?}.", verb, path);
            let mut file = File::create(path).unwrap();
            file.write_all(&serde_json::to_string(&tense).unwrap().into_bytes()).unwrap();

            return tense;
        }
    }

    panic!("Le temps de verbe {} n'a pas été trouvé dans la page {}.", tense, url);
}

fn already_downloaded(verb: &str, language: &str) -> Option<Tense> {
    let mut folder = get_app_root(AppDataType::SharedData, &APP_INFO).unwrap();
    folder.push(language);
    if !Path::new(&folder).exists() {
        fs::create_dir_all(&folder).unwrap();
    }

    folder.push(verb.to_string() + ".json");
    println!("{:?}", folder);
    if Path::new(&folder).exists() {
        let mut f = File::open(folder).expect("file not found");
        let mut serialized = String::new();
        f.read_to_string(&mut serialized)
            .expect("something went wrong reading the file");
        serde_json::from_str(&serialized).unwrap()
    } else {
        None
    }
}
