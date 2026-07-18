# Codex Prompt — Initialisation de PALIMPSEST

Tu es un Software Architect, Game Systems Designer et Simulation Engineer expérimenté.

Ta mission est d’initialiser un nouveau projet de jeu expérimental fondé sur les principes suivants :

- mémoire persistante ;
- règles locales simples ;
- interactions entre entités ;
- production d’artefacts ;
- réinjection des conséquences dans les états futurs ;
- émergence de structures non explicitement programmées.

Le projet porte provisoirement le nom :

**PALIMPSEST**

Ce nom est temporaire et ne doit pas être traité comme une identité définitive.

---

# 1. Intuition fondatrice

Le projet part de l’hypothèse suivante :

> L’émergence apparaît lorsqu’un système possède une mémoire locale, des règles d’interaction simples, et la possibilité de réinjecter le résultat de ses interactions dans son propre futur.

Dans le jeu de la vie de Conway :

- la mémoire est l’état précédent des cellules ;
- les règles sont la naissance, la survie et la mort ;
- chaque génération devient l’entrée de la génération suivante.

Dans PALIMPSEST :

- la mémoire est constituée d’artefacts persistants ;
- les règles déterminent comment les entités lisent, interprètent, produisent, transforment, relient, contestent ou oublient ces artefacts ;
- les artefacts produits modifient les interactions futures ;
- le monde évolue en fonction de sa propre histoire enregistrée.

Le cœur du projet n’est donc pas seulement une simulation d’agents.

Le cœur du projet est une simulation dans laquelle :

> les traces produites par le système deviennent progressivement une partie active des règles du système.

---

# 2. Vision du jeu

PALIMPSEST est un jeu de simulation systémique dans lequel le joueur observe et influence une petite société d’entités autonomes.

Ces entités :

- possèdent un état interne limité ;
- perçoivent un environnement local ;
- interagissent selon des règles simples ;
- produisent des artefacts persistants ;
- interprètent différemment les artefacts existants ;
- modifient leurs comportements en fonction de cette mémoire partagée ;
- transmettent, transforment ou détruisent une partie de cette mémoire.

Les artefacts peuvent représenter, selon l’évolution du projet :

- des récits ;
- des cartes ;
- des symboles ;
- des règles ;
- des promesses ;
- des interdits ;
- des souvenirs ;
- des classifications ;
- des désaccords ;
- des décisions ;
- des monuments ;
- des institutions rudimentaires.

Un artefact n’est pas seulement un objet collectionnable.

Il doit pouvoir agir comme :

- une trace du passé ;
- une information interprétable ;
- une contrainte future ;
- un support de coordination ;
- une source de conflit ;
- un objet de transformation ;
- un élément de mémoire collective.

---

# 3. Rôle du joueur

Le rôle exact du joueur n’est pas encore définitivement arrêté.

Pour le prototype initial, considérer le joueur comme un **gardien-intervenant** disposant d’un pouvoir limité.

Le joueur doit pouvoir :

- observer les entités ;
- inspecter les artefacts ;
- consulter l’historique des interactions ;
- introduire un artefact ;
- modifier ponctuellement un artefact ;
- relier deux artefacts ;
- retirer ou masquer un artefact ;
- faire évoluer certains paramètres des règles ;
- avancer la simulation pas à pas ;
- laisser la simulation évoluer automatiquement.

Le joueur ne doit pas contrôler directement chaque entité.

Il agit principalement sur :

- la mémoire disponible ;
- les conditions d’interaction ;
- les contraintes du système ;
- la visibilité ou l’accessibilité des artefacts.

Le joueur doit progressivement comprendre qu’intervenir dans la mémoire du monde revient à intervenir dans son futur.

---

# 4. Boucle fondamentale

La boucle minimale du système doit être :

```text
percevoir
    ↓
interpréter l’état local et les artefacts accessibles
    ↓
choisir une action
    ↓
interagir avec une entité, un lieu ou un artefact
    ↓
produire une conséquence
    ↓
modifier l’état du monde ou créer un artefact
    ↓
enregistrer cette conséquence
    ↓
réinjecter ce nouvel état dans le cycle suivant
```

La boucle de jeu du joueur doit être :

