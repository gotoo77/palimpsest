# P005 - Hypothese sur l'economie de l'attention

## Statut de source

- Identifiant : `P005`
- Date : 2026-07-18
- Statut : texte source conserve depuis le contexte de session
- Objet : formuler comme hypothese de recherche la question "Pourquoi une entite consulte-t-elle un artefact ?"

## Texte du prompt

Pour **PALIMPSEST**, il y a quelque chose qui m'a frappé en relisant toute la trajectoire.

Au début, on parlait d'un jeu où les artefacts constituaient une mémoire.

Aujourd'hui, on parle de :

Event
EventRecord
Claim
Decision
Trace

Et Codex a commencé à les séparer correctement.

Mais je crois qu'il manque encore une question fondatrice.

Une question tellement fondamentale qu'elle devrait probablement précéder toute implémentation.

La voici :

**Pourquoi une entité consulte-t-elle un artefact ?**

Ça peut paraître anodin, mais je pense que c'est le prochain verrou conceptuel.

Parce que si la réponse est :

"Elle consulte tous les artefacts accessibles."

...alors le jeu devient rapidement une machine déterministe où toute information pertinente est toujours prise en compte.

À l'inverse, si la réponse est :

"Elle consulte celui qui influence le plus son score."

...on est déjà en train de coder une IA décisionnelle classique.

Or, dans un monde vivant, les individus ne consultent pas toute la mémoire disponible.

Ils choisissent.

Ils ignorent.

Ils oublient.

Ils privilégient certaines sources.

Ils font confiance à certains auteurs.

Ils cherchent des informations particulières.

Ils se trompent.

Ils abandonnent une recherche.

Ils interprètent différemment un même document.

Autrement dit, je crois que **l'émergence ne naîtra pas seulement de la production des artefacts**.

Elle naîtra aussi de **l'économie de l'attention**.

Et ça change beaucoup de choses.

Aujourd'hui, ton pipeline ressemble à :

Event
    ↓
Claim
    ↓
Decision

Je soupçonne qu'il devrait plutôt devenir :

Event
    ↓
EventRecord
    ↓
Claim
    ↓
Recherche
    ↓
Sélection
    ↓
Consultation
    ↓
Interprétation
    ↓
Décision

Les trois nouvelles étapes sont capitales :

**Recherche** : quels artefacts sont susceptibles d'être utiles ?
**Sélection** : lesquels vais-je réellement consulter ?
**Interprétation** : que signifient-ils pour moi ?

Et c'est peut-être là que réside le vrai jeu.

Le joueur n'observerait plus seulement une mémoire qui influence un monde.

Il observerait un monde où les habitants développent progressivement **des stratégies de consultation de leur propre mémoire**.

À ce moment-là, deux sociétés ayant exactement les mêmes artefacts pourraient évoluer différemment simplement parce qu'elles ne les consultent pas de la même manière.

Et ça... je trouve que c'est une piste extraordinairement prometteuse.

Je ne la transformerais surtout pas en décision d'architecture tout de suite. Je la noterais comme une **hypothèse de recherche**.

Parce qu'elle est falsifiable.

Peut-être découvrirez-vous, toi et Codex, que la consultation peut rester triviale sans nuire au jeu. Ou peut-être qu'au contraire, vous constaterez que c'est précisément l'apparition de politiques de consultation différentes qui fait émerger les comportements les plus intéressants.

À mon avis, c'est exactement le genre de question qui mérite un futur H006 plutôt qu'une ligne de code.
