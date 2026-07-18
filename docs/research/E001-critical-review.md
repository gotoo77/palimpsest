# C002 - Revue critique du protocole E001

## Statut et portee

- Identifiant : `C002`
- Date : 2026-07-18
- Objet revu : `E001`, `docs/experiments/E001-minimal-claim-divergence.md`
- Hypothese concernee : `H001`
- Source : `P003`
- Statut : revue critique, non decisionnelle
- Verdict : `E001` est conceptuellement solide, mais ne devrait pas encore etre implemente sans corrections mineures a moyennes.

Cette revue attaque le protocole comme experience, pas comme architecture logicielle. Elle cherche les biais, les faux positifs, les confusions de variables et les signes de scenario cache.

## Resume critique

`E001` accomplit le travail principal attendu : il separe clairement fait, trace, interpretation, accessibilite, influence, decision et trace causale. Le choix de garder `claim-001` present dans les deux runs mais inaccessible a `Ada` dans le Run B est methodologiquement fort.

Le protocole reste cependant vulnerable a un faux positif : la decision cible, l'action par defaut et la force du claim sont definies de telle maniere que le resultat attendu peut etre obtenu par une regle presque tautologique. Cela n'invalide pas `E001`, mais impose de durcir les controles avant implementation.

## Points forts

- La variable independante est nette : accessibilite de `claim-001` a `Ada`.
- Les deux runs gardent le meme evenement source, le meme record, le meme claim, les memes entites et les memes regles.
- Le protocole distingue correctement `event-001`, `record-001`, `claim-001`, croyance locale, influence et decision.
- Le mecanisme assume sa modestie : il teste une causalite minimale, pas l'emergence ni la profondeur sociale.
- Les criteres d'echec et d'inconclusion existent deja ; l'experience peut theoriquement affaiblir `H001`.

## Findings

### F001 - La force du Claim preconfigure le succes

Gravite : moyenne.

Le champ `force experimentale : suffisante pour changer une decision neutre` encode deja l'effet attendu. Meme si l'influence reste inspectable, cette formulation peut transformer `claim-001` en variable causale garantie.

Risque : le protocole prouve seulement qu'une condition concue pour inverser une decision inverse une decision.

Correction recommandee : remplacer cette force par une regle d'applicabilite structurelle. Par exemple : un claim de polarite `caution_against_cooperation`, ciblant le partenaire, dans une portee compatible, et referencant une violation de promesse envers l'acteur, devient une raison prioritaire dans une situation neutre. La priorite doit appartenir a la regle generale, pas au claim individuel.

### F002 - L'action par defaut rend le Run B trop passif

Gravite : moyenne.

Run B choisit `cooperate` parce qu'aucune influence applicable n'est trouvee. Cela fonctionne pour une experience minimale, mais le protocole doit montrer que `cooperate` n'est pas un simple placeholder choisi pour maximiser la divergence.

Risque : la divergence mesure "claim present dans le filtre" contre "liste vide", pas une vraie decision sous regles equivalentes.

Correction recommandee : expliciter la raison positive du choix par defaut, par exemple `baseline_cooperation_when_cost_is_low_and_relation_neutral`. Run B doit aussi produire une raison referencee, meme si elle ne reference aucun artefact.

### F003 - Consultation et accessibilite sont encore trop proches

Gravite : moyenne.

Le pseudo-code consulte tout claim accessible et pertinent. Cela rend `claim_accessible = true` presque equivalent a `claim_consulted = true`.

Risque : l'experience ne teste pas vraiment la consultation ; elle teste seulement la presence dans un filtre.

Correction recommandee : distinguer trois etapes : decouverte des artefacts accessibles, selection des artefacts consultes, puis applicabilite. Pour E001, la selection peut rester deterministe, mais elle doit produire une trace separee du type `consultation_policy = consult_all_accessible_relevant_claims`.

### F004 - L'accessibilite contient deja une condition de portee decisionnelle

Gravite : faible a moyenne.

La regle d'accessibilite inclut "l'artefact est dans la portee de la decision courante". Cela melange deux notions : pouvoir acceder a un artefact et juger qu'il est applicable a une decision.

Risque : un claim devient inaccessible parce qu'il est non pertinent, alors que le protocole veut distinguer accessibilite, consultation et applicabilite.

Correction recommandee : separer `accessible_artifacts` et `applicable_claims`. L'accessibilite devrait dependre de presence, audience, masquage et eventuellement localisation. La portee devrait etre testee pendant l'applicabilite.

### F005 - Le scenario reste proche d'un cas unique encode

Gravite : moyenne.

Les noms `Ada`, `Borin`, `Ciro`, `resource-r1`, `claim-001` sont acceptables comme fixture, mais la regle `prior_promise_violation_claim` pourrait etre implemente comme cas particulier de cette histoire.

Risque : E001 reussit seulement pour "Borin a pris la ressource promise a Ada".

