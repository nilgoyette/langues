
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Conjugation {
    Root,
    FirstSingular,
    SecondSingular,
    ThirdSingular,
    FirstPlural,
    SecondPlural,
    ThirdPlural
}

impl Conjugation {
    pub fn all() -> Vec<Conjugation> {
        vec![
            Conjugation::Root,
            Conjugation::FirstSingular,
            Conjugation::SecondSingular,
            Conjugation::ThirdSingular,
            Conjugation::FirstPlural,
            Conjugation::SecondPlural,
            Conjugation::ThirdPlural]
    }

    // TODO Prendre en charge les verbes impersonels. ex. pleuvoir, falloir, etc.
    pub fn from_string(det: &str) -> Conjugation {
        match det {
            "" => Conjugation::Root,
            "j'" | "je" | "(yo)" => Conjugation::FirstSingular,
            "tu" | "(tú)" => Conjugation::SecondSingular,
            "il/elle" | "(él/ella/Ud.)" => Conjugation::ThirdSingular,
            "nous" | "(nosotros)" => Conjugation::FirstPlural,
            "vous" | "(vosotros)" => Conjugation::SecondPlural,
            "ils/elles" | "(ellos/ellas/Uds.)" => Conjugation::ThirdPlural,
            _ => panic!("{} n'est pas un pronom personel du français ou de l'espagnol.", det)
        }
    }
}
