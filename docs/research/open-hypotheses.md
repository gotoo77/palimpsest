# Hypotheses ouvertes

## H001 - Un artefact consulte peut modifier une decision future de maniere lisible

- Origine : `D001`, critique `C001`
- Motivation : verifier le coeur de PALIMPSEST sans simuler une societe complete.
- Experience candidate : `E001`, deux runs identiques sauf acces a un `Claim` sur une entite qui a pris une ressource.
- Criteres susceptibles de renforcer l'hypothese : une entite refuse ou accepte une interaction differemment, et la trace reference explicitement l'artefact consulte.
- Criteres susceptibles de l'affaiblir : le joueur ne peut pas distinguer l'influence de l'artefact d'un hasard ou d'une regle cachee.
- Statut actuel : E001 amende documentairement le 2026-11-18 sous politique de consultation triviale ; revue de gate humaine requise avant une implementation, non teste.

## H002 - Trois types d'artefacts suffisent pour tester la boucle centrale

- Origine : `D001`, critique `C001`
- Motivation : limiter le modele initial a `EventRecord`, `Claim` et `Norm`.
- Experience candidate : `E002`, produire une chaine event -> claim -> modification de confiance -> refus -> proposition de norm.
- Criteres susceptibles de renforcer l'hypothese : la chaine est expressive, inspectable et ne demande pas d'autres types d'artefacts.
- Criteres susceptibles de l'affaiblir : les types deviennent des fourre-tout ou obligent a encoder des cas particuliers.
- Statut actuel : plausible, non validee.

## H003 - Le potentiel ludique principal reside dans l'enquete causale

- Origine : `C001`, precisee par `P006`
- Motivation : le joueur ne controle pas directement les entites ; l'interet doit venir de la lecture des consequences et de l'intervention sur la memoire.
- Experience candidate : proposer deux interventions memorielles concurrentes et demander au joueur d'anticiper laquelle modifiera le comportement collectif.
- Criteres susceptibles de renforcer l'hypothese : le joueur formule des hypotheses, observe des surprises comprehensibles et ajuste ses interventions.
- Criteres susceptibles de l'affaiblir : l'experience ressemble a du debogage de simulation, les effets sont trop opaques, ou le dispositif devient un bon laboratoire sans produire d'activite interessante pour le joueur.
- Garde-fou `P006` : la question scientifique "comment une memoire persistante influence-t-elle un systeme ?" ne repond pas automatiquement a la question ludique "pourquoi est-ce interessant pour le joueur ?".
- Statut actuel : hypothese ouverte, non testee, a maintenir en parallele des hypotheses de recherche systemique.

## H004 - L'oubli par baisse d'influence est plus utile au prototype que la suppression

- Origine : `C001`
- Motivation : supprimer les artefacts detruit la provenance et reduit la capacite d'inspection.
- Experience candidate : comparer suppression, masquage et affaiblissement progressif sur la lisibilite des consequences.
- Criteres susceptibles de renforcer l'hypothese : l'affaiblissement conserve la trace tout en produisant des effets visibles.
- Criteres susceptibles de l'affaiblir : le joueur ne percoit pas la difference avec une simple variable numerique.
- Statut actuel : non testee.

## H005 - La reflexivite doit rester hors du premier prototype

- Origine : `D001`, critique `C001`
- Motivation : representer des regles sociales comme artefacts internes est prometteur mais risque de transformer le prototype en programme de recherche trop vaste.
- Experience candidate : aucune pour le premier incremente ; revisiter apres validation d'une chaine causale simple.
- Criteres susceptibles de renforcer l'hypothese : le prototype minimal fournit deja une profondeur suffisante sans regles auto-modifiables.
- Criteres susceptibles de l'affaiblir : les artefacts restent decoratifs tant qu'ils ne peuvent pas devenir de vraies contraintes sociales.
- Statut actuel : hypothese de cadrage, non decision finale.

## H006 - L'economie de l'attention est un moteur central de divergence

