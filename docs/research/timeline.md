# Chronologie de recherche

## 2026-07-18

### Initialisation critique de la memoire de conception

- Identifiant : `P001`
- Evenement : demande de revue approfondie de `codex-initialize-palimpsest.md` et creation d'une memoire de conception.
- Artefacts concernes : `D001`, `C001`, `docs/research/README.md`, `docs/research/prompt-history.md`, `docs/research/artifact-provenance.md`, `docs/research/concept-evolution.md`, `docs/research/open-hypotheses.md`, `docs/research/rejected-ideas.md`.
- Resultat : creation de la structure documentaire `docs/research/` et formalisation d'une revue critique.
- Consequence : le projet dispose d'un premier registre explicite de prompts, de filiations, d'hypotheses ouvertes et d'idees differees avant toute implementation.

### Formalisation de l'experience E001

- Identifiant : `P002`, `E001`
- Evenement : demande de formalisation du protocole `E001 - Divergence minimale par acces a un Claim`.
- Artefacts concernes : `C001`, `H001`, `docs/experiments/E001-minimal-claim-divergence.md`, `docs/research/prompt-history.md`, `docs/research/artifact-provenance.md`, `docs/research/open-hypotheses.md`, `docs/research/concept-evolution.md`.
- Resultat : creation d'un protocole experimental conceptuel comparant deux runs identiques sauf l'accessibilite d'un `Claim`.
- Consequence : l'etape suivante reste une revue du protocole avant toute implementation ; `H001` n'est pas encore validee.

### Revue critique de E001

- Identifiant : `P003`, `C002`
- Evenement : revue methodologique de `E001` avant autorisation d'implementation.
- Artefacts concernes : `E001`, `H001`, `docs/research/E001-critical-review.md`, `docs/research/prompt-history.md`, `docs/research/artifact-provenance.md`, `docs/research/open-hypotheses.md`.
- Resultat : `E001` est juge conceptuellement solide mais trop favorable a une reussite pre-cablee sans corrections.
- Consequence : implementation non autorisee a ce stade ; les corrections bloquantes portent sur la force du claim, la raison par defaut, la separation accessibilite-consultation-applicabilite, la trace atomique et les controles negatifs.

### Correction de l'autonomie documentaire

- Identifiant : `P004`, `D008`, `RP001`, `RP002`, `RP003`
- Evenement : detection d'une lacune entre provenance structuree et conservation autonome des sources primaires.
- Artefacts concernes : `docs/research/README.md`, `docs/research/prompt-history.md`, `docs/research/artifact-provenance.md`, `docs/research/prompts/`, `docs/research/reports/`.
- Resultat : creation d'une convention distinguant prompt original, rapport de run et capitalisation ; retrodocumentation de `P001` a `P004` et des rapports disponibles.
- Consequence : les prompts structurants futurs doivent etre archives comme fichiers autonomes lorsque leur texte est disponible ; les reconstructions doivent etre signalees explicitement.

### Hypothese sur l'economie de l'attention

- Identifiant : `P005`, `H006`, `E004`, `RP004`
- Evenement : formulation de la question "Pourquoi une entite consulte-t-elle un artefact ?" comme verrou conceptuel.
- Artefacts concernes : `docs/research/prompts/P005-attention-economy-hypothesis.md`, `docs/research/open-hypotheses.md`, `docs/research/concept-evolution.md`, `docs/research/artifact-provenance.md`, `docs/research/reports/RP004-attention-economy-hypothesis-report.md`.
- Resultat : ajout de `H006`, selon laquelle l'economie de l'attention pourrait etre un moteur central de divergence entre mondes ayant les memes artefacts.
- Consequence : aucune implementation autorisee ; `H006` reste une hypothese falsifiable et `E004` une experience candidate non formalisee.

### Double fil rouge recherche et jeu

- Identifiant : `P006`, `D009`, `RP005`
- Evenement : cadrage de PALIMPSEST comme laboratoire sur la memoire dont un jeu pourrait emerger.
- Artefacts concernes : `docs/research/prompts/P006-research-game-discipline.md`, `docs/research/README.md`, `docs/research/open-hypotheses.md`, `docs/research/concept-evolution.md`, `docs/research/artifact-provenance.md`, `docs/research/reports/RP005-research-game-discipline-report.md`.
- Resultat : ajout d'un garde-fou documentaire distinguant question scientifique et question ludique.
- Consequence : les futures experiences doivent eviter de confondre validite de recherche et interet joueur ; aucune architecture ni implementation n'est autorisee par ce cadrage.

### Analyse d'impact de H006 sur E001

- Identifiant : `P007`, `C003`, `RP006`
- Evenement : remise en question de la prochaine tranche "E001 corrigee" au profit d'une analyse d'impact de `H006`.
- Artefacts concernes : `docs/research/prompts/P007-H006-impact-on-E001.md`, `docs/research/H006-impact-on-E001.md`, `docs/research/open-hypotheses.md`, `docs/research/concept-evolution.md`, `docs/research/artifact-provenance.md`, `docs/research/reports/RP006-H006-impact-on-E001-report.md`.
- Resultat : `H006` n'annule pas `E001`, mais rend incomplete son hypothese implicite `claim accessible -> claim consulte`.
- Consequence : `E001` doit etre requalifie comme test sous politique de consultation triviale avant correction ; `E004` reste distinct pour tester directement l'economie de l'attention.

### Methode de recherche emergente

- Identifiant : `P008`, `D010`, `RP007`
- Evenement : reconnaissance de la metadiscipline de PALIMPSEST comme methode de recherche potentiellement generale.
- Artefacts concernes : `docs/research/prompts/P008-emergent-research-method.md`, `docs/research/research-method.md`, `docs/research/README.md`, `docs/research/concept-evolution.md`, `docs/research/artifact-provenance.md`, `docs/research/reports/RP007-emergent-research-method-report.md`.
- Resultat : creation d'un document de methode decrivant la sequence vision -> hypothese -> protocole -> revue -> analyse d'impact -> decision de statut -> implementation eventuelle.
- Consequence : la methode est surveillee comme patrimoine potentiel, mais aucun nouveau projet, framework, outil de graphe ou automatisation n'est lance.

## 2026-11-18

### Revision documentaire P0 de E001 via GitHub

- Identifiants : `P009` (mandat), `RP008` (revue).
- Origine : reprise acceptee par l'utilisateur, avec instruction explicite de travailler directement dans le depot GitHub.
- Artefacts concernes : E001, hypotheses ouvertes, chronologie, provenance, P009 et RP008.
- Resultat : corrections documentaires proposees dans une branche et une PR, sans implementation ni preuve experimentale.
- Consequence : gate humaine P0 encore ouverte ; H006/E004 restent distinctes ; aucune architecture GPE/Rust validee.

### E005 - Revue du gameplay apres le premier essai P2

- Source primaire : `P010` (retour utilisateur et acceptation de la suite), issue GitHub #3.
- Observation : la scene P2 fonctionne visuellement mais demeure ressentie comme un outil de demonstration ; **H003 non validee, REVISE**.
- Proposition : `E005`, deux demandes successives pour un seul creneau de travail d'Ada, meme Claim E001 et deux consequences concurrentes.
- Implementation candidate : prototype Rust headless sous PR dediee, sans changer le noyau E001 ni GPE.
- Limite : la preuve technique et le verdict de gameplay restent separes ; pas de validation humaine de E005.

## Entrees futures

Ajouter seulement les etapes qui changent substantiellement la vision, le modele de domaine, le game design, l'architecture, une experience, une decision ou un document structurant.
