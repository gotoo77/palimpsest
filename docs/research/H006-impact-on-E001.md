# C003 - Analyse d'impact de H006 sur E001

## Statut et portee

- Identifiant : `C003`
- Date : 2026-07-18
- Objet analyse : impact de `H006` sur `E001`
- Sources : `P007`, `H006`, `E001`, `C002`
- Statut : analyse d'impact, non decisionnelle
- Verdict : `H006` n'invalide pas `E001`, mais rend incompletes plusieurs hypotheses implicites du protocole.

Cette analyse ne corrige pas `E001`. Elle determine d'abord si le protocole peut etre amende, doit etre remplace, ou reste valide en l'etat.

## Question analysee

Est-ce que `H006` invalide une partie des hypotheses implicites de `E001` ?

Reponse courte : oui, partiellement.

`E001` reste utile pour tester une version bornee de `H001`, mais seulement si sa politique de consultation est explicite et triviale. Si `H006` est vraie, `E001` ne suffit plus a representer la chaine complete par laquelle un artefact accessible devient une influence decisionnelle.

## Hypothese implicite principale de E001

`E001` suppose pratiquement :

```text
claim accessible
-> claim consulte
-> claim interprete
-> decision influencee
```

`H006` introduit une chaine plus longue :

```text
knowledge space
-> recherche
-> selection
-> consultation
-> interpretation
-> decision
```

La difference n'est pas cosmetique. Elle ajoute des points d'echec entre accessibilite et decision.

## Hypotheses implicites rendues incompletes

### I001 - Accessibilite et consultation sont presque equivalentes

Dans `E001`, un claim accessible et pertinent est consulte par construction. `C002` l'avait deja signale comme finding `F003`.

Impact de `H006` : cette equivalence devient une hypothese forte, pas un detail d'implementation.

Consequence : `E001` doit soit declarer une politique triviale `consult_all_accessible_relevant_claims`, soit etre refondu pour tester recherche, selection et consultation comme etapes distinctes.

### I002 - Le non-acces suffit a expliquer la non-consultation

Dans le Run B de `E001`, `Ada` ne consulte pas `claim-001` parce qu'il est inaccessible.

Impact de `H006` : une entite pourrait ne pas consulter un claim meme s'il est accessible.

Consequence : Run A ne peut plus etre resume a `claim_accessible = true`. Il doit aussi prouver que le claim a ete recherche, selectionne et consulte, ou declarer que ces etapes sont neutralisees par protocole.

### I003 - La variable independante suffit a isoler la causalite

`E001` isole l'accessibilite de `claim-001`.

Impact de `H006` : si la consultation est un choix, alors l'accessibilite n'est qu'une condition necessaire. La causalite peut echouer a quatre nouveaux niveaux : non-recherche, non-selection, non-consultation, non-applicabilite.

Consequence : la variable independante de `E001` reste propre pour tester l'acces, mais incomplete pour tester la chaine attentionnelle complete.

### I004 - L'absence de divergence affaiblit directement H001

Dans `E001`, si le claim est accessible mais ne modifie pas la decision, `H001` peut etre affaiblie.

Impact de `H006` : l'absence de divergence peut venir d'une politique de consultation, pas de l'incapacite du claim a influencer une decision une fois consulte.

Consequence : un echec de `E001` devient ambigu si la trace ne distingue pas recherche, selection, consultation et interpretation.

### I005 - La trace causale minimale est suffisante

`E001` demande une trace `accessible -> consulte -> interpretation -> decision`.

Impact de `H006` : cette trace manque deux etapes critiques, recherche et selection, et ne permet pas d'identifier ou la chaine attentionnelle echoue.

Consequence : la trace causale minimale de `E001` doit etre etendue ou explicitement limitee a une politique de consultation triviale.

## Ce que H006 n'invalide pas

`H006` ne contredit pas `H001`.

`H006` ne rend pas inutile `E001`.

