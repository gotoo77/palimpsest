# Evolution des concepts

## Statut

Ce document initialise le suivi des concepts centraux de PALIMPSEST. Les definitions courantes ci-dessous sont des formulations de travail, pas des decisions finales.

## Nature du projet

- Formulation initiale : PALIMPSEST est un jeu de simulation systemique fonde sur une memoire persistante d'artefacts.
- Critiques recues : la trajectoire documentaire a deplace le centre de gravite vers des hypotheses, protocoles et revues experimentales avant toute boucle de jeu.
- Reformulation `P006` : PALIMPSEST est actuellement un laboratoire sur la memoire dont un jeu pourrait emerger.
- Definition courante : projet de recherche et de conception visant a comprendre comment une memoire persistante peut influencer un systeme, tout en maintenant ouverte la question de son interet ludique.
- Questions ouvertes : a quel moment un protocole experimental devient-il une boucle jouable ? Quels criteres distinguent un bon outil de recherche d'un bon jeu ?

## Artefact

- Formulation initiale : objet persistant issu d'une interaction passee, identifiable, inspectable, attribuable, relie, transformable, interpretable, contestable et potentiellement oublie.
- Critiques recues : le concept couvre a la fois trace, information, contrainte, institution, recit et regle sociale. Cette amplitude risque de rendre l'objet trop general pour etre manipulable.
- Reformulation de travail : un artefact est une trace structuree du monde qui peut etre consultee par des entites et influencer des decisions futures selon des regles explicites.
- Definition courante : objet persistant de domaine, distinct du log technique, dont l'influence future doit etre observable et referencee.
- Clarification `E001` : un artefact influent doit passer par accessibilite, consultation, applicabilite et reference dans la decision ; sinon il risque de n'etre qu'une variable decoree.
- Questions ouvertes : quels types minimaux suffisent ? Un artefact doit-il toujours avoir un auteur ? Comment mesurer son influence reelle ?

## Claim

- Formulation initiale : artefact interpretatif formule a propos d'un evenement ou d'une entite.
- Critiques recues : un `Claim` negatif peut facilement devenir un simple modificateur de confiance masque.
- Reformulation `E001` : un `Claim` est une interpretation persistante, attribuee a un auteur, ciblant une entite ou un evenement, associee a une portee d'application, et consultable seulement sous conditions d'accessibilite explicites.
- Definition courante : artefact interpretatif qui ne se confond ni avec l'evenement survenu, ni avec son `EventRecord`, ni avec une croyance durable de l'entite qui le consulte.
- Questions ouvertes : faut-il modeler la credibilite de l'auteur des le prototype ? Comment verifier qu'un claim n'est pas seulement une variable ordinaire enveloppee dans un objet ?

## Memoire

- Formulation initiale : ensemble d'artefacts persistants produits par le systeme et reinjectes dans les etats futurs.
- Critiques recues : la memoire est parfois decrite comme stockage, parfois comme contrainte, parfois comme moteur causal. Ces roles doivent rester distingues.
- Reformulation de travail : la memoire est l'ensemble des artefacts accessibles et des regles d'acces qui peuvent modifier la perception ou la decision des entites.
- Definition courante : couche persistante et interpretable du monde, non equivalente a l'historique complet de simulation.
- Clarification `H006` : une memoire accessible n'est pas encore une memoire consultee. PALIMPSEST doit distinguer ce qui existe, ce qui est accessible, ce qui est recherche, ce qui est selectionne, ce qui est consulte et ce qui est finalement interprete.
- Questions ouvertes : quelle part de la memoire doit rester locale ? L'oubli doit-il supprimer, masquer, affaiblir ou transformer ? Une politique de consultation triviale suffit-elle au prototype ?

## Regle

