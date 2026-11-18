# E001 - Divergence minimale par acces a un Claim

## Statut du document

- Identifiant : `E001`
- Statut : protocole experimental conceptuel, non implemente ; revision documentaire P0 du 2026-11-18, soumise a revue humaine
- Date : 2026-07-18
- Hypothese cible : `H001`
- Sources : `C001`, `P002`
- Objet : tester la chaine minimale `artefact consulte -> influence interpretable -> decision divergente -> trace causale inspectable`.

Ce document n'autorise pas encore l'implementation de PALIMPSEST. Il definit seulement une experience minimale qui pourra etre revue, critiquee, puis eventuellement transformee en fixtures et tests executables.

## Hypothese testee

`H001` : un artefact consulte peut modifier une decision future de maniere lisible.

La version testee ici est plus stricte :

> A monde identique, regles identiques et seed identique, rendre un `Claim` accessible a une entite peut produire une decision differente, et cette divergence peut etre attribuee a la consultation de ce `Claim` par une trace causale explicite.

## Question experimentale

Dans une micro-simulation deterministe, une entite choisit-elle differemment entre `cooperate` et `refuse` lorsque la seule difference intentionnelle entre deux runs est l'accessibilite d'un `Claim` pertinent sur la fiabilite d'un partenaire ?

## Perimetre

Inclus :

- un monde abstrait sans carte etendue ;
- trois entites au maximum ;
- une ressource promise ;
- un evenement anterieur ;
- un `EventRecord` accessible au niveau observateur ;
- un seul `Claim` accessible ou inaccessible selon le run ;
- une decision cible binaire ;
- une trace causale courte et inspectable.

Exclus :

- `Norm` ;
- institution ;
- oubli ;
- contestation complexe ;
- economie ;
- dialogue generatif ;
- modele de langage ;
- psychologie realiste ;
- moteur generique de regles ;
- interface de jeu ;
- code de production.

## Modele minimal du monde

Le monde contient une situation de cooperation autour d'une ressource indivisible.

Une entite `Ada` doit choisir si elle aide `Borin` a recolter une ressource commune. Aider coute une action a `Ada` et beneficie a `Borin`. Refuser conserve l'action d'`Ada`.

Un evenement anterieur indique que `Borin` a deja pris une ressource qui etait promise a `Ada`. Cet evenement existe dans l'histoire du monde des deux runs. La difference entre les runs ne porte pas sur l'evenement, mais sur l'accessibilite d'un `Claim` formule a propos de cet evenement.

## Entites

### `Ada`

- Role : entite decisionnaire.
- Capacite minimale : percevoir une demande de cooperation, consulter les claims accessibles, choisir une action.
- Decision cible : `cooperate` ou `refuse` face a `Borin`.

### `Borin`

- Role : partenaire potentiel.
- Capacite minimale : emettre ou etre associe a une demande d'aide.
- Particularite : cible du `Claim`.

### `Ciro`

- Role : auteur du `Claim`.
- Capacite minimale : produire un claim sur un evenement observe ou rapporte.
- Particularite : n'a pas besoin d'agir pendant la decision cible.

Deux entites seraient possibles si `Ada` formulait elle-meme le `Claim`, mais trois entites separant decisionnaire, cible et auteur rendent plus visible la distinction entre evenement, interpretation et croyance.

## Ressources ou objets necessaires

- `resource-r1` : ressource indivisible initialement promise a `Ada`.
- `task-t1` : demande ulterieure d'aide de `Borin` envers `Ada`.
- `event-001` : evenement anterieur reellement survenu.
- `record-001` : trace evenementielle structurante de `event-001`, disponible pour l'observateur.
- `claim-001` : interpretation formulee par `Ciro` a propos de `event-001`.
- `decision-001` : decision cible prise par `Ada`.

## Etat initial

Etat commun aux deux runs :