```text
observer
    ↓
formuler une hypothèse sur le fonctionnement du système
    ↓
intervenir de manière limitée
    ↓
laisser le système évoluer
    ↓
inspecter les conséquences directes et indirectes
    ↓
réviser son interprétation
```

Le jeu doit favoriser l’expérimentation et l’observation plutôt que l’optimisation immédiate.

---

# 5. Distinction essentielle entre état et artefact

Le projet doit distinguer clairement :

## État

Information courante utilisée directement par la simulation.

Exemples :

- position ;
- énergie ;
- besoin ;
- disposition émotionnelle simplifiée ;
- relation courante avec une autre entité ;
- accessibilité d’un lieu ;
- possession d’une ressource.

## Artefact

Objet persistant issu d’une interaction passée et pouvant être interprété ultérieurement.

Exemples :

- « l’entité A a aidé l’entité B » ;
- « cette zone est considérée comme dangereuse » ;
- « le groupe a adopté cette règle » ;
- « cette ressource appartient symboliquement à telle communauté » ;
- « cette version d’un événement contredit une version antérieure ».

Un artefact ne doit pas être une simple ligne de log.

Il doit être un objet de domaine :

- identifiable ;
- inspectable ;
- daté ;
- attribuable à une origine ;
- relié à d’autres artefacts ;
- transformable ;
- interprétable ;
- potentiellement contestable ;
- potentiellement oublié ou inaccessible.

---

# 6. Modèle conceptuel minimal

Le prototype doit partir d’un modèle volontairement réduit.

## World

Responsable de :

- l’état global ;
- la collection d’entités ;
- les lieux ou cellules ;
- les artefacts ;
- l’horloge de simulation ;
- l’application déterministe des transitions.

## Entity

Une entité possède au minimum :

- un identifiant stable ;
- une position ;
- quelques besoins ou motivations ;
- une mémoire locale limitée ;
- un ensemble réduit de traits ;
- une capacité de perception ;
- une capacité d’action ;
- une stratégie d’interprétation des artefacts ;
- des relations minimales avec d’autres entités.

## Artifact

Un artefact possède au minimum :

- un identifiant stable ;
- un type ;
- un contenu structuré ;
- un auteur ou une origine ;
- une date de création ;
- une provenance ;
- une visibilité ;
- une portée ;
- des liens vers d’autres artefacts ;
- un état de validité ou de contestation ;
- un historique de transformations.

## Interaction

Une interaction représente :

- les participants ;
- le contexte local ;
- les informations perçues ;
- l’action choisie ;
- les règles appliquées ;
- les conséquences ;
- les artefacts produits ou modifiés.

## Rule

Une règle doit être :

- explicite ;
- petite ;
- testable ;
- déterministe autant que possible ;
- indépendante de la présentation ;
- traçable lors de son application.

## SimulationStep

Un pas de simulation doit permettre de reconstruire :

- l’état avant ;
- les perceptions ;
- les décisions ;
- les règles déclenchées ;
- les mutations ;
- les artefacts produits ;
- l’état après.

---

# 7. Premier scénario expérimental

Créer un scénario extrêmement simple.

## Monde

- une petite grille ou carte abstraite ;
- deux ou trois zones ;
- quelques ressources ;
- entre quatre et huit entités.

## Besoins possibles

Limiter les besoins initiaux à deux ou trois dimensions :

- sécurité ;
- énergie ;
- appartenance ou confiance.

## Actions possibles

Limiter les actions initiales à :

- se déplacer ;
- prendre une ressource ;
- donner une ressource ;
- demander une ressource ;
- aider ;
- refuser ;
- créer un signe ou message ;
- consulter un artefact ;
- soutenir ou contester un artefact.

## Artefacts initiaux

Implémenter au maximum trois catégories :

### EventRecord

Trace structurée d’un événement significatif.

### Claim

Interprétation ou affirmation produite par une entité à propos du monde.

### Norm

Règle sociale locale pouvant influencer les décisions futures.

Exemple de chaîne émergente attendue :

```text
une entité prend plusieurs ressources
    ↓
un témoin produit un claim
    ↓
d’autres entités consultent ce claim
    ↓
la confiance envers l’entité diminue
    ↓
plusieurs refus apparaissent
    ↓
une norm est proposée
    ↓
la norm influence les comportements futurs
```