- Formulation initiale : petite, explicite, testable, deterministe autant que possible, tracable lors de son application.
- Critiques recues : le document hesite entre regles de simulation codees et normes internes representees par des artefacts.
- Reformulation de travail : une regle prototype est du code deterministe ; une norme est un artefact pouvant influencer certaines regles, sans devenir immediatement du code executable.
- Definition courante : mecanisme explicite qui transforme un etat en consequence observable.
- Questions ouvertes : a quel moment une norme interne peut-elle etre traitee comme regle sociale active ? Faut-il un langage de regles ou des types fermes suffisent-ils ?

## Emergence

- Formulation initiale : structures non explicitement programmees produites par memoire locale, regles simples et reinjection.
- Critiques recues : une chaine attendue trop specifique peut devenir un scenario masque. La presence de complexite ne prouve pas l'emergence.
- Reformulation de travail : parler d'emergence seulement lorsqu'un comportement global apparait par composition de regles locales et reste reproductible, explicable apres coup, mais non encode comme sequence narrative.
- Definition courante : propriete observee a demontrer experimentalement, pas promesse de design.
- Questions ouvertes : quels criteres distinguent emergence faible, variation scenarisee et simple bruit ?

## Role du joueur

- Formulation initiale : gardien-intervenant qui observe, inspecte, introduit, modifie, relie, retire ou masque des artefacts, sans controler directement les entites.
- Critiques recues : le role risque d'etre puissant mais peu ludique si les interventions ne produisent que des debugs de simulation. Il peut aussi devenir un editeur omnipotent.
- Reformulation de travail : le joueur formule des hypotheses causales sur la memoire du monde et teste des interventions limitees.
- Definition courante : observateur actif dont le levier principal est l'accessibilite et la forme de la memoire.
- Clarification `H006` : le joueur pourrait observer non seulement quels artefacts existent, mais comment les entites developpent ou appliquent des strategies de consultation de leur propre memoire. Cette piste reste une hypothese, pas un role valide.
- Garde-fou `P006` : un dispositif experimental lisible ne suffit pas a definir une experience ludique. Le role du joueur doit etre evalue par son interet propre, pas seulement par sa valeur d'inspection.
- Questions ouvertes : le joueur est-il hors du monde ou lui-meme represente par des artefacts ? Qu'est-ce qu'une intervention interessante mais non arbitraire ? Le jeu reside-t-il davantage dans l'accessibilite des artefacts ou dans l'attention que les entites leur accordent ?

## Recherche, selection et consultation

- Formulation initiale implicite : une entite consulte les artefacts accessibles et pertinents.
- Critiques recues : `C002` signale que consultation et accessibilite sont trop proches dans `E001`. `P005` formule le risque plus largement : consulter tous les artefacts accessibles transforme la memoire en information totale toujours prise en compte.
- Reformulation de travail : la consultation devrait peut-etre etre precedee par une recherche et une selection. La recherche identifie les artefacts susceptibles d'aider une decision ; la selection choisit ceux qui seront effectivement consultes ; la consultation rend leur contenu disponible a l'interpretation.
- Definition courante : ensemble de processus attentionnels possibles entre memoire accessible et interpretation, encore non valide comme mecanisme necessaire.
- Clarification `C003` : si l'attention est neutralisee, le protocole doit le dire. Sinon, accessibilite, recherche, selection, consultation et applicabilite doivent etre tracees comme etapes distinctes.
- Hypothese liee : `H006`.
- Questions ouvertes : une politique deterministe simple suffit-elle ? Faut-il modeler confiance, cout de recherche, ordre de consultation, fatigue, priorite d'auteur ou erreur ? Comment eviter de remplacer la memoire par une IA de scoring classique ?

## Interpretation