- seed : `E001-fixed-seed`
- ordre d'iteration : `Ada`, `Borin`, `Ciro`
- entites : `Ada`, `Borin`, `Ciro`
- relation initiale `Ada -> Borin` : neutre
- ressource `resource-r1` : existe
- promesse initiale : `resource-r1` est reservee a `Ada`
- evenement anterieur : `event-001` s'est produit dans les deux runs
- demande cible : `Borin` demande l'aide d'`Ada` pour `task-t1`

Aucune variable aleatoire ne doit intervenir dans la decision cible apres l'initialisation.

## Evenement anterieur

### Evenement reellement survenu dans le monde

`event-001` :

- acteur : `Borin`
- action : prend `resource-r1`
- contexte : `resource-r1` etait promise a `Ada`
- resultat : `Ada` ne recoit pas `resource-r1`

Cet evenement appartient au passe simule dans les deux runs.

### Trace technique eventuelle

La trace technique est le log interne produit par l'execution. Elle peut contenir l'ordre d'appel des regles, les transitions d'etat et les donnees de debug.

Elle n'est pas consultable par les entites et ne doit pas etre confondue avec la memoire du monde.

### `EventRecord` accessible dans le monde

`record-001` :

- type : `EventRecord`
- evenement source : `event-001`
- contenu : `Borin took resource-r1 after it had been promised to Ada.`
- statut : trace de monde, pas jugement
- accessibilite aux entites dans E001 : aucune consultation directe par `Ada` pendant la decision cible
- accessibilite observateur : oui

Le record sert a distinguer le fait passe de son interpretation. Dans E001, il ne doit pas influencer directement la decision cible.

## Definition exacte du Claim

`claim-001` est un artefact d'interpretation, attribue a `Ciro`, cible `Borin`, pointe vers `event-001` et `record-001`, et porte :

- `scope: cooperation` ;
- `polarity: caution_against_cooperation` ;
- `allegation: { kind: unkept_resource_promise, actor: Borin, affected: Ada, resource: resource-r1 }` ;
- `text: "Borin ignored a prior resource promise to Ada."` (affichage uniquement).

Le Claim n'embarque **aucune force experimentale**, poids numerique ni action finale. Les regles generales ci-dessous produisent les raisons et decident de leur priorite. Le texte libre ne participe pas aux tests. Les allegations sont des affirmations de l'auteur, **pas des verites verifiees** : Ada ne peut pas inspecter l'evenement technique source dans cette experience. La credibilite de Ciro, la verification factuelle et les croyances persistantes sont hors perimetre.

Les identifiants Ada, Borin, Ciro et claim-001 restent des fixtures interchangeables : aucune branche de decision ne doit en dependre.

## Regle d'accessibilite

Un Claim est accessible a une entite si et seulement si l'artefact existe, si l'entite appartient a son audience, et si aucun masque d'acces ne lui interdit la consultation.

**La portee semantique et la cible ne participent pas a l'accessibilite** : elles sont verifiees ensuite, a l'etape d'applicabilite.

- Run A : `claim-001` existe et est visible pour Ada.
- Run B : le **meme Claim** existe, mais est masque pour Ada.
- La visibilite pour Ada est **la seule variable independante** ; l'evenement passe, le Claim, le contexte decisionnel et toutes les regles sont identiques.

Le masque ne supprime pas le Claim et ne modifie ni les relations, ni la demande d'aide, ni l'etat des ressources.

## Regle de consultation

**Attention neutralisee pour E001**, conformement a C003. Il s'agit d'un controle experimental, pas d'un modele realiste d'attention ni d'un test de H006.

1. `search_performed = true` : enumerer deterministement les Claims accessibles, sans filtrage semantique.
2. `selection_policy = consult_all_accessible_claims` : selectionner tous ces Claims dans un ordre stable.
3. Enregistrer `selected_for_consultation`, puis `consulted_artifact_ids` uniquement pour les consultations effectivement executees.
4. Evaluer l'applicabilite **apres** la consultation : un Claim hors cible ou hors portee reste consultable et produit des checks negatifs.

Aucun `EventRecord` n'est consultable par Ada dans E001. La politique d'attention non triviale reste l'objet distinct de l'experience candidate E004.

