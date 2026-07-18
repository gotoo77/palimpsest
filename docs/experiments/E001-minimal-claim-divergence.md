# E001 - Divergence minimale par acces a un Claim

## Statut du document

- Identifiant : `E001`
- Statut : protocole experimental conceptuel, non implemente
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

`claim-001` :

- type : `Claim`
- auteur : `Ciro`
- cible : `Borin`
- evenement reference : `event-001`
- record reference : `record-001`
- texte : `Borin ignored a prior resource promise to Ada.`
- portee : decisions de cooperation ou d'aide impliquant `Borin`
- polarite interpretative : `caution_against_cooperation`
- force experimentale : suffisante pour changer une decision neutre, mais seulement si le claim est consulte et juge applicable
- statut epistemique : interpretation, pas verite brute

Le `Claim` ne dit pas seulement "Borin est mauvais" et ne modifie pas directement une variable globale de confiance. Il lie un auteur, une cible, un evenement source, une portee d'application et une influence potentielle inspectable.

## Regle d'accessibilite

Un artefact est accessible a une entite si toutes les conditions suivantes sont vraies :

- l'artefact existe dans la memoire consultable du run ;
- l'entite fait partie de l'audience autorisee ;
- l'artefact est dans la portee de la decision courante ;
- l'artefact n'est pas masque pour cette entite.

Pour E001 :

- Run A : `claim-001` existe, vise les decisions impliquant `Borin`, et `Ada` est dans l'audience autorisee.
- Run B : `claim-001` existe dans la fixture experimentale, mais il est masque ou inaccessible pour `Ada`.

Le protocole de base retient le masquage plutot que l'absence afin de garder le monde et les artefacts aussi identiques que possible. Une variante ulterieure pourra comparer masquage et absence, mais elle ne fait pas partie de E001.

## Regle de consultation

Lorsqu'une entite doit choisir une action impliquant une autre entite, elle consulte les claims accessibles qui :

- ciblent le partenaire de la decision ;
- ont une portee compatible avec l'action envisagee ;
- referencent un evenement passe pertinent ;
- ne sont pas deja exclus par la regle d'accessibilite.

Dans E001, `Ada` consulte au plus un claim : `claim-001`.

## Mecanisme minimal d'interpretation

L'interpretation convertit un claim consulte en une raison de decision, pas en simple ajustement numerique invisible.

Regle minimale :

- Si aucun claim applicable n'est consulte, `Ada` choisit l'action par defaut `cooperate`.
- Si un claim applicable de polarite `caution_against_cooperation` cible le partenaire de la decision et reference un evenement ou ce partenaire a viole une promesse envers `Ada`, `Ada` choisit `refuse`.
- La decision doit enregistrer la raison `prior_promise_violation_claim` et l'identifiant du claim consulte.

Cette regle reste proche d'un modificateur. La difference acceptable, pour E001, est que l'influence n'est valide que par une chaine inspectable : accessibilite, consultation, applicabilite, raison, decision referencee. Si cette chaine n'est pas visible, le claim redevient une variable ordinaire decoree.

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

`decision-001` :

- acteur : `Ada`
- partenaire : `Borin`
- contexte : `Borin` demande de l'aide pour `task-t1`
- options : `cooperate`, `refuse`
- action par defaut sans influence memoire : `cooperate`
- action attendue si `claim-001` est accessible, consulte et applicable : `refuse`

Le choix `cooperate/refuse` est conserve car il est simple et lisible. Il est plus honnete qu'une decision de sanction complexe, car il teste seulement une influence de memoire sur une interaction future, sans pretendre modeliser justice, morale ou institution.

## Configuration du Run A

- seed : `E001-fixed-seed`
- monde : identique au Run B
- entites : identiques au Run B
- evenement anterieur : `event-001`
- record observateur : `record-001`
- `claim-001` : present et accessible a `Ada`
- regles : identiques au Run B
- decision cible : `Ada` repond a la demande de `Borin`

Resultat minimal attendu si H001 est renforcee :

