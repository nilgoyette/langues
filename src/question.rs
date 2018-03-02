
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
            let pronoun = from.get_pronoun(conjugation);
            let question = from.get(conjugation);
            println!("{} {}", pronoun, question);

            user_answer.clear();
            stdout().flush().unwrap();
            stdin().read_line(&mut user_answer).expect("Did not enter a correct string");
            user_answer = user_answer.trim().to_string();

            let answer = to.get(conjugation);
            if user_answer != answer {
                println!("Wrong! Answer is {}\n", answer);
            } else {
                println!("");
            }
        }
    }
}
