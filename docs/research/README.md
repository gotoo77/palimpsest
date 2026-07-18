# Memoire de conception de PALIMPSEST

## Statut

Ce dossier initialise une memoire de conception pour PALIMPSEST. Il ne constitue ni une specification definitive, ni une architecture validee, ni une roadmap.

La date d'initialisation est le 2026-07-18.

## Objectif

Conserver une trace utile de la maniere dont le projet evolue :

- prompts structurants ;
- documents de vision ;
- critiques de conception ;
- hypotheses ;
- experiences candidates ;
- decisions lorsqu'elles seront explicitement validees ;
- idees rejetees, differees ou rouvertes.

L'objectif n'est pas d'archiver tout l'historique conversationnel. L'objectif est de preserver la provenance intellectuelle des transformations importantes du projet.

## Deux fils rouges

PALIMPSEST est actuellement traite comme un laboratoire sur la memoire dont un jeu pourrait emerger, pas comme un jeu deja specifie.

Deux questions doivent rester visibles dans les documents structurants :

- Question scientifique : comment une memoire persistante influence-t-elle un systeme ?
- Question ludique : pourquoi cette influence est-elle interessante pour le joueur ?

Ces deux questions ne convergent pas automatiquement. Une experience peut etre scientifiquement utile sans produire un gameplay interessant. Inversement, une piste ludique peut etre attirante sans tester proprement l'hypothese de memoire.

Les futures hypotheses, experiences et revues devraient donc signaler explicitement quel fil elles servent, et ce qu'elles ne permettent pas encore de conclure sur l'autre.

## Archive, provenance et decision

Une archive conserve un artefact ou une reference vers un artefact.

La provenance documente une relation de transformation : quel prompt, document, critique, hypothese ou resultat a conduit a quel autre artefact.

Une decision engage le projet. Aucune decision architecturale ou game design majeure ne doit etre inventee par ces fichiers. Les decisions formelles devront etre documentees separement, par exemple sous forme d'ADR, apres validation humaine.

## Sources primaires et capitalisation

Les prompts structurants et les rapports de run doivent etre conserves comme fichiers autonomes lorsque le texte est disponible.

Conventions :

- `docs/research/prompts/` conserve les prompts structurants fournis a Codex ;
- `docs/research/reports/` conserve les rapports de run ou reponses finales importantes de Codex ;
- les registres (`prompt-history.md`, `artifact-provenance.md`, `timeline.md`, `open-hypotheses.md`) capitalisent et relient les sources, mais ne les remplacent pas.

Chaque source primaire doit indiquer son statut :

- `texte source conserve` lorsque le texte exact est disponible ;
- `reponse finale conservee` lorsque le rapport de run est disponible ;
- `reconstruction documentaire` lorsque le texte exact n'est pas disponible localement.

Une reconstruction doit signaler explicitement sa limite. Elle ne doit pas etre presentee comme un prompt exact.

## Artefacts suivis

Les identifiants initiaux suivent une convention simple :

- `P001` : prompt ;
- `D001` : document ;
- `C001` : critique ou revue ;
- `H001` : hypothese ;
- `E001` : experience candidate ;
- `O001` : observation ;
- `A001` : architecture candidate ;
- `RP001` : rapport de run ou entree de catalogue de rapport ;
- `ADR001` : decision acceptee, uniquement si elle est validee ulterieurement.

Cette convention est provisoire. Elle sert a rendre les liens lisibles, pas a imposer une ontologie complete.

## Conventions de mise a jour

Mettre a jour uniquement les fichiers concernes lorsqu'un futur prompt modifie substantiellement la vision, le modele de domaine, le game design, l'architecture, une hypothese, une experience, une decision ou un document structurant.

Chaque entree significative devrait indiquer :

- la date ;
- les artefacts concernes ;
- le statut ;
- ce qui est observe, propose ou decide ;
- les incertitudes encore ouvertes.

Pour tout prompt structurant futur, creer d'abord ou mettre a jour :

- un fichier `docs/research/prompts/Pxxx-*.md` ;
- si le run produit une reponse finale ou un rapport important, un fichier `docs/research/reports/*.md` ;
- puis seulement les registres de capitalisation concernes.

Ne pas deduire artificiellement des filiations. Lorsqu'un lien est probable mais non documente, le signaler comme incertain.

## Limites

Ce dispositif est volontairement manuel et Markdown. Il ne fournit pas :

- de base de donnees ;
- de graphe de connaissances executable ;
- d'ontologie complete ;
- de capture automatique des conversations ;
- d'integration LLM ;
- d'interface dediee.

Toute automatisation future devra etre justifiee par une friction observee, pas par l'attrait technique du systeme de provenance lui-meme.