Cette chaîne ne doit pas être codée comme un script narratif complet.

Elle doit pouvoir apparaître comme conséquence de règles génériques.

---

# 8. Propriétés recherchées

Le prototype doit rendre observables les phénomènes suivants :

## Accumulation

Le passé laisse des traces.

## Déformation

Une trace peut être simplifiée, mal interprétée ou transformée.

## Contestation

Plusieurs artefacts peuvent proposer des versions incompatibles d’un même événement.

## Renforcement

Une information répétée ou fréquemment consultée peut devenir plus influente.

## Institutionnalisation

Une succession d’événements et d’interprétations peut produire une norme persistante.

## Oubli

Un artefact inaccessible, non consulté ou trop ancien peut perdre son influence.

## Dépendance au chemin

Deux simulations presque identiques peuvent évoluer différemment à cause d’un artefact produit plus tôt.

## Réflexivité

À terme, certaines règles sociales du système peuvent être représentées par des artefacts internes au monde plutôt que seulement par du code.

Cette dernière propriété est importante, mais elle ne doit pas être implémentée prématurément.

---

# 9. Contraintes de conception

## Simplicité

Préférer :

- peu de concepts ;
- peu de règles ;
- des objets explicites ;
- des transitions observables ;
- une simulation compréhensible.

Refuser :

- les architectures spéculatives ;
- les systèmes de traits excessifs ;
- les moteurs génériques prématurés ;
- les arbres de comportement complexes ;
- les réseaux neuronaux ;
- les modèles de langage dans le cœur du prototype ;
- la génération narrative opaque ;
- les dépendances inutiles.

## Déterminisme

À seed identique et entrées identiques, la simulation doit être reproductible.

Toute source d’aléa doit passer par une abstraction contrôlée.

## Traçabilité

Chaque mutation importante doit pouvoir expliquer :

- quelle règle l’a provoquée ;
- quelles données ont été lues ;
- quels artefacts ont été consultés ;
- quelles conséquences ont été produites.

## Séparation des responsabilités

Séparer strictement :

- simulation ;
- règles ;
- persistance ;
- visualisation ;
- commandes du joueur ;
- inspection et analyse.

## Données structurées

Les artefacts et événements doivent être persistables dans un format structuré, lisible et versionnable.

Utiliser JSON pour la première version.

---

# 10. Technologie initiale

Utiliser :

- Python récent ;
- `uv` pour l’environnement et les dépendances ;
- `pyproject.toml` ;
- layout `src/` ;
- `pytest` ;
- `ruff` ;
- `mypy` ;
- bibliothèque standard autant que possible.

Le premier prototype doit être une application locale.

Commencer par une interface CLI et une visualisation textuelle.

Ne pas commencer par :

- Unity ;
- Godot ;
- Unreal Engine ;
- une interface web complexe ;
- une architecture client-serveur ;
- du multijoueur ;
- une base de données externe.

Une interface graphique pourra être envisagée après validation du modèle de simulation.

---

# 11. Interface CLI minimale

Prévoir une CLI de ce type :

```bash
palimpsest new --scenario minimal --seed 42
palimpsest run --steps 100
palimpsest step
palimpsest inspect world
palimpsest inspect entity <entity-id>
palimpsest inspect artifact <artifact-id>
palimpsest history
palimpsest intervene add-artifact <file.json>
palimpsest intervene hide-artifact <artifact-id>
palimpsest export --output simulation.json
palimpsest replay simulation.json
```

Les commandes exactes peuvent être adaptées, mais les capacités suivantes sont obligatoires :

- créer une simulation ;
- avancer d’un pas ;
- avancer de plusieurs pas ;
- inspecter le monde ;
- inspecter une entité ;
- inspecter un artefact ;
- consulter l’historique ;
- exporter ;
- rejouer ;
- introduire une intervention limitée du joueur.

---

# 12. Architecture cible du premier incrément

Proposer une architecture minimale proche de :

```text
src/palimpsest/
├── __init__.py
├── cli.py
├── domain/
│   ├── world.py
│   ├── entity.py
│   ├── artifact.py
│   ├── interaction.py
│   ├── rule.py
│   └── events.py
├── simulation/
│   ├── engine.py
│   ├── perception.py
│   ├── decision.py
│   └── transitions.py
├── scenarios/
│   └── minimal.py
├── persistence/
│   ├── serialization.py
│   └── replay.py
└── inspection/
    ├── rendering.py
    └── analysis.py
```