Correction recommandee : ajouter des controles negatifs conceptuels avant implementation :

- meme claim, cible autre que `Borin` : pas d'effet sur la decision avec `Borin` ;
- meme cible, portee non cooperative : pas d'effet ;
- claim accessible a une autre entite que `Ada` : pas d'effet sur `Ada` ;
- `record-001` accessible sans `claim-001` applicable : pas d'effet dans E001.

### F006 - La trace causale peut devenir declarative plutot que probante

Gravite : moyenne.

Le protocole exige une trace causale, mais il ne precise pas comment eviter qu'elle repete simplement l'intention de la regle.

Risque : la trace dit "claim consulte donc refus" sans exposer les etapes qui rendraient l'attribution auditable.

Correction recommandee : imposer une trace en champs atomiques derives de l'execution : `accessible_artifact_ids`, `selected_for_consultation`, `applicability_checks`, `decision_reasons`, `chosen_action`. La trace doit montrer aussi les checks echoues dans les controles negatifs.

### F007 - Le protocole ne definit pas encore le seuil d'autorisation d'implementation

Gravite : faible.

`E001` dit qu'une revue humaine doit valider le protocole, mais ne dit pas quelles corrections sont bloquantes.

Risque : une implementation commence avec des ambiguites connues, puis les durcit en architecture.

Correction recommandee : ajouter une section "conditions avant implementation" distinguant corrections bloquantes et ameliorations optionnelles.

## Faux positifs principaux

- Le claim est equivalent a `force_refusal_against_borin`.
- La decision par defaut existe seulement pour contraster avec le refus.
- La consultation est automatique et donc non distinguee de l'accessibilite.
- La trace causale est produite apres coup comme justification, pas comme resultat de la chaine de decision.
- La regle est generale en texte mais speciale en fixture.

## Faux negatifs possibles

- Le protocole peut affaiblir `H001` si la trace est mauvaise, meme si l'idee d'artefact influent reste valide.
- Un claim pourrait etre conceptuellement pertinent mais ne pas changer une decision parce que la situation neutre ou le cout d'aide sont mal choisis.
- La restriction "pas de croyance persistante" peut rendre l'influence trop locale pour representer ce que PALIMPSEST visera ensuite.

## Attaques methodologiques recommandees

Avant implementation, `E001` devrait inclure au minimum ces variantes conceptuelles :

### Controle positif

Run A et Run B actuels :

- Run A : `claim-001` accessible a `Ada`, decision attendue `refuse`.
- Run B : `claim-001` masque pour `Ada`, decision attendue `cooperate`.

### Controle negatif 1 - Claim hors cible

`claim-001` cible une autre entite que `Borin`.

Resultat attendu : `Ada` coopere avec `Borin`, meme si le claim est accessible.

### Controle negatif 2 - Claim hors portee

`claim-001` concerne une portee non cooperative.

Resultat attendu : `Ada` coopere avec `Borin`, et la trace montre que le claim a ete consulte mais juge non applicable, ou accessible mais non selectionne selon la politique retenue.

### Controle negatif 3 - Record sans Claim

`record-001` est disponible au niveau observateur, mais aucun claim applicable n'est accessible a `Ada`.

Resultat attendu : `Ada` coopere, afin de confirmer que `EventRecord` et `Claim` ne sont pas fusionnes.

### Controle negatif 4 - Claim accessible a la mauvaise entite

`claim-001` est accessible a `Ciro` ou a une entite non decisionnaire, mais pas a `Ada`.

Resultat attendu : `Ada` coopere.

Ces variantes ne demandent pas encore plus de simulation ; elles durcissent seulement la falsifiabilite du protocole.

## Conditions avant implementation

Corrections bloquantes :

- retirer ou reformuler `force experimentale : suffisante pour changer une decision neutre` ;
- separer accessibilite, selection pour consultation et applicabilite ;
- ajouter une raison explicite pour l'action par defaut dans Run B ;
- definir les champs atomiques minimaux de la trace causale ;
- ajouter au moins deux controles negatifs au protocole.

Corrections utiles mais non bloquantes :

- nommer la politique de consultation deterministe ;
- preciser que les controles negatifs utilisent les memes regles que le controle positif ;
- garder la comparaison "claim absent" contre "claim masque" hors de E001, comme variante ulterieure.

## Verdict

`E001` ne doit pas encore etre implemente tel quel.

Le protocole est assez solide pour servir de base, mais il doit etre revise avant autorisation d'implementation. La revision doit viser un objectif precis : rendre impossible une reussite par simple variable decoree ou par scenario encode.

Une fois les corrections bloquantes integrees, une implementation minimale de fixtures et tests conceptuels deviendra defendable. Cette implementation ne devrait toujours pas construire PALIMPSEST comme jeu ; elle devrait seulement verifier la chaine causale de `H001`.