## Mecanisme minimal d'interpretation

**Meme regle generale pour tous les runs et tous les controles**, fixee avant d'observer les resultats :

1. Si `help_cost == low` et `prior_relation == neutral`, creer la raison positive `baseline_cooperation_when_cost_low_and_relation_neutral` (action candidate `cooperate`, sans Claim).
2. Pour chaque Claim effectivement consulte, tracer separement les checks :
   - `target == decision.partner` ;
   - `scope == cooperation` ;
   - `polarity == caution_against_cooperation` ;
   - `allegation.kind == unkept_resource_promise` ;
   - `allegation.actor == decision.partner` ;
   - `allegation.affected == decision.actor`.
3. Si tous les checks passent, ajouter une raison candidate `reported_prior_promise_violation`, avec l'ID de l'artefact consulte, son auteur et les checks qui l'ont produite. La raison ne confirme **pas** la verite de l'allegation.
4. Dans ce contexte neutre de cout faible, une raison de prudence applicable prime sur la raison de cooperation de base. Sinon, la raison de base gagne. La meme politique de resolution doit s'appliquer a tous les runs.
5. `reported_prior_promise_violation` conduit a `refuse` ; la raison de base conduit a `cooperate`.

Cette priorite est une **hypothese de regle bornee**, non une theorie psychologique. L'experience mesure l'effet causal d'une consultation structuree sur la decision. Il est interdit de consulter le texte libre, ou de coder `Ada`, `Borin` ou `claim-001` comme conditions.

## Distinctions obligatoires

### Verite

Le monde peut conserver que `event-001` s'est produit. Cette verite de simulation n'est pas directement equivalente a ce que les entites savent ou croient.

### Memoire

La memoire contient des objets persistants consultables, comme `claim-001`. Elle n'est pas l'historique technique complet.

### Croyance

E001 ne modelise pas une croyance durable. `Ada` n'acquiert pas encore un etat mental persistant du type "Borin est peu fiable". Elle utilise seulement une raison locale pendant une decision.

### Influence

L'influence est la production d'une raison de decision a partir d'un claim consulte et applicable.

### Decision

La decision est le choix final entre `cooperate` et `refuse`.

### Trace technique

La trace technique aide le developpeur, mais ne constitue pas une preuve diegetique.

### Trace causale observateur

La trace causale observateur relie explicitement les champs suivants : claim accessible, claim consulte, regle d'interpretation appliquee, raison produite, action choisie.

## Decision cible

`decision-001` : Ada repond a une demande de cooperation de Borin pour `task-t1`. Actions possibles : `cooperate` ou `refuse`.

Les deux runs ont `help_cost = low`, `prior_relation = neutral`, et aucune raison concurrente de refus. **La cooperation est justifiee positivement** par `baseline_cooperation_when_cost_low_and_relation_neutral` ; elle n'est pas un choix par defaut arbitraire.

**Prediction a tester** : lorsque le Claim est consulte et applicable, la raison de prudence prime et le choix devient `refuse`. Ce n'est pas un resultat deja observe.

## Configuration du Run A

- Seed : `E001-fixed-seed`.
- Meme monde, meme evenement, meme EventRecord, meme Claim, memes acteurs, cout d'aide, relation et regles que dans B.
- Unique override : `claim-001` est accessible a Ada.
- Prediction (non resultat) : Ada consulte le Claim, produit une raison de prudence applicable et refuse.

```yaml
run: A
visibility_override: { actor: Ada, artifact: claim-001, visible: true }
predicted:
  search_performed: true
  accessible_artifact_ids: [claim-001]
  selection_policy: consult_all_accessible_claims
  selected_for_consultation: [claim-001]
  consulted_artifact_ids: [claim-001]
  applicability_checks:
    - { artifact_id: claim-001, target_matches: true, scope_matches: true, allegation_matches: true, applicable: true }
  selected_reason: { kind: reported_prior_promise_violation, artifact_id: claim-001 }
  chosen_action: refuse
```