`H006` ne prouve pas qu'il faut implementer un systeme d'attention.

`H006` ne prouve pas que la consultation triviale serait insuffisante pour un premier prototype.

`H006` dit seulement : si la consultation est elle-meme un choix, alors un protocole qui teste uniquement l'accessibilite ne couvre pas toute la causalite entre memoire et decision.

## Classification de E001 apres H006

### Option A - E001 reste valide en l'etat

Rejetee.

Raison : `E001` ne declare pas assez clairement qu'il suppose une politique de consultation triviale. En l'etat, il peut etre lu comme un test general de `Claim accessible -> decision`, alors qu'il teste plutot `Claim accessible sous consultation triviale -> decision`.

### Option B - E001 doit etre remplace

Rejetee a ce stade.

Raison : `E001` reste utile comme protocole de base pour tester `H001` dans un monde volontairement simplifie. Le remplacer maintenant risquerait de perdre une experience minimale deja bien delimitee.

### Option C - E001 doit etre amende et requalifie

Retenue.

`E001` devrait devenir explicitement :

> un protocole minimal sous politique de consultation triviale et controlee, destine a tester si un claim effectivement consulte peut influencer une decision de maniere lisible.

Cette requalification evite de pretendre que `E001` teste deja l'economie de l'attention.

### Option D - Creer E001.1 ou E004 apres amendement

Retenue comme prochaine etape possible, mais non immediate.

Une fois `E001` requalifie, une experience ulterieure pourra tester la meme situation avec une politique de recherche/selection non triviale. Cette experience pourrait etre `E001.1` si elle reste une variante directe, ou `E004` si elle cible principalement `H006`.

## Protocoles existants affectes

### E001

Impact : fort.

`E001` suppose une consultation triviale. Il doit etre amende avant implementation pour expliciter cette hypothese et etendre la trace minimale.

### E002

Impact : probable, mais non urgent.

La chaine `EventRecord -> Claim -> Norm` dependra fortement de ce que les entites consultent ou ignorent. Si `H006` est vraie, l'emergence d'une norme ne peut pas dependre seulement de la presence des artefacts.

### E003

Impact : fort.

L'oubli par baisse d'influence ou masquage ne peut pas etre separe longtemps de la consultation. Un artefact non consulte ressemble parfois a un artefact oublie, meme s'il reste accessible.

### E004

Impact : fondateur.

`E004` devient la candidate naturelle pour tester directement `H006`, mais elle ne doit pas remplacer `E001` avant que le protocole minimal de `H001` soit proprement borne.

## Nouvelle regle de discipline proposee

Chaque nouvelle hypothese importante devrait declencher une question d'impact :

> Quels protocoles existants deviennent incomplets si cette hypothese est vraie ?

Cette question ne sert pas a invalider les documents precedents. Elle sert a eviter que des protocoles localement coherents reposent sur des hypotheses profondes devenues implicites ou contradictoires.

## Recommandation

Ne pas passer directement a une correction de contenu de `E001`.

Prochaine etape recommandee :

1. amender le statut de `E001` pour dire qu'il teste `H001` sous politique de consultation triviale ;
2. ajouter a `E001` une section "hypotheses attentionnelles neutralisees" ;
3. etendre la trace causale de `E001` avec `search_performed`, `selection_policy`, `selected_for_consultation`, `consulted_artifacts`, `applicability_checks` ;
4. maintenir `E004` comme experience candidate distincte pour tester `H006` ;
5. ne formaliser `E004` qu'apres avoir stabilise la portee de `E001`.

## Conclusion

`H006` ne detruit pas `E001`.

Elle revele que `E001` etait plus etroit que sa formulation pouvait le laisser croire. Le protocole reste amendable, mais il doit etre requalifie comme test sous attention neutralisee. Sans cette requalification, corriger `E001` risquerait d'optimiser une experience construite sur une hypothese devenue invisible : l'idee qu'un artefact accessible est naturellement consulte.