- Formulation initiale : les entites lisent et interpretent differemment les artefacts.
- Critiques recues : le terme peut cacher une complexite cognitive excessive. Sans modele minimal, il devient un mot-valise.
- Reformulation de travail : l'interpretation est une fonction simple qui convertit un artefact accessible en modificateur explicite de decision.
- Definition courante : transformation bornee, tracable et testable d'un artefact en influence comportementale.
- Clarification `E001` : l'interpretation produit une raison de decision referencee, pas une croyance persistante ni une modification globale de confiance.
- Clarification `H006` : l'interpretation ne doit pas absorber la question de l'attention. Avant d'interpreter un artefact, une entite doit l'avoir trouve ou selectionne pour consultation.
- Questions ouvertes : faut-il modeler la verite, la croyance et la confiance separement ? Quelle deformation minimale est lisible par le joueur ?

## Norme et institution

- Formulation initiale : une succession d'evenements et d'interpretations peut produire une norme persistante ou une institution rudimentaire.
- Critiques recues : ces termes sont lourds. Les utiliser trop tot peut faire croire a une simulation sociale plus riche qu'elle ne l'est.
- Reformulation de travail : une norme prototype est un artefact influent propose ou soutenu par plusieurs entites, qui modifie localement des decisions.
- Definition courante : artefact social stabilise, pas institution complete.
- Questions ouvertes : combien de soutien, de consultation ou de duree faut-il avant qu'une norme soit active ? Une norme existe-t-elle sans entites qui y accordent du poids ?

## Oubli

- Formulation initiale : un artefact inaccessible, non consulte ou trop ancien peut perdre son influence.
- Critiques recues : l'oubli peut signifier disparition, inaccessibilite, perte de poids, corruption ou remplacement. Ces options n'ont pas les memes consequences.
- Reformulation de travail : commencer par l'inaccessibilite et l'affaiblissement d'influence, sans destruction silencieuse.
- Definition courante : diminution ou modification tracable de la capacite d'un artefact a influencer le futur.
- Questions ouvertes : l'oubli doit-il etre passif, actif ou strategique ? Le joueur doit-il pouvoir distinguer oubli et censure ?

## Dependance au chemin

- Formulation initiale : deux simulations presque identiques peuvent diverger a cause d'un artefact produit plus tot.
- Critiques recues : la divergence seule ne suffit pas ; il faut une chaine causale inspectable et attribuable.
- Reformulation de travail : dependance au chemin signifie qu'une difference anterieure documentee dans la memoire explique une difference posterieure de comportement.
- Definition courante : divergence reproductible, reliee a un artefact ou a son absence.
- Clarification `E001` : pour attribuer la divergence a la memoire, les deux runs doivent controler l'evenement source, l'etat initial, la seed, les regles, l'ordre d'iteration et la decision cible ; la variable independante est seulement l'accessibilite du `Claim`.
- Questions ouvertes : quels indicateurs rendent cette divergence visible sans transformer le jeu en outil d'analyse uniquement ?

## Discipline de conception

- Formulation initiale : conserver les prompts, critiques, hypotheses, experiences, decisions futures et idees differees afin de ne pas transformer trop vite les intuitions en architecture.
- Observation `P006` : la methode documentaire commence a agir comme un garde-fou du projet lui-meme. Les idees sont formulees, critiquees, reliees, retrodocumentees et conservees avant d'etre implementees.
- Clarification `P008` : l'implementation devient une issue possible d'une chaine de raisonnement, pas le reflexe suivant une idee.
- Definition courante : culture de conception qui exige qu'une idee brillante devienne d'abord hypothese, experience candidate, revue, analyse d'impact ou piste differee avant de devenir architecture.
- Regle `C003` : chaque nouvelle hypothese importante doit demander quels protocoles existants deviennent incomplets si cette hypothese est vraie.
- Cadrage `D010` : la methode est documentee comme potentiellement generale, mais ne doit pas etre extraite en projet autonome sans besoin ou friction documente.
- Risque : la discipline peut produire un excellent objet de recherche sans jamais converger vers une experience jouable.
- Questions ouvertes : quels seuils autorisent le passage d'une hypothese a une experience, puis d'une experience a une implementation ? Comment conserver la lenteur critique sans bloquer indefiniment le projet ?