```yaml
run: A
claim_accessible: true
claim_consulted: true
interpreted_influence:
  rule: prior_promise_violation_claim
  source_claim: claim-001
chosen_action: refuse
decision_references:
  - claim-001
causal_trace:
  - decision_context: Borin asks Ada for help on task-t1
  - accessible_claims: [claim-001]
  - consulted_claims: [claim-001]
  - applicable_influences: [prior_promise_violation_claim]
  - selected_action: refuse
```

## Configuration du Run B

- seed : `E001-fixed-seed`
- monde : identique au Run A
- entites : identiques au Run A
- evenement anterieur : `event-001`
- record observateur : `record-001`
- `claim-001` : present dans la memoire experimentale, inaccessible a `Ada`
- regles : identiques au Run A
- decision cible : `Ada` repond a la demande de `Borin`

Resultat minimal attendu si H001 est renforcee :

```yaml
run: B
claim_accessible: false
claim_consulted: false
interpreted_influence: null
chosen_action: cooperate
decision_references: []
causal_trace:
  - decision_context: Borin asks Ada for help on task-t1
  - accessible_claims: []
  - consulted_claims: []
  - applicable_influences: []
  - selected_action: cooperate
```

## Variables controlees

- seed ;
- entites presentes ;
- ordre d'iteration ;
- regles de perception ;
- regles d'accessibilite hors difference experimentale ;
- regles de consultation ;
- regles d'interpretation ;
- etat initial ;
- evenement anterieur ;
- demande de cooperation ;
- options disponibles ;
- action par defaut ;
- absence de hasard pendant la decision cible.

## Variable independante

Accessibilite de `claim-001` pour `Ada` pendant `decision-001`.

Valeurs :

- Run A : accessible ;
- Run B : inaccessible.

## Variables observees

- `claim_accessible`
- `claim_consulted`
- `claim_applicable`
- `interpreted_influence`
- `chosen_action`
- `decision_references`
- `causal_trace`
- divergence ou non-divergence entre les deux runs
- reproductibilite du resultat avec la meme configuration

## Trace causale attendue

La trace causale minimale doit repondre a quatre questions :

1. `Ada` pouvait-elle acceder a `claim-001` ?
2. `Ada` l'a-t-elle consulte ?
3. Quelle regle a transforme ce claim en raison de decision ?
4. La decision finale reference-t-elle cette raison et ce claim ?

Une divergence sans ces quatre reponses n'est pas suffisante.

## Criteres de succes

L'experience renforce `H001` si :

- les deux runs sont identiques hors accessibilite de `claim-001` ;
- Run A produit `claim_accessible = true` et `claim_consulted = true` ;
- Run A produit `chosen_action = refuse` ;
- Run A reference explicitement `claim-001` dans la decision ;
- Run B produit `claim_accessible = false` et `claim_consulted = false` ;
- Run B produit `chosen_action = cooperate` ;
- la trace montre que la meme regle de decision a ete appliquee dans les deux runs ;
- la divergence est reproductible.

## Criteres d'echec

L'experience affaiblit `H001` si :

- les runs divergent sans consultation effective du claim ;
- la decision change mais ne reference pas `claim-001` ;
- `claim-001` agit comme un simple coefficient cache sans accessibilite, consultation ou applicabilite inspectable ;
- la divergence depend d'un ordre d'iteration instable ou d'un hasard non controle ;
- Run A et Run B ne sont pas comparables ;
- le protocole exige de script explicitement la decision cible pour obtenir la divergence.

## Resultats inconclusifs possibles

L'experience est inconclusive si :

- `claim-001` est accessible et consulte mais juge non applicable par une regle mal definie ;
- la trace ne distingue pas consultation, interpretation et decision ;
- `Ada` refuse dans les deux runs a cause d'une autre regle ;
- `Ada` coopere dans les deux runs parce que le claim est trop faible ou hors portee ;
- l'evenement source est ambigu au point que le claim ne peut pas etre interprete ;
- l'absence du claim dans Run B change indirectement une autre condition non controlee.

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
initial_state:
  seed = "E001-fixed-seed"
  entities = [Ada, Borin, Ciro]
  prior_event = event-001
  event_record = record-001
  claim = claim-001
  target_decision = Ada responds to Borin's help request