## Configuration du Run B

- Etat identique a A, sauf que `claim-001` est masque pour Ada.
- Prediction (non resultat) : le Claim n'est pas consulte ; la raison positive de cooperation est retenue.

```yaml
run: B
visibility_override: { actor: Ada, artifact: claim-001, visible: false }
predicted:
  search_performed: true
  accessible_artifact_ids: []
  selection_policy: consult_all_accessible_claims
  selected_for_consultation: []
  consulted_artifact_ids: []
  applicability_checks: []
  selected_reason: { kind: baseline_cooperation_when_cost_low_and_relation_neutral, artifact_id: null }
  chosen_action: cooperate
```

Ces traces YAML sont des **oracles attendus**, jamais des observations d'execution.

## Variables controlees

Seed, entites et identifiants, evenement passe, EventRecord, contenu du Claim, contexte de decision, cout `low`, relation `neutral`, demande de cooperation, regles d'accessibilite hors override, consultation, applicabilite, resolution des raisons, ordre d'iteration et absence d'alea non controle.

Une future implementation doit comparer une empreinte canonique des etats initiaux **hors override de visibilite** et prouver qu'aucun autre champ ne varie. Toute mutation cachee invalide la comparaison.

## Variable independante

**Uniquement** la visibilite de `claim-001` pour Ada au moment de `decision-001` : visible dans A, masque dans B. Le Claim existe dans les deux mondes, sans changement de contenu. La variable ne teste pas l'attention ni la verite du Claim.

## Variables observees

- Empreinte canonique de l'etat hors override et preuve de l'unique difference.
- `search_performed`, `accessible_artifact_ids`, `selection_policy`, `selected_for_consultation`.
- `consulted_artifact_ids` et `applicability_checks` avec leurs echecs eventuels.
- `candidate_reasons` avec sources, `selected_reason` et `chosen_action`.
- References exactes aux Claims effectivement consultes et applicables.
- Divergence ou absence de divergence, controles negatifs, stabilite des repetitions.

## Trace causale attendue

La trace doit etre **accumulee pendant l'execution**, jamais fabriquee apres la decision a partir de l'action retenue. Chaque champ provient de l'etape qu'il decrit.

```text
initial_state_fingerprint_excluding_visibility
visibility_override
search_performed
accessible_artifact_ids
selection_policy
selected_for_consultation
consulted_artifact_ids
applicability_checks[]     # inclure checks echoues
candidate_reasons[]        # raison de base + raisons issues de Claims
selected_reason
chosen_action
```

Invariants : `consulted_artifact_ids ⊆ selected_for_consultation ⊆ accessible_artifact_ids` ; chaque raison issue d'un Claim reference un Claim consulte et applicable ; la raison retenue appartient aux raisons candidates ; l'action est deduite de la raison retenue selon la regle partagee ; aucun EventRecord ou fait technique n'est consulte par Ada.

## Criteres de succes

Le protocole renforce **localement** H001 sous attention neutralisee si et seulement si :

1. A et B sont identiques hormis la visibilite du Claim.
2. Les traces atomiques sont produites par l'execution, sont completes et satisfont les invariants.
3. Dans A, Ada consulte le Claim applicable, choisit la raison de prudence puis refuse.
4. Dans B, Ada ne consulte pas le Claim, choisit la raison positive de cooperation puis coopere.
5. Les **quatre controles negatifs** passent avec les memes regles.
6. Une permutation coherente des identifiants conserve les comportements structurels.
7. Memes donnees et seed donnent memes actions et traces canoniques.

La divergence seule ne suffit pas. Meme en cas de succes, E001 **ne demontre pas** emergence, valeur ludique, verite des interpretations, necessite de H006, ni generalite sociale.

## Criteres d'echec

Separer obligatoirement :

- **Invalide techniquement / methodologiquement** : runs non comparables, trace recreee apres coup, consultation d'un Claim inaccessible, Claim inapplicable cite comme cause, cas special code sur un identifiant, controles negatifs non respectes ou execution non reproductible. Dans ce cas, aucune conclusion sur H001.
- **Defavorable a H001 dans ce domaine borne** : protocole et execution valides, mais la consultation ne produit pas d'influence decisionnelle attribuable ou lisible.

