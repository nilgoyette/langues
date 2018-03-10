# langues &emsp; [![dependency status](https://deps.rs/repo/github/nilgoyette/langues/status.svg)](https://deps.rs/repo/github/nilgoyette/langues)

`langues` est un projet personnel pour apprendre diverses langues. Il n'y
présentement que des outils pédagogiques pour l'espagnol mais je suis également
intéressé par la japonais alors ce projet va certainement évoluer.

Je suis conscient qu'il y a une panoplie d'outils mieux construits et plus
solides (anki, memrise, duolingo, etc.), je ne code pas ces outils pour être
meilleurs qu'eux; je le fais pour le plaisir et pour apprendre Rust. Ma
critique de ces outils est qu'ils ne demandent plus un mot lorsqu'ils
considèrent qu'il est appris. C'est parfaitement logique mais ce moment arrive
beaucoup trop rapidement selon eux.

Le projet est open source car il pourrait potentiellement aider d'autres
francophones. Je ne crois pas que ça va arriver, mais j'aime mieux partager.

Je viens tout juste de commencer ce projet alors presque tout est à faire! Cela
dit, je me concentre sur l'espagnol pour le moment, en particulier sur les
verbes.

## Installation

[Linux](https://github.com/sfackler/rust-openssl#linux) et
[OSX](https://github.com/sfackler/rust-openssl#osx) doivent installer OpenSSL.
Windows n'a rien de particulier à installer. Ensuite, un simple
`cargo run --release verbes.txt` et le tour est joué.

## Ce qui fonctionne bien ou presque

- Les verbes sont téléchargés à partir de `conjugueur.reverso.net`. Ils sont
automatiquement sérialisés afin de ne pas abuser de ce serveur.
- De simple questions/réponses sur une liste de verbes de votre choix.
- Flag --vos pour activer ou non vosotros, car ce pronom est utilisé seulement
en Espagne. AFAIK, l'Amérique latine ne l'utilise pas.

## À faire

- Étant facultatifs, les pronoms espagnols pourraient avoir un % de chance
d'être enlevés. Par contre, ce serait bien, un jour, de forcer un genre et de
s'attendre à une réponse genrée, par exemple "él camina" -> "il marche".
- Gérer les verbes "doubles" ou "triples", ie. `être, ser, estar`
- Flag --version X% pour choisir le % de chance de `thème` ou `version`.
`version` étant une question espagnol -> français, et `thème` l'inverse.
- Un concept d'apprentissage un peu plus évolué. Certains verbes sont "faciles"
et devrait donc apparaître moins souvent.
- Le pronom 'vous/vosotros' peut être enlevé des pratiques mais ce serait
peut-être une meilleure idée d'accepter sa forme singulière (él/ella/usted) et
pluriel (ellos/ellas/ustedes). Bref, gardez le 'vous' français et accepter
davantage de choix espagnols.
