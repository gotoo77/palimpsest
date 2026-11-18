# E005 - Le pont : deux demandes concurrentes

- Date : 2026-11-18
- Statut : **experience exploratoire H003, prototype headless en PR, non validee ludiquement**
- Origine : retour utilisateur de P2 via [issue #3](https://github.com/gotoo77/palimpsest/issues/3), et prolongement du protocole E001
- Ne remplace pas E001 et ne teste pas H006.

## Probleme observe

La scene P2 de E001 affiche une causalite fonctionnelle mais ressemble, selon le retour utilisateur, a un **outil de demonstration plutot qu'a un jeu**. Un bouton inverse une action, sans arbitrage.

## Hypothese ludique, non encore verifiee

Introduire **deux besoins legitimes concurrents** et un temps de travail fini peut transformer l'intervention sur l'accessibilite de la memoire en choix d'allocation, plutot qu'en simple prediction d'une reponse.

Cette hypothese concerne le **gameplay H003**. L'experience ne teste ni la veracite des allegations ni un modele economique credible.

## Situation et ressources

- Ada a **un seul creneau de travail** pour la journee.
- Demande 1 : Borin sollicite Ada pour reparer un pont.
- Demande 2 : Ciro sollicite Ada pour renforcer un abri.
- Le Claim historique de Ciro concerne **Borin** uniquement.
- Les deux demandes sont chacune faisables en un creneau **si Ada l'accorde et si ce creneau existe encore**.
- Les artefacts `EventRecord` et `Claim` restent inchanges et precharges.
- Le joueur ne peut modifier que l'accessibilite du Claim a Ada, **avant** les demandes.

Une seule variable change entre les deux runs : acces d'Ada au Claim de Ciro.

## Regles d'execution minimales

1. Construire le meme monde de base pour les deux runs et ne changer que le masque d'acces d'Ada.
2. **Intention 1** : utiliser `palimpsest::decide` pour la demande de Borin.
3. Si Ada accepte, le pont est repare et son seul creneau est consomme.
4. **Intention 2** : utiliser **la meme fonction** `decide`, cette fois pour une demande de Ciro. Le Claim portant sur Borin ne doit pas etre applicable a la demande de Ciro, meme si Ada le consulte.
5. Si Ada accepte la deuxieme demande et dispose encore d'un creneau, l'abri est renforce ; sinon, il ne l'est pas.
6. Conserver et afficher les deux traces de decision, et distinguer l'intention d'aider de la capacite reelle a le faire.

**Attention :** la fonction E001 ne modelise pas le temps disponible. L'application de la contrainte de temps **apres** les decisions est une regle nouvelle, explicite et bornee a ce prototype. Elle ne prouve pas que les agents raisonnent sur leurs ressources.

## Predictions de test (pas des observations ludiques)

| Acces du Claim | Ada envers Borin | Pont | Ada envers Ciro | Abri |
| --- | --- | --- | --- | --- |
| Visible | Refuse | Non repare | Accepte, creneau disponible | Renforce |
| Masque | Accepte | Repare | Accepte en intention, plus de creneau | Non renforce |

Les deux branches procurent un gain concret et entrainent un renoncement. Aucune ne permet de tout sauver.

## Test falsifiable

- La comparaison A/B doit donner deux resultats differents avec un seul masque modifie.
- L'ordre des demandes, le creneau initial et les regles restent identiques.
- Les deux intentions doivent provenir de `decide`, sans `if Ada`, `if Borin`, ou decision scriptant un resultat a partir de l'option du joueur.
- Le Claim ne doit pas s'appliquer a un autre partenaire.
- L'information textuelle du Claim ne doit pas etre executable.
- Les traces et les resultats doivent etre reproductibles.
- Un seul creneau implique qu'on ne peut realiser les deux travaux.
- Les references de provenance invalides doivent toujours bloquer le calcul.

## Test humain a faire APRES la preuve technique

Presenter les deux besoins et les consequences **sans montrer les resultats**. Observer si le joueur :

1. identifie deux objectifs legitimes et incompatibles ;
2. sait expliquer en quoi son intervention sur la memoire affecte l'allocation ;
3. ressent un arbitrage ou choisit simplement un bouton deja evident ;
4. souhaite rejouer pour une nouvelle raison, et non pour voir l'autre branche.

**REVISE** si le dispositif reste une table de verite narrative. Un succes de compilation n'est pas un GO pour H003.

## Revue adversariale

- **Risque majeur :** le jeu se resume encore a un booleen, habille en deux resultats. Il n'y a toujours ni incertitude, ni deuxieme intervention.
- **Autre risque :** la seconde demande est une intention toujours favorable selon les regles d'E001 ; le blocage vient du creneau, pas d'un agent qui arbitre entre les deux projets.
- **Important :** masquer une accusation n'entraine ici **aucune consequence de confiance ou de reputation**. En inventer une sans protocole serait encore du scenario precable.
- **Limite scientifique :** les artefacts sont precharges ; aucun souvenir n'est produit, propage, oublie ou transmis dans ce test.
- **Absence de GO architectural :** ne pas tirer de ce test une justification pour un simulateur social, un moteur de besoins ou un nouveau composant GPE.

## Implementation minimale

Le test headless `src/bridge.rs` reutilise le crate racine ; la CLI actuelle `compare` reste compatible.

```bash
cargo run --locked -- bridge
cargo run --locked -- bridge visible
cargo run --locked -- bridge masque
cargo test --locked
```

Le texte est en francais par defaut. Aucun changement d'interface GPE, de core E001 ou de dependance n'est prevu.
