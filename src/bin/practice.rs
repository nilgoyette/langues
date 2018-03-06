extern crate docopt;
extern crate espagnol;
extern crate rand;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use docopt::Docopt;

use espagnol::{Language, Questions, get_base_path, get_verb};

static USAGE: &'static str = "
Pratiquer les verbes français-espagnol à partir des verbes contenus dans un
fichier texte. Un verbe par ligne, ie. \"manger, comer\" Une ligne commençant
par '#' sera ignorée. La lecture du fichier s'arrête dès qu'une ligne commence
par '-'.

Usage:
  practice <verbs> [options]
  practice (-h | --help)
  practice (-v | --version)

Options:
  -h --help     Show this screen.
  -v --version  Show version.
";

fn main() {
    let version = String::from(env!("CARGO_PKG_VERSION"));
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.version(Some(version)).parse())
                      .unwrap_or_else(|e| e.exit());

    println!(
      "Saving/reading verb in {}\n",
      get_base_path().into_os_string().into_string().unwrap());

    let mut all_verbs = vec![];
    let reader = BufReader::new(File::open(args.get_str("<verbs>")).unwrap());
    for line in reader.lines() {
        let mut line = line.unwrap();
        if line.starts_with("#") { continue; }
        if line.starts_with("-") { break; }

        let mut parts = line.split_terminator(", ");
        let french = get_verb(parts.next().unwrap(), Language::French);
        let spanish = get_verb(parts.next().unwrap(), Language::Spanish);
        all_verbs.push((french, spanish));
    }

    let mut questions = Questions::new(all_verbs, 2);
    questions.practice();
}
