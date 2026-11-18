# P009 - Mandat de reprise documentaire E001

- Date : 2026-11-18
- Statut : mandat utilisateur et cadrage de portee ; aucune implementation.
- Sources : E001, C002, C003, H001, H006.

## Messages utilisateur (extraits exacts)

> suivi ta proposition ca me convient!

> attend je veux que tu travaille directement sur le depot gitub moi hein, on est d accord?

## Interpretation de portee

L'utilisateur a accepte la proposition de reprendre PALIMPSEST par une revision P0, et precise qu'il souhaite que les changements soient effectues **directement sur GitHub**. Cette interpretation est un cadrage de travail, pas une citation litterale.

La livraison autorisee ici est une branche de correction documentaire et une PR pour revue, sans fusion de `main` et sans simulation. Les critiques `C002` et `C003` demeurent des sources historiques, sans reecriture.

## Actions ciblees

- Amender E001 sous consultation triviale, sans tester H006.
- Retirer la force attribuee au Claim ; etablir une raison positive de cooperation.
- Separer acces, recherche, selection, consultation, applicabilite et decision.
- Formaliser trace atomique, controles negatifs et permutation d'identifiants.
- Mettre a jour le statut de H001 et la provenance documentaire.

## Hors scope

Rust ou Python, GPE, moteur social generique, LLM, E004, gameplay ou couplages a NOCTIS et Smog City ; fusion dans `main`.
