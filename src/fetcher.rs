
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use app_dirs::{AppDataType, AppInfo, get_app_root};
use reqwest;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use serde_json;

use {Conjugation, Language, Tense};

const APP_INFO: AppInfo = AppInfo{name: "Langues", author: "nilgoyette"};

// http://conjugueur.reverso.net/conjugaison-francais-verbe-dormir.html
// http://conjugueur.reverso.net/conjugaison-espagnol-verbe-dormir.html
pub fn get_verb(verb: &str, language: Language) -> Tense {
    let language_name = language.name();
    if let Some(tense) = already_downloaded(verb, language_name) {
        return tense;
    }

    let url = format!(
        "http://conjugueur.reverso.net/conjugaison-{}-verbe-{}.html", language_name, verb);
    let resp = reqwest::get(&url).unwrap();
    assert!(resp.status().is_success());

    let path_to_tense = Name("p");
    let path_to_text = Class("wrap-verbs-listing").descendant(Name("li")).descendant(Name("i"));
    let tense_name = language.present_tense();
    let pronoun_length = language.pronoun_length();

    let document = Document::from_read(resp).unwrap();
    for node in document.find(Class("blue-box-wrap")) {
        if node.find(path_to_tense).next().unwrap().text().trim_right() == tense_name {
            let words: Vec<String> = node.find(path_to_text).map(|n| n.text()).collect();

            let mut tense = Tense::new(language);
            tense.set(Conjugation::Root, verb);

            for chunk in words.chunks(pronoun_length + 1) {
                let pronoun = chunk[0..pronoun_length].join("");
                let verb = chunk.last().unwrap().clone();
                tense.set(Conjugation::from_string(pronoun.trim_right()), &verb);
            }

            // Serialize it before returning. Lets disturb the verb source as less as possible
            let mut file = File::create(get_json_path(language_name, verb)).unwrap();
            file.write_all(&serde_json::to_string(&tense).unwrap().into_bytes()).unwrap();

            return tense;
        }
    }

    panic!("Le temps de verbe {} n'a pas été trouvé dans la page {}.", tense_name, url);
}

fn already_downloaded(verb: &str, language: &str) -> Option<Tense> {
    let json_path = get_json_path(language, verb);
    if Path::new(&json_path).exists() {
        let mut f = File::open(json_path).expect("file not found");
        let mut serialized = String::new();
        f.read_to_string(&mut serialized)
            .expect("something went wrong reading the file");
        let tense = serde_json::from_str(&serialized);
        if tense.is_err() {
            panic!("{} is corrupted", get_json_path(language, verb));
        }
        tense.unwrap()
    } else {
        None
    }
}

pub fn get_base_path() -> PathBuf {
    get_app_root(AppDataType::SharedData, &APP_INFO).unwrap()
}

fn get_json_path(language: &str, verb: &str) -> String {
    let mut path = get_base_path();
    path.push(language);
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }

    path.push(verb.to_string());
    path.set_extension("json");
    path.to_str().unwrap().to_string()
}