Si le seul moyen d'obtenir la divergence consiste a inscrire la decision finale dans la fixture ou a ajuster la regle apres observation, verdict `NO-GO` pour la preuve causale.

## Resultats inconclusifs possibles

Resultat inconclusif si le Claim est consulte mais que l'applicabilite ou la resolution est ambigue, si un autre motif prioritaire annule toute difference, si les etapes causales sont confondues, ou si une variable autre que l'acces a change. Documenter le biais plutot que recalibrer la force du Claim.

## Risques de faux positif

- La regle pourrait etre calibree pour produire exactement l'exemple attendu.
- Le claim pourrait etre une variable `refuse_borin = true` maquillee.
- La decision par defaut pourrait etre trop artificielle.
- La trace pourrait seulement repeter l'intention de conception sans prouver que le moteur a suivi cette chaine.
- L'accessibilite pourrait modifier l'etat du monde avant la decision, introduisant une difference supplementaire.

## Risques de scenario implicitement scripte

Le protocole devient scripte si :

- le texte du claim encode directement l'action finale ;
- `Ada` a une regle speciale pour `claim-001` plutot qu'une regle generale pour claims de promesse violee ;
- `Borin` est traite par un cas particulier ;
- l'evenement anterieur est cree uniquement pour declencher une branche narrative unique ;
- la trace causale est ecrite a la main au lieu d'etre derivee des etapes de decision.

Mitigation minimale : la regle doit porter sur une structure generale `claim.polarity`, `claim.target`, `claim.scope`, `claim.event_reference`, pas sur l'identifiant `claim-001`.

## Limites de l'experience

E001 ne permet pas de conclure que :

- PALIMPSEST produit de l'emergence ;
- un systeme social complet est viable ;
- les claims sont credibles, contestables ou socialement riches ;
- les entites possedent des croyances realistes ;
- le mecanisme est ludique ;
- les artefacts peuvent devenir des normes ;
- la memoire fonctionne a grande echelle ;
- le joueur aura des interventions interessantes.

E001 teste seulement une causalite minimale entre accessibilite d'un claim et decision future lisible.

## Pseudo-code strictement minimal

```text
base_world = fixture_E001()                 # EventRecord et Claim toujours presents
for visible in [true, false]:
    world = copy(base_world)
    world.visibility.set(world.decision.actor, claim_id, visible)
    assert equal_except_visibility(base_world, world)
    trace = start_trace(world)               # les faits sont enregistres a chaque etape
    accessible = enumerate_accessible_claims(world, world.decision.actor)
    trace.search(performed=true, accessible_ids=ids(accessible))
    selected = sort_by_id(accessible)        # consultation triviale, aucune pertinence ici
    trace.select(policy="consult_all_accessible_claims", ids=ids(selected))
    reasons = [baseline_reason_if_low_cost_and_neutral_relation(world.decision)]
    trace.add_reason(reasons[0])
    for claim in selected:
        data = consult(claim)
        trace.consult(claim.id)
        checks = evaluate_applicability(data, world.decision)
        trace.check(claim.id, checks)         # y compris les tests echoues
        if checks.all_pass:
            reason = caution_reason(source=claim.id, rule="reported_prior_promise_violation")
            reasons.append(reason)
            trace.add_reason(reason)
    selected_reason = resolve_by_shared_priority(reasons)
    action = action_from_reason(selected_reason)
    trace.decide(selected_reason, action)
    emit(trace)                              # pas de justification reconstruite
run_four_negative_controls()
run_identifier_permutation()
```

Ce pseudo-code est indicatif, pas un appel a un moteur generique de regles. Chaque test doit interroger les champs structures, jamais les identifiants des fixtures ni les textes narratifs.

## Fixtures ou exemples de donnees candidats

