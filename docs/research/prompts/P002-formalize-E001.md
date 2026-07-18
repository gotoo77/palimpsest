# P002 - Formaliser l'experience E001 avant implementation

## Statut de source

- Identifiant : `P002`
- Date : 2026-07-18
- Statut : texte source conserve depuis le prompt joint a la session
- Source locale initiale : `/home/gduval/.codex/attachments/2639174b-cc6c-4ae6-bc41-6f3c0bcce6f0/pasted-text.txt`

## Texte du prompt

Pour PALIMPSEST, la prochaine étape est très nette : ne pas initialiser tout le jeu. Il faut d’abord transformer E001 en protocole expérimental précis.

La revue de Codex a fait émerger le vrai noyau à tester :

À monde identique, un artefact accessible peut-il provoquer une décision différente, de manière reproductible et causalement inspectable ?

Je donnerais maintenant ceci à Codex :

Codex Prompt — Formalisation de l’expérience E001 de PALIMPSEST

Salut Codex,

Je souhaite poursuivre le travail sur PALIMPSEST à partir des documents présents dans :

docs/research/

Lis en priorité :

codex-initialize-palimpsest-review.md ;
open-hypotheses.md ;
concept-evolution.md ;
artifact-provenance.md ;
prompt-history.md.

La revue précédente recommande de formaliser l’expérience E001 — Divergence minimale par accès à un Claim avant toute implémentation.

Je souhaite que tu réalises cette étape maintenant.

Mission

Tu interviens comme :

Game Systems Designer ;
Simulation Engineer ;
Research Engineer ;
Software Architect ;
Critical Reviewer.

Ta mission n’est pas encore de construire PALIMPSEST.

Ta mission est de concevoir le plus petit protocole expérimental capable de tester honnêtement H001 :

Un artefact consulté peut modifier une décision future de manière lisible.

L’expérience doit comparer deux exécutions aussi identiques que possible :

Run A
Même monde, mêmes entités, mêmes règles, même seed
Un Claim pertinent est accessible à une entité

Run B
Même monde, mêmes entités, mêmes règles, même seed
Ce Claim est absent ou inaccessible

La seule différence causale intentionnelle entre les deux runs doit être l’accès au Claim.

Questions auxquelles le protocole doit répondre

Formalise précisément :

Quel est le monde minimal nécessaire ?
Combien d’entités sont réellement nécessaires ?
Quelle décision concrète doit pouvoir diverger ?
Quel événement antérieur justifie le Claim ?
Que contient exactement le Claim ?
Quelle entité peut le consulter ?
Pourquoi peut-elle ou non y accéder ?
Comment le Claim influence-t-il la décision ?
Comment éviter qu’il ne soit qu’un simple modificateur numérique décoré ?
Quelle trace permet d’attribuer la divergence à sa consultation ?
Quelles autres variables doivent être contrôlées ?
Quels résultats renforceraient H001 ?
Quels résultats l’affaibliraient ?
Quels résultats rendraient l’expérience inconclusive ?
Qu’est-ce que cette expérience ne permettrait absolument pas de conclure ?

Exigence centrale

Ne conçois pas E001 pour qu’elle réussisse artificiellement.

Le protocole doit pouvoir :

confirmer l’hypothèse ;
l’affaiblir ;
produire un résultat ambigu ;
révéler que le concept de Claim est mal défini ;
révéler que la divergence existe mais n’est pas lisible ;
révéler que l’artefact n’est en réalité qu’une variable ordinaire enveloppée dans un objet.

La possibilité d’un échec expérimental doit rester réelle.

Distinctions obligatoires

Distingue clairement :

l’événement réellement survenu dans le monde ;
sa trace technique éventuelle ;
l’EventRecord accessible dans le monde ;
le Claim formulé à propos de cet événement ;
la croyance éventuelle de l’entité ;
l’influence du Claim sur une décision ;
la décision finalement prise ;
la trace causale utilisée par l’observateur.

Ne fusionne pas :

vérité
mémoire
croyance
influence
décision
trace technique

Simplicité

Cherche le modèle minimal.

Le protocole ne doit pas nécessiter :

de Norm ;
d’institution ;
d’oubli ;
de contestation complexe ;
de carte étendue ;
d’économie ;
de dialogue génératif ;
de modèle de langage ;
de psychologie réaliste ;
de système social complet ;
de moteur générique de règles ;
de véritable interface de jeu.

Deux ou trois entités suffisent probablement.

Une seule décision significative suffit.

Un seul Claim suffit.

Formalisation attendue

Crée le document :

docs/experiments/E001-minimal-claim-divergence.md

Il devra contenir au minimum :

statut du document ;
hypothèse testée ;
question expérimentale ;
périmètre ;
modèle minimal du monde ;
entités ;
ressources ou objets nécessaires ;
état initial ;
événement antérieur ;
définition exacte du Claim ;
règle d’accessibilité ;
règle de consultation ;
mécanisme minimal d’interprétation ;
décision cible ;
configuration du Run A ;
configuration du Run B ;
variables contrôlées ;
variable indépendante ;
variables observées ;
trace causale attendue ;
critères de succès ;
critères d’échec ;
résultats inconclusifs possibles ;
risques de faux positif ;
risques de scénario implicitement scripté ;
limites de l’expérience ;
pseudo-code strictement minimal ;
fixtures ou exemples de données candidats ;
tests qui seraient nécessaires lors d’une future implémentation ;
questions restant ouvertes.

Pseudo-code

Le pseudo-code doit montrer la chaîne complète :

état initial
→ perception
→ accessibilité du Claim
→ consultation éventuelle
→ interprétation
→ choix entre deux actions
→ décision
→ trace causale

Il doit également montrer que, sans accès au Claim, les deux runs suivent exactement les mêmes règles.

Évite de proposer une architecture logicielle complète.

Le pseudo-code doit servir à examiner la validité du protocole, pas à pré-écrire le moteur du jeu.

Résultats observables

Définis un résultat minimal de ce type :

Run A:
  claim_accessible = true
  claim_consulted = true
  chosen_action = refuse
  decision_references = [claim-001]

Run B:
  claim_accessible = false
  claim_consulted = false
  chosen_action = cooperate
  decision_references = []

Cet exemple n’est pas une réponse imposée.

Critique-le et remplace-le si une autre décision produit une expérience plus honnête ou plus intéressante.

La divergence seule ne suffit pas.

Il faut pouvoir expliquer :

pourquoi elle s’est produite ;
quelle règle a utilisé le Claim ;
si elle aurait pu se produire sans lui ;
si elle reste reproductible.

Mémoire de conception

Enregistre le présent prompt dans :

docs/research/prompt-history.md

Attribue-lui le prochain identifiant disponible.

Mets à jour uniquement les documents de recherche réellement concernés :

timeline.md ;
artifact-provenance.md ;
open-hypotheses.md ;
éventuellement concept-evolution.md.

Ne mets pas à jour artificiellement tous les registres.

Le nouveau document expérimental doit recevoir un identifiant stable et être relié à :

H001 ;
E001 ;
la revue C001 ;
le présent prompt.

Interdiction d’implémentation

Ne crée pas encore :

de projet Python ;
de pyproject.toml ;
de moteur de simulation ;
de CLI ;
de classes de domaine ;
de tests exécutables ;
de code de production.

Cette tranche est exclusivement conceptuelle et expérimentale.

Une implémentation ne sera envisagée qu’après revue du protocole.
