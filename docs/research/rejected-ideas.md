# Idees rejetees ou differees

## Statut

Ce registre conserve les idees a ne pas implementer maintenant. Un report n'est pas un rejet definitif.

## R001 - Modele de langage dans le coeur du prototype

- Formulation : utiliser un LLM pour generer dialogues, interpretations ou recits internes.
- Origine : exclu explicitement par `D001`, confirme par `C001`.
- Raison du rejet ou du report : opacite, non-determinisme pratique, difficulte d'attribution causale, risque de confondre generation narrative et emergence systemique.
- Caractere : report provisoire fort.
- Conditions de reouverture : seulement apres validation d'une boucle deterministe inspectable, et si le LLM reste hors du coeur causal ou strictement encapsule.

## R002 - Civilisation, politique, religion, factions complexes

- Formulation : etendre rapidement PALIMPSEST a une simulation sociale large.
- Origine : liste d'exclusions de `D001`, critique `C001`.
- Raison du rejet ou du report : ces domaines donneraient une profondeur apparente mais masqueraient les mecanismes fondamentaux.
- Caractere : report provisoire.
- Conditions de reouverture : lorsque des mecanismes simples de memoire, interpretation, contestation et norme auront produit des comportements observables.

## R003 - Moteur de regles generique

- Formulation : construire un systeme abstrait de regles configurables ou auto-modifiables.
- Origine : exclusions de `D001`, critique `C001`.
- Raison du rejet ou du report : complexite prematuree ; risque de construire une plateforme au lieu d'une experience testable.
- Caractere : report provisoire fort.
- Conditions de reouverture : friction observee dans l'ajout ou la comparaison de regles simples.

## R004 - Graphe de provenance executable

- Formulation : creer une base de donnees, une ontologie ou un outil automatique pour suivre la provenance de conception.
- Origine : `P001`.
- Raison du rejet ou du report : la provenance documentaire doit rester Markdown pour cette etape ; aucune friction d'automatisation n'est encore observee.
- Caractere : report provisoire.
- Conditions de reouverture : accumulation de frictions concretes dans la mise a jour manuelle.

## R005 - Interface graphique ou moteur de jeu des le depart

- Formulation : commencer par Unity, Godot, Unreal, une interface web complexe ou une visualisation riche.
- Origine : exclusions de `D001`, critique `C001`.
- Raison du rejet ou du report : le premier risque n'est pas visuel mais systemique ; une interface riche peut masquer une boucle causale faible.
- Caractere : report provisoire.
- Conditions de reouverture : chaine verticale validant que les artefacts influencent des decisions de maniere inspectable.

## R006 - Reflexivite complete des regles sociales

- Formulation : permettre rapidement a des artefacts internes de devenir des regles executables ou quasi executables.
- Origine : propriete recherchee dans `D001`, critique `C001`.
- Raison du rejet ou du report : concept prometteur mais disproportionne pour le premier prototype ; risque d'ambiguite entre norme, regle codee et decision.
- Caractere : report provisoire, a surveiller.
- Conditions de reouverture : lorsque `EventRecord`, `Claim` et `Norm` auront montre leurs limites sur une experience minimale.