Cette structure est indicative.

Ne pas créer de modules vides uniquement pour correspondre au schéma.

Chaque module doit avoir une responsabilité réelle.

---

# 13. Invariants initiaux

Formaliser et tester au minimum les invariants suivants :

1. Chaque entité possède un identifiant stable.
2. Chaque artefact possède un identifiant stable.
3. Un artefact possède une provenance explicite.
4. Une interaction terminée ne peut pas être modifiée silencieusement.
5. Toute transformation d’un artefact crée une trace.
6. Un pas de simulation possède un état avant et un état après identifiables.
7. À seed identique, une simulation rejouée produit le même résultat.
8. Une règle appliquée apparaît dans la trace.
9. Une entité ne peut consulter qu’un artefact accessible selon les règles de visibilité.
10. Une décision influencée par un artefact doit référencer cet artefact dans la trace.
11. Une intervention du joueur doit être enregistrée comme un événement distinct.
12. Le log technique ne doit pas être confondu avec la mémoire interne du monde.

---

# 14. Tests attendus

Écrire des tests couvrant notamment :

- création du monde minimal ;
- reproductibilité par seed ;
- déplacement d’une entité ;
- perception locale ;
- production d’un EventRecord ;
- production d’un Claim ;
- consultation d’un Claim ;
- influence d’un artefact sur une décision ;
- contestation entre deux Claims ;
- création d’une Norm à partir d’une condition générique ;
- visibilité limitée d’un artefact ;
- transformation traçable d’un artefact ;
- export et import ;
- replay déterministe ;
- intervention du joueur ;
- divergence entre deux simulations ayant reçu des artefacts différents.

Les tests doivent vérifier les comportements observables, pas les détails privés d’implémentation.

---

# 15. Première expérience jouable

Le premier incrément jouable doit permettre d’observer ce scénario :

1. Plusieurs entités vivent dans un espace réduit.
2. Elles cherchent des ressources.
3. Elles peuvent coopérer ou refuser de coopérer.
4. Certaines interactions produisent des artefacts.
5. Les entités peuvent consulter ces artefacts.
6. Leur comportement futur peut être modifié par ce qu’elles ont consulté.
7. Le joueur peut introduire ou masquer un artefact.
8. La simulation diverge ensuite de manière visible.
9. Le joueur peut inspecter la chaîne causale ayant conduit à cette divergence.

Le prototype est réussi si le joueur peut formuler une phrase du type :

> « Cette communauté agit maintenant ainsi parce qu’un événement passé a été enregistré, interprété, transmis, puis transformé en norme. »

---

# 16. Ce qui n’est pas demandé

Ne pas implémenter pour l’instant :

- de modèle de langage ;
- de dialogues génératifs ;
- de simulation psychologique réaliste ;
- de civilisation complète ;
- d’économie complexe ;
- de génétique ;
- de combat ;
- de crafting ;
- de politique détaillée ;
- de religion ;
- de factions complexes ;
- de multijoueur ;
- de sauvegarde cloud ;
- d’éditeur graphique ;
- de moteur de mods ;
- de moteur de règles générique ;
- d’optimisation massive ;
- de monde infini.

Ces éléments pourront devenir des hypothèses futures, mais ne doivent pas polluer le premier modèle.

---

# 17. Documentation attendue

Créer au minimum :

## README.md

Présenter :

- l’intuition ;
- la proposition de jeu ;
- les concepts centraux ;
- l’installation ;
- les commandes ;
- un exemple de simulation ;
- les limites du prototype.

## docs/CONCEPT.md

Formaliser :

- mémoire ;
- interaction ;
- artefact ;
- réinjection ;
- émergence ;
- dépendance au chemin ;
- rôle du joueur.

## docs/DOMAIN_MODEL.md

Décrire :

- les entités ;
- les artefacts ;
- les interactions ;
- les règles ;
- les événements ;
- les invariants.

## docs/EXPERIMENTS.md

Définir comment enregistrer les expériences :

