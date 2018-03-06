
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
    pub fn new(verbs: Vec<(Tense, Tense)>, nb: usize) -> Questions {
        let mut rng = rand::weak_rng();
        let mut conjugations = Conjugation::all();
        
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

            match answers.iter().find(|&s| s == &user_answer) {
                Some(_) => println!(""),
                None => println!("Wrong! Could have been   {}\n", answers[0])
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
    let pronouns = parts.next().unwrap();
    let verb = parts.next().unwrap();

    let mut possibilities = vec![];
    if is_facultative {
        possibilities.push(verb.to_string());
    }
    for pronoun in pronouns.split('/') {
        possibilities.push(pronoun.to_string() + " " + verb);
    }

    possibilities
}