```yaml
entities:
  - { id: Ada, role: decision_actor }
  - { id: Borin, role: cooperation_partner }
  - { id: Ciro, role: claim_author }
events:
  - id: event-001
    actor: Borin
    action: take_resource
    object: resource-r1
    prior_commitment: { resource: resource-r1, promised_to: Ada }
event_records:
  - { id: record-001, event_id: event-001, observer_only: true }
claims:
  - id: claim-001
    author: Ciro
    target: Borin
    event_reference: event-001
    record_reference: record-001
    scope: cooperation
    polarity: caution_against_cooperation
    allegation: { kind: unkept_resource_promise, actor: Borin, affected: Ada, resource: resource-r1 }
decision_context:
  id: decision-001
  actor: Ada
  partner: Borin
  task: task-t1
  options: [cooperate, refuse]
  help_cost: low
  prior_relation: neutral
visibility_override:
  run_A: { actor: Ada, artifact: claim-001, visible: true }
  run_B: { actor: Ada, artifact: claim-001, visible: false }
```

Les noms ne sont que des fixtures. Un Claim inexact resterait une interpretation consultable ; E001 ne teste pas la veracite de l'allegation.

## Tests necessaires lors d'une future implementation

1. Deux runs identiques hors masque du Claim ; empreintes canoniques comparees.
2. Memes entrees, seed et ordre d'iteration => meme action **et** meme trace.
3. Claim consulte uniquement dans A ; `record-001` non consulte par Ada.
4. Chaque raison issue d'un Claim vient d'une consultation et de checks reussis.
5. Controle positif A/B : raison prudente puis refus ; raison positive puis cooperation.
6. **Negatif 1, Claim hors cible** : target different du partenaire => cooperation, check `target_matches=false`.
7. **Negatif 2, Claim hors portee** : scope different de cooperation => consultation effective, check `scope_matches=false`, cooperation.
8. **Negatif 3, EventRecord seul** : record present mais Claim masque => cooperation, aucune influence du record.
9. **Negatif 4, mauvaise audience** : Claim accessible a Ciro mais pas Ada => aucune consultation d'Ada, cooperation.
10. Permuter acteurs, ressources et IDs d'artefacts en maintenant les references coherentes : comportements inchanges.
11. Modifier uniquement le texte libre du Claim ne modifie pas le comportement.
12. La trace est collectee en cours de decision, pas produite selon le seul `chosen_action`.

Tous les controles emploient les **memes regles** que le controle positif.

## Gate P0 - autorisation d'implementation

**Statut : protocole revise, non implemente et non teste.** Les critiques historiques C002 et C003 restent conservees. Cette version integre leurs corrections documentaires mais ne valide ni H001 ni une architecture.

Le reviewer doit confirmer : aucune force embarquee dans le Claim ; priorite de raison generale ; raison positive de cooperation ; masque d'acces unique ; separation acces / recherche / selection / consultation / applicabilite ; trace atomique ; controles negatifs et permutations ; hypothese H006 explicitement hors scope.

Verdicts : `GO` autorise seulement un test minimal ; `REVISE` si une ambiguite bloquante demeure ; `NO-GO` si la causalite ne peut etre eprouvee sans preprogrammer le resultat. Le choix Python initial versus Rust futur est une **decision distincte**.

## Questions restant ouvertes

- Une variante ulterieure doit-elle comparer "claim absent" et "claim present mais masque" apres le protocole de base ?
- Le `Claim` doit-il etre produit pendant le run ou precharge dans l'etat initial experimental ?
- Quel niveau de generalite la regle `prior_promise_violation_claim` doit-elle avoir pour eviter le scenario encode ?
- A quel moment une influence locale devient-elle une croyance persistante ?
- Quelle trace serait comprehensible par un joueur sans exposer tout le log technique ?
- Faut-il introduire une credibilite minimale de l'auteur dans une experience ulterieure ?
- Comment tester que le claim n'est pas seulement une variable ordinaire enveloppee dans un objet ?
- Quelle forme de revue humaine doit valider E001 avant implementation ?