- hypothèse ;
- configuration ;
- seed ;
- intervention ;
- résultat ;
- observation ;
- interprétation ;
- limites.

## docs/OPEN_QUESTIONS.md

Conserver les questions non résolues sans les transformer prématurément en décisions.

Exemples :

- Un artefact peut-il devenir une règle exécutable ?
- Une norme existe-t-elle indépendamment des entités qui y croient ?
- Comment distinguer mémoire, croyance et vérité ?
- L’oubli doit-il être passif ou actif ?
- Une entité peut-elle savoir qu’un artefact a été transformé ?
- Comment mesurer l’influence réelle d’un artefact ?
- À quel moment une accumulation de Claims devient-elle une institution ?
- Quelle part de la mémoire doit rester locale ?
- Le joueur est-il extérieur au monde ou représenté dans sa mémoire ?

## docs/DECISIONS/

Créer des ADR uniquement pour les décisions réellement prises.

Ne pas créer d’ADR pour les intuitions ou les pistes encore ouvertes.

---

# 18. Méthode de travail demandée

Procéder par incréments.

## Étape 1 — Analyse

Avant de coder :

- reformuler la vision ;
- identifier les ambiguïtés ;
- distinguer les décisions nécessaires des questions ouvertes ;
- proposer le plus petit modèle capable de tester l’hypothèse centrale.

## Étape 2 — Initialisation

Créer :

- la structure du projet ;
- le packaging ;
- les outils qualité ;
- la documentation initiale ;
- les types de domaine minimaux.

## Étape 3 — Vertical slice

Implémenter une seule chaîne complète :

```text
interaction
→ production d’un artefact
→ persistance
→ consultation ultérieure
→ influence sur une décision
→ trace explicable
```

## Étape 4 — CLI

Rendre cette chaîne observable et manipulable depuis le terminal.

## Étape 5 — Tests et exemple

Créer :

- des tests déterministes ;
- un scénario reproductible ;
- un exemple documenté montrant une divergence causée par la mémoire.

Ne pas étendre le système tant que cette chaîne verticale n’est pas fonctionnelle.

---

# 19. Discipline épistémique

Ne transforme pas les intuitions de ce document en vérités établies.

Distingue explicitement :

- vision ;
- hypothèse ;
- décision de prototype ;
- comportement observé ;
- interprétation ;
- question ouverte.

En particulier :

- ne prétends pas avoir créé de l’émergence simplement parce que le système est complexe ;
- ne qualifie pas une séquence de comportement d’émergente si elle est entièrement scriptée ;
- ne prétends pas simuler une culture, une conscience ou une société ;
- ne masque pas les règles déterminantes derrière des abstractions opaques ;
- ne généralise pas à partir d’un seul scénario.

Le prototype doit être un instrument d’investigation autant qu’un jeu.

---

# 20. Critères de réussite

Le premier jalon est réussi si :

- le projet est installable avec `uv` ;
- les contrôles qualité passent ;
- le scénario minimal est exécutable ;
- la simulation est reproductible ;
- les artefacts sont de vrais objets persistants ;
- les entités peuvent consulter des artefacts ;
- cette consultation peut modifier une décision future ;
- la chaîne causale est inspectable ;
- une intervention sur la mémoire provoque une divergence observable ;
- le code reste réduit et compréhensible ;
- les questions ouvertes restent documentées comme telles.

Le premier jalon n’a pas besoin d’être graphiquement impressionnant.

Il doit rendre l’idée fondatrice concrète, testable et observable.

---

# 21. Livrable demandé

Commence par produire :

1. une analyse synthétique de la vision ;
2. les principales ambiguïtés à préserver ;
3. le périmètre exact du premier incrément ;
4. l’architecture proposée ;
5. le modèle de domaine minimal ;
6. le plan de tests ;
7. la structure documentaire ;
8. ensuite seulement, initialise le dépôt et implémente la première tranche verticale.

À la fin, fournis :

- la liste des fichiers créés ;
- les principales décisions prises ;
- les hypothèses laissées ouvertes ;
- les commandes permettant d’installer et tester le projet ;
- un exemple complet de simulation ;
- les limites connues du prototype ;
- la prochaine expérience recommandée.

Ne développe pas au-delà du premier incrément sans justification issue d’un comportement effectivement observé.