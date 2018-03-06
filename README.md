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

## Ce qui fonctionne bien ou presque

- Les verbes sont téléchargés à partir de `conjugueur.reverso.net`. Ils sont
automatiquement sérialisés afin de ne pas abuser de ce serveur.
- De simple questions/réponses sur une liste de verbes de votre choix.

## À faire

- Accepter les réponses avec ou sans accent
- Gérer les verbes "doubles" ou "triples", ie. `être, ser, estar`
- Flag --vos pour activer ou non vosotros, car ce pronom est utilisé seulement
en Espagne. AFAIK, l'Amérique latine ne l'utilise pas.
- Flag --percent X% pour choisir le % de chance de demander le sens de la
question (espagnol -> français, ou inverse)
- Un concept d'apprentissage un peu plus évolué. Certains verbes sont "faciles"
et devrait donc apparaître moins souvent.
