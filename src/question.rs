
use std::io::{stdin, stdout, Write};

use rand;
use rand::Rng;

use {Conjugation, Tense};

#[derive(Debug)]
pub struct Questions {
    verbs: Vec<(Tense, Tense)>,
    questions: Vec<(usize, Conjugation)>
}

impl Questions {
    pub fn new(
        verbs: Vec<(Tense, Tense)>,
        nb: usize,
        vosotros: bool
    ) -> Questions {
        let mut rng = rand::weak_rng();
        let mut conjugations = Conjugation::all();

        // Remove 'vous/vosotros' if the user didn't ask for it
        if !vosotros {
            let vosotros_position = conjugations.iter().position(
                |&c| c == Conjugation::SecondPlural).unwrap();
            conjugations.swap_remove(vosotros_position);
        }
        
        let mut questions = Vec::with_capacity(verbs.len());
        for i in 0..verbs.len() {
            rng.shuffle(&mut conjugations);
            for conjugation in &conjugations[0..nb] {
                questions.push((i, *conjugation));
            }
        }

        Questions { verbs, questions }
    }

    pub fn practice(&mut self) {
        let mut rng = rand::weak_rng();
        let mut user_answer = String::new();
        while !self.questions.is_empty() {
            rng.shuffle(&mut self.questions);

            let (idx, conjugation) = self.questions.pop().unwrap();
            let &(ref french, ref spanish) = &self.verbs[idx];

            // Choose side
            let (from, to) = if rng.gen() {
                (french, spanish)
            } else {
                (spanish, french)
            };
            let question = from.get(conjugation);
            println!("{}", question);

            let answers = possibilities(to.get(conjugation));

            user_answer.clear();
            stdout().flush().unwrap();
            stdin().read_line(&mut user_answer).expect("Did not enter a correct string");
            let user_answer = user_answer.trim().to_string();

            if good_answer(user_answer, &answers) {
                println!("");
            } else {
                println!("Wrong! Could have been   {}\n", answers[0]);
            }
        }
    }
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

/// Spanish has 5 accentuated letters: á, é, í, ó, ú and ñ. They are all really hard to type on a
/// french keyboard (except 'é'), so we want to accept the user answer if he "forgot" the accents.
fn good_answer(user_answer: String, answers: &Vec<String>) -> bool {
    answers.into_iter().any(|answer| compare_one(&user_answer, answer))
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
}
