
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Conjugation {
    Root, Je, Tu, Il, Nous, Vous, Ils
}

impl Conjugation {
    // TODO Prendre en charge les verbes impersonels. ex. pleuvoir, falloir, etc.
    pub fn from_string(det: &str) -> Conjugation {
        match det {
            "" => Conjugation::Root,
            "je" | "(yo)" => Conjugation::Je,
            "tu" | "(tú)" => Conjugation::Tu,
            "il/elle" | "(él/ella/Ud.)" => Conjugation::Il,
            "nous" | "(nosotros)" => Conjugation::Nous,
            "vous" | "(vosotros)" => Conjugation::Vous,
            "ils/elles" | "(ellos/ellas/Uds.)" => Conjugation::Ils,
            _ => panic!("{} n'est pas un pronom personel du français ou de l'espagnol.", det)
        }
    }
}