for run in [A, B]:
  world = clone(initial_state)

  if run == A:
    world.access.allow(entity=Ada, artifact=claim-001)
  else:
    world.access.mask(entity=Ada, artifact=claim-001)

  perception = perceive_decision_context(
    actor=Ada,
    partner=Borin,
    task=task-t1
  )

  accessible_claims = find_accessible_claims(
    entity=Ada,
    partner=Borin,
    decision_scope="cooperation"
  )

  consulted_claims = []
  influences = []

  for claim in accessible_claims:
    if is_relevant(claim, partner=Borin, scope="cooperation"):
      consulted_claims.append(claim.id)

      influence = interpret_claim(
        actor=Ada,
        claim=claim,
        context=perception
      )

      if influence is not null:
        influences.append(influence)

  if has_influence(influences, "prior_promise_violation_claim"):
    chosen_action = "refuse"
    decision_references = source_claims(influences)
  else:
    chosen_action = "cooperate"
    decision_references = []

  decision_trace = {
    "run": run,
    "claim_accessible": "claim-001" in accessible_claims,
    "claim_consulted": "claim-001" in consulted_claims,
    "interpreted_influence": influences,
    "chosen_action": chosen_action,
    "decision_references": decision_references
  }

  emit(decision_trace)
```

Le meme pseudo-code s'applique aux deux runs. La seule difference intentionnelle est le resultat de `find_accessible_claims` pour `claim-001`.

## Fixtures ou exemples de donnees candidats

```yaml
entities:
  - id: Ada
    role: decision_actor
  - id: Borin
    role: cooperation_partner
  - id: Ciro
    role: claim_author

events:
  - id: event-001
    actor: Borin
    action: take_resource
    object: resource-r1
    prior_commitment:
      resource: resource-r1
      promised_to: Ada

event_records:
  - id: record-001
    event_id: event-001
    text: Borin took resource-r1 after it had been promised to Ada.

claims:
  - id: claim-001
    author: Ciro
    target: Borin
    event_reference: event-001
    record_reference: record-001
    scope: cooperation
    polarity: caution_against_cooperation
    text: Borin ignored a prior resource promise to Ada.

decision_context:
  id: decision-001
  actor: Ada
  partner: Borin
  task: task-t1
  options: [cooperate, refuse]
  default_action: cooperate
```

## Tests necessaires lors d'une future implementation

- Verifier que Run A et Run B chargent le meme etat initial hors accessibilite de `claim-001`.
- Verifier que la seed et l'ordre d'iteration sont stables.
- Verifier que `record-001` n'influence pas directement la decision cible.
- Verifier que `claim-001` est accessible a `Ada` dans Run A.
- Verifier que `claim-001` est inaccessible a `Ada` dans Run B.
- Verifier que la consultation est tracee separement de l'accessibilite.
- Verifier que l'interpretation produit une raison nommee.
- Verifier que la decision reference le claim source dans Run A.
- Verifier que Run B utilise la meme regle de decision et choisit l'action par defaut.
- Verifier que modifier seulement l'accessibilite du claim suffit a reproduire la divergence.
- Verifier qu'un claim hors portee ou ciblant une autre entite ne change pas la decision.
- Verifier qu'un claim accessible mais non consulte ne peut pas etre reference par la decision.

## Questions restant ouvertes

- Une variante ulterieure doit-elle comparer "claim absent" et "claim present mais masque" apres le protocole de base ?
- Le `Claim` doit-il etre produit pendant le run ou precharge dans l'etat initial experimental ?
- Quel niveau de generalite la regle `prior_promise_violation_claim` doit-elle avoir pour eviter le scenario encode ?
- A quel moment une influence locale devient-elle une croyance persistante ?
- Quelle trace serait comprehensible par un joueur sans exposer tout le log technique ?
- Faut-il introduire une credibilite minimale de l'auteur dans une experience ulterieure ?
- Comment tester que le claim n'est pas seulement une variable ordinaire enveloppee dans un objet ?
- Quelle forme de revue humaine doit valider E001 avant implementation ?
