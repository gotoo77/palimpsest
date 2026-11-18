# P2 - Scène causale GPE (prototype jouable)

Ce sous-projet est un **consommateur natif expérimental** du crate racine `palimpsest`. Il utilise directement les API publiques de GPE, sans changer le moteur et sans réimplémenter la décision de E001.

## Démarrage

Dans `~/dev/palimpsest`, avec Rust 1.97.1 :

```bash
cargo run --manifest-path p2/Cargo.toml --locked
```

Sous Linux, GPE peut nécessiter `libasound2-dev`, `libudev-dev` et `pkg-config`. Pour WSLg, une session graphique fonctionnelle est nécessaire.

## Langues

L'interface utilise **le français par défaut**, avec une version anglaise intégrale.
Appuyer sur **L** pour passer de FR à EN (ou inversement), y compris après avoir révélé la décision.
Cette préférence concerne uniquement l'affichage : le modèle E001, les données, les règles et les traces ne changent pas de langue. Les textes sont centralisés dans `p2/src/i18n.rs` (pas de framework supplémentaire).

## Comprendre l'expérience

Borin a pris une ressource qui était promise à Ada. Ciro affirme que Borin a rompu une promesse, mais cette **accusation n'est pas une preuve**. Plus tard, Borin demande de l'aide à Ada. Le joueur peut lire le compte rendu de l'événement, qu'Ada ne peut pas consulter, puis décider si le Claim de Ciro est accessible à Ada et prédire la décision d'Ada.

**M** ne réécrit pas le passé : il masque ou révèle seulement le Claim pour Ada. **E** n'a aucun effet sur Ada : c'est uniquement l'inspection du joueur.

## Commandes

- **E** : inspecter / masquer l'affichage de l'accusation de Ciro (inspection du joueur, indépendante de l'accès d'Ada).
- **M** : autoriser / masquer le Claim pour Ada, **avant** la décision.
- **C** : prédire la coopération.
- **F** : prédire le refus.
- **Entrée** : révéler la décision et sa raison, après une prédiction.
- **R** : réinitialiser complètement pour un nouvel essai.
- **L** : passer du français à l'anglais, sans réinitialiser la scène.
- **Échap** : quitter.

Le résultat provient de `palimpsest::decide` (P1), **pas de l'interface**. La scène est volontairement fixe et rudimentaire.

## Valider

```bash
cargo fmt --manifest-path p2/Cargo.toml -- --check
cargo check --manifest-path p2/Cargo.toml --locked
cargo test --manifest-path p2/Cargo.toml --locked
cargo clippy --manifest-path p2/Cargo.toml --locked --all-targets -- -D warnings
```

Les tests automatiques vérifient les transitions et l'absence de logique décisionnelle dupliquée. Ils ne valident **pas** le plaisir de jeu, ni le rendu sous WSLg. La validation visuelle et une séance d'essai humain restent nécessaires avant de décider si H003 est renforcée.

## Limites

Un Claim unique et préchargé, aucune génération de rumeurs, pas de persistance, pas d'E004/H006, pas de carte, pas de dialogue, pas de design sonore, pas de vraie interaction sociale, pas de WASM. Une fenêtre GPE native sert uniquement de support d'expérience H003.
