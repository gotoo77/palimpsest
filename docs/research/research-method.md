# Methode de recherche emergente

## Statut

- Identifiant : `D010`
- Date : 2026-07-18
- Source : `P008`
- Statut : cadrage methodologique, non decisionnel
- Portee : PALIMPSEST d'abord ; portee generale possible, non validee.

Ce document ne cree pas un nouveau projet et ne transforme pas PALIMPSEST en framework methodologique. Il preserve une observation : la facon de travailler sur PALIMPSEST commence a former une methode de recherche explicite.

## Sequence observee

La dynamique initiale courante d'un projet logiciel peut etre resumee ainsi :

```text
idee
-> implementation
-> ca marche / ca ne marche pas
```

La dynamique observee dans PALIMPSEST devient plutot :

```text
vision
-> hypothese
-> protocole experimental
-> revue critique
-> nouvelle hypothese
-> analyse d'impact
-> decision de statut
   - inchange
   - amende
   - remplace
-> implementation eventuelle
```

L'implementation n'est plus le prochain reflexe. Elle devient une issue possible d'une chaine de raisonnement.

## Domaine de validite

Une experience ou un protocole n'est pas simplement vrai ou faux. Il est valide dans un domaine explicitement qualifie.

Exemple issu de `C003` :

`H006` n'annule pas `E001`. Elle revele que `E001` reste valable seulement si sa politique de consultation triviale est declaree et controlee.

Cette logique evite deux erreurs symetriques :

- jeter trop vite un protocole utile lorsqu'une hypothese nouvelle apparait ;
- maintenir un protocole comme general alors que son domaine de validite vient de se resserrer.

## Artefacts methodologiques

La memoire de conception ne suit plus seulement des idees isolees. Elle suit aussi les relations entre elles.

Familles d'artefacts actuellement visibles ou anticipees :

- visions ;
- prompts structurants ;
- hypotheses ;
- experiences candidates ;
- protocoles experimentaux ;
- revues critiques ;
- analyses d'impact ;
- decisions futures ;
- observations ;
- resultats ;
- questions ouvertes ;
- idees differees ou rejetees ;
- rapports de run.

Les analyses d'impact occupent une place particuliere. Elles demandent :

> Quelles connaissances existantes deviennent incompletes si cette nouvelle hypothese est vraie ?

Elles ne servent pas a invalider automatiquement les artefacts precedents. Elles servent a qualifier leur domaine de validite.

## Graphe de connaissances vivant

Le depot commence a fonctionner comme un graphe de connaissances vivant plutot que comme une documentation lineaire.

Cette affirmation reste descriptive, pas prescriptive. Le projet ne doit pas encore introduire de base de donnees, d'ontologie executable ou d'outil de graphe. La forme Markdown reste suffisante tant qu'elle permet :

- d'identifier les artefacts ;
- de conserver les sources primaires ;
- de relier les transformations ;
- de distinguer hypothese, protocole, revue, impact et decision ;
- de comparer les tranches par Git.

## Portee potentiellement generale

La methode pourrait s'appliquer hors PALIMPSEST, par exemple a :

- un moteur IA ;
- un compilateur ;
- un langage ;
- une architecture distribuee ;
- un protocole reseau ;
- un projet scientifique.

Cette portee n'est pas encore une decision ni un objectif. C'est une observation a surveiller. Le risque serait de transformer prematurement une methode emergente en framework autonome.

## Garde-fous

- Ne pas extraire la methode en nouveau projet sans friction ou besoin externe documente.
- Ne pas ajouter d'automatisation tant que Markdown et Git suffisent.
- Ne pas confondre richesse documentaire et progression reelle du modele ou du jeu.
- Ne pas laisser la methode devenir plus importante que les questions scientifiques et ludiques de PALIMPSEST.
- Continuer a qualifier le domaine de validite des protocoles au lieu de les classer seulement comme valides ou invalides.

## Questions ouvertes

- Quels seuils permettent de passer d'une hypothese a un protocole, puis d'un protocole a une implementation ?
- Quand une analyse d'impact est-elle necessaire ?
- Comment eviter que le graphe documentaire devienne plus couteux que les connaissances qu'il preserve ?
- Quelle partie de cette methode est propre a PALIMPSEST, et quelle partie pourrait etre reutilisee ailleurs ?
- Quels signes indiqueraient qu'il est temps d'extraire cette methode comme patrimoine independant ?
