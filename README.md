# PALIMPSEST

Laboratoire experimental sur la memoire persistante, la consultation de traces et leur influence sur des decisions futures.

**Etat au 2026-11-18 :** P0 documentaire fusionnee ; P1, micro-simulation E001 implementee sur branche de PR et validee par la CI GitHub (12 tests, formatage, compilation, Clippy et CLI). Ce n'est pas un jeu GPE ni une simulation sociale complete.

## Experience E001

Deux mondes partent du meme etat, contenant un evenement, son `EventRecord` et un `Claim` de Ciro sur Borin. Ada decide si elle aide Borin. Une seule variable change : le Claim est visible ou masque pour Ada.

Cette experience teste uniquement la chaine causale sous une politique de consultation triviale. Elle **ne prouve ni emergence, ni interet ludique, ni necessite de l'economie de l'attention (H006)**.

## Executer

Avec Rust 1.97.1 :

```sh
cargo run --locked -- compare
cargo fmt --all -- --check
cargo check --locked
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
```

Le programme affiche la comparaison A/B et les traces structurees en format de debug Rust. Aucun JSON externe, aucune base de donnees, aucun LLM, aucune dependance externe : il s'agit d'une preuve de concept verifiable.

Les tests verifient l'influence causale, la consultation, la reproductibilite, quatre controles negatifs, la permutation des identifiants, la provenance structurelle et la difference entre alleguer et prouver.

## Prototype P2 (GPE natif)

Une scène interactive expérimentale est disponible dans [p2/](p2/README.md) : afficher l'histoire, inspecter un Claim, masquer ou autoriser son accès à Ada, prédire puis révéler sa décision réelle.

```sh
cargo run --manifest-path p2/Cargo.toml --locked
```

Ceci reste **une proposition de prototype H003**, pas une preuve de qualité ludique. Le crate E001 racine et ses tests sont indépendants du rendu GPE.

## Sources et limites

- [Protocole E001](docs/experiments/E001-minimal-claim-divergence.md)
- [Revue critique C002](docs/research/E001-critical-review.md)
- [Analyse H006/C003](docs/research/H006-impact-on-E001.md)
- [Revue P0](docs/research/reports/RP008-E001-P0-review.md)

Les artefacts sont precharges : ni generation dynamique, ni sauvegarde, ni retention entre sessions ne sont testees. La CI est le critere de validation de compilation pour cette PR.
