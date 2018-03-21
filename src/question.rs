
use std::io::{stdin, stdout, Write};

use rand::{self, Rng};

use {Conjugation, Tense};

type Verb = (Tense, Tense);

#[derive(Debug)]
pub struct Questions {
    nb_conjugations: usize,
    questions: Vec<(Verb, Vec<Conjugation>)>,
    vosotros: bool,
    nb_on_error: usize
}

impl Questions {
    pub fn new(
        verbs: Vec<(Tense, Tense)>,
        nb: usize,
        vosotros: bool,
        nb_on_error: usize
    ) -> Questions {
        let mut rng = rand::weak_rng();
        let mut conjugations = get_conjugations(vosotros);
        let mut questions = Vec::with_capacity(verbs.len());
        for verb in verbs {
            rng.shuffle(&mut conjugations);
            questions.push((verb, conjugations[0..nb].iter().cloned().collect()));
        }

        Questions {
            nb_conjugations: if vosotros { 7 } else { 6 },
            questions, vosotros, nb_on_error
        }
    }

    pub fn practice(&mut self) {
        let mut user_answer = String::new();
        while !self.questions.is_empty() {
            let (question, answers) = self.get_one();
            println!("{}", question);

            user_answer.clear();
            stdout().flush().unwrap();
            stdin().read_line(&mut user_answer).expect("Did not enter a correct string");

            if good_answer(user_answer.trim(), &answers) {
                println!("");
                // Remove the verb from the list if there's no more conjugation to practice
                if self.questions.last().unwrap().1.is_empty() {
                    self.questions.pop().unwrap();
                }
            } else {
                println!("Wrong! Could have been   {}\n", answers[0]);
                self.add_it_back_and_more();
            }
        }
    }

    fn get_one(&mut self) -> (String, Vec<String>) {
        let mut rng = rand::weak_rng();
        rng.shuffle(&mut self.questions);

        let &mut (ref verb, ref mut conjugations) = self.questions.last_mut().unwrap();
        let conjugation = conjugations.pop().unwrap();

        // Choose side
        let (question, answers) = {
            let (from, to) = if rng.gen() {
                (&verb.0, &verb.1)
            } else {
                (&verb.1, &verb.0)
            };
            (from.get(conjugation).to_string(), possibilities(to.get(conjugation)))
        };

        (question, answers)
    }

    fn add_it_back_and_more(&mut self) {
        // A new scope because `self.questions` is already borrowed
        let mut possible_conjugations = {
            let mut possible_conjugations = get_conjugations(self.vosotros);
            let current_conjugations = &self.questions.last().unwrap().1;
            if current_conjugations.len() < self.nb_conjugations {
                for cc in current_conjugations {
                    let i = possible_conjugations.iter().position(|&pc| pc == *cc).unwrap();
                    possible_conjugations.swap_remove(i);
                }
            }
            possible_conjugations
        };

        // Now pick a N random conjugations from the list
        let mut rng = rand::weak_rng();
        for _ in 0..self.nb_on_error {
            let new_conjugation = *rng.choose(&possible_conjugations).unwrap();
            let i = possible_conjugations.iter().position(|&pc| pc == new_conjugation).unwrap();
            possible_conjugations.swap_remove(i);

            self.questions.last_mut().unwrap().1.push(new_conjugation);
        }
    }
}

fn get_conjugations(vosotros: bool) -> Vec<Conjugation> {
    let mut conjugations = Conjugation::all();

    // Remove 'vous/vosotros' if the user didn't ask for it
    if !vosotros {
        let vosotros_position = conjugations.iter().position(
            |&c| c == Conjugation::SecondPlural).unwrap();
        conjugations.swap_remove(vosotros_position);
    }

    conjugations
}

fn possibilities(answer: &str) -> Vec<String> {
    let is_facultative = answer.contains("(");
    if !answer.contains("/") && !is_facultative {
        return vec![answer.to_string()];
    }

    let mut parts = answer.split(' ');
    let mut pronouns = parts.next().unwrap();
    let verb = parts.next().unwrap();

    let mut possibilities = vec![];
    if is_facultative {
        // Remove the parenthesis
        pronouns = &pronouns[1..pronouns.len() - 1];

        for verb in verb.split('/') {
            possibilities.push(verb.to_string());
        }
    }

    for pronoun in pronouns.split('/') {
        for verb in verb.split('/') {
            possibilities.push(pronoun.to_string() + " " + verb);
        }
    }

    possibilities
}

/// Spanish has 6 accentuated letters: á, é, í, ó, ú and ñ. They are all really hard to type on a
/// french keyboard (except 'é'), so we want to accept the user answer if he "forgot" the accents.
fn good_answer(user_answer: &str, answers: &Vec<String>) -> bool {
    answers.into_iter().any(|answer| compare_one(user_answer, answer))
}

fn compare_one(user_answer: &str, answer: &str) -> bool {
    // Never use `.len()` to get the number of characters
    if answer.chars().count() != user_answer.chars().count() {
        false
    } else {
        for (c1, c2) in answer.chars().zip(user_answer.chars()) {
            let matched = match c1 {
                'á' => c2 == 'a' || c2 == 'á',
                'é' => c2 == 'e' || c2 == 'é',
                'í' => c2 == 'i' || c2 == 'í',
                'î' => c2 == 'i' || c2 == 'î', // Only in french
                'ó' => c2 == 'o' || c2 == 'ó',
                'ú' => c2 == 'u' || c2 == 'ú',
                'ñ' => c2 == 'n' || c2 == 'ñ',
                _ => c1 == c2
            };
            if !matched { return false; }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possibilities() {
        assert_eq!(possibilities("tu manges"), vec!["tu manges"]);
        assert_eq!(possibilities("il/elle mange"), vec!["il mange", "elle mange"]);
        assert_eq!(possibilities("ils/elles mangent"), vec!["ils mangent", "elles mangent"]);

        assert_eq!(possibilities("je paye/paie"), vec!["je paye", "je paie"]);
        assert_eq!(
            possibilities("ils/elles payent/paient"),
            vec!["ils payent", "ils paient", "elles payent", "elles paient"]);

        assert_eq!(possibilities("(tú) comes"), vec!["comes", "tú comes"]);
        assert_eq!(
            possibilities("(él/ella/Ud.) come"),
            vec!["come", "él come", "ella come", "Ud. come"]);
        assert_eq!(
            possibilities("(ellos/ellas/Uds.) comen"),
            vec!["comen", "ellos comen", "ellas comen", "Uds. comen"]);
    }

    #[test]
    fn test_compare() {
        assert!(good_answer("comer", &vec!["comer".into()]));
        assert!(good_answer("comer", &vec!["fsdfr".into(), "comer".into()]));
        assert!(!good_answer("comér", &vec!["fsdfr".into(), "comer".into()]));
        assert!(good_answer("connaitre", &vec!["connaître".into()]));
        assert!(!good_answer("connaître", &vec!["connaitre".into()]));

        assert!(compare_one("sueño", "sueño"));
        assert!(compare_one("sueno", "sueño"));
    }
}