- Origine : `P005`, a partir de la revue de la trajectoire `Event -> EventRecord -> Claim -> Decision -> Trace` et du finding `F003` de `C002`.
- Motivation : si une entite consulte tous les artefacts accessibles, la memoire risque de devenir une information totale toujours appliquee ; si elle consulte seulement l'artefact au meilleur score, le systeme risque de redevenir une IA decisionnelle classique. PALIMPSEST pourrait avoir besoin de politiques de recherche, selection et consultation pour que la memoire reste vivante, partielle et differenciante.
- Hypothese : deux mondes disposant des memes artefacts peuvent diverger de maniere interessante parce que leurs entites ne recherchent, ne selectionnent et ne consultent pas les memes artefacts.
- Experience candidate : `E004`, comparer des runs avec memes evenements, memes artefacts et memes regles de decision, mais politiques de consultation differentes.
- Criteres susceptibles de renforcer l'hypothese : des politiques de consultation differentes produisent des decisions divergentes, des traces causales lisibles et des trajectoires reproductibles sans changer les artefacts disponibles.
- Criteres susceptibles de l'affaiblir : une consultation triviale de tous les artefacts accessibles produit deja des comportements aussi lisibles, interessants et suffisants que des politiques d'attention plus riches.
- Resultats inconclusifs possibles : la divergence apparait mais reste illisible ; les politiques de consultation se reduisent a des scores caches ; la variation observee vient d'un artefact ou d'une regle de decision plutot que de la politique d'attention.
- Impact documente : `C003`, `docs/research/H006-impact-on-E001.md`, conclut que `H006` rend incompletes certaines hypotheses implicites de `E001` sans l'annuler.
- Statut actuel : hypothese de recherche nouvelle, non testee, aucune decision d'architecture.

## Experiences candidates

### E001 - Divergence minimale par acces a un Claim

- Hypothese cible : `H001`
- Document : `docs/experiments/E001-minimal-claim-divergence.md`
- Revue critique : `C002`, `docs/research/E001-critical-review.md`
- Analyse d'impact : `C003`, `docs/research/H006-impact-on-E001.md`
- Statut : protocole conceptuel initial du 2026-07-18 ; amendement P0 propose le 2026-11-18, non implemente et non teste ; gate de validation humaine toujours ouverte.
- Configuration : deux simulations avec meme seed, memes entites, memes ressources, meme evenement anterieur et memes regles.
- Variable changee : dans une branche, `claim-001` est accessible a `Ada` ; dans l'autre, il existe mais reste masque ou inaccessible pour `Ada`.
- Observation attendue : `Ada` choisit differemment face a une demande de cooperation de `Borin`, et la trace indique accessibilite, consultation, interpretation, decision et references causales.
- Limite : cette experience ne prouve pas l'emergence ; elle teste seulement la causalite memoire -> decision.
- Requalification explicite : E001 teste H001 sous `consult_all_accessible_claims`, et ne teste pas H006 ; la portee semantique ne determine pas l'acces.
- Corrections documentaires P0 : aucune force decisionnelle embarquee dans le Claim, raison positive de cooperation, etapes causales separees, trace atomique, quatre controles negatifs et permutations d'identifiants. Revue independante encore requise avant P1.

### E002 - Chaine EventRecord -> Claim -> Norm

- Hypothese cible : `H002`
- Configuration : plusieurs interactions autour d'une ressource limitee.
- Variable observee : emergence ou non d'une `Norm` a partir de traces et contestations.
- Observation attendue : la `Norm` apparait par conditions generiques, pas par script narratif.
- Limite : si les seuils sont trop specifiques, l'experience devient un scenario encode.

### E003 - Lisibilite de l'oubli

- Hypothese cible : `H004`
- Configuration : artefacts anciens, non consultes ou masques dans des runs comparables.
- Variable changee : suppression, masquage, affaiblissement.
- Observation attendue : identifier quel mecanisme reste comprehensible et utile au joueur.
- Limite : cette experience dependra fortement des outils d'inspection disponibles.

### E004 - Divergence par politique de consultation

- Hypothese cible : `H006`
- Statut : experience candidate non formalisee.
- Configuration : plusieurs runs avec memes entites, memes evenements, memes `EventRecord`, memes `Claim`, memes regles de decision et meme seed.
- Variable changee : politique de recherche, selection ou consultation des artefacts.
- Observation attendue : deux entites ou deux societes possedant la meme memoire disponible prennent des decisions differentes parce qu'elles ne consultent pas les memes artefacts, ou ne les consultent pas dans le meme ordre.
- Limite : cette experience ne doit pas remplacer `E001`; elle doit tester directement `H006` apres clarification de la portee de `E001`.
