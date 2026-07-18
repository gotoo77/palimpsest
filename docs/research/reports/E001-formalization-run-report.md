# E001 - Rapport de run Codex pour la formalisation

## Statut de source

- Identifiant : `RP002`
- Date : 2026-07-18
- Statut : reponse finale conservee depuis le contexte de session
- Prompt source : `P002`
- Artefact principal produit : `docs/experiments/E001-minimal-claim-divergence.md`

## Reponse finale Codex

J’ai formalisé la tranche conceptuelle E001, sans créer de code ni de projet.

Créé :
`docs/experiments/E001-minimal-claim-divergence.md`

Mis à jour :
`docs/research/prompt-history.md` avec `P002`
`docs/research/artifact-provenance.md` avec la filiation `P002 -> E001`
`docs/research/timeline.md`
`docs/research/open-hypotheses.md`
`docs/research/concept-evolution.md`

Point de conception important : j’ai retenu un Run B où `claim-001` existe mais reste inaccessible à `Ada`, plutôt qu’un claim absent. C’est plus propre expérimentalement, car le monde et la fixture restent identiques ; la variable indépendante est uniquement l’accessibilité.

Vérification faite : aucun fichier de code, `pyproject.toml`, moteur, CLI ou test exécutable créé. Le dossier courant n’est pas un dépôt Git, donc je n’ai pas pu fournir de diff Git.
