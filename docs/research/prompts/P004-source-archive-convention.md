# P004 - Corriger l'autonomie documentaire des sources primaires

## Statut de source

- Identifiant : `P004`
- Date : 2026-07-18
- Statut : texte source conserve depuis le contexte de session
- Objet : corriger la convention documentaire pour archiver les prompts structurants et les rapports de run comme sources primaires autonomes.

## Texte du prompt

moi : un doute m assaille : pas sur qu il trace et documente ce qu il fait comme dan sla demande initiale du promp ayant decmlenche le projet

gpt : 
Ton doute est justifié.

Il trace bien l’évolution du projet, mais pas encore avec le niveau d’autonomie documentaire que suggérait la demande initiale.

Ce qu’il fait correctement :

P003 est enregistré dans prompt-history.md ;
C002 est enregistré dans artifact-provenance.md ;
la filiation E001 → P003 → C002 apparaît dans le graphe ;
la chronologie contient l’événement, le résultat et la conséquence ;
les hypothèses concernées sont mises à jour ;
le statut de E001 est corrigé : formalisé, revu, corrections requises avant implémentation.

Donc la généalogie intellectuelle est bien présente.

Le problème est cette formulation :

Chemin : prompt de session Codex du 2026-07-18
Statut : source conservée par référence sessionnelle

Cela signifie que le dépôt conserve une description synthétique du prompt, mais pas nécessairement son texte intégral. Or une « référence sessionnelle » dépend de l’interface Codex et risque de devenir inaccessible, invisible ou ambiguë plus tard.

Le dépôt sait actuellement dire :

P003 a demandé une attaque méthodologique de E001

Mais il ne permet probablement pas de relire exactement :

voici le prompt complet P003, tel qu’il a été soumis

Même limite pour les comptes rendus de Codex : leurs conclusions sont intégrées aux registres, mais la réponse finale originale n’est pas nécessairement archivée comme artefact distinct.

La situation réelle est donc :

provenance structurée       oui
chronologie                 oui
filiation                   oui
synthèse des prompts        oui
texte intégral des prompts  non garanti
rapport original de Codex   non garanti
autonomie hors session      partielle

Je corrigerais cela maintenant, avant que davantage d’itérations s’accumulent. Le principe devrait devenir :

Tout prompt structurant doit exister comme fichier autonome dans le dépôt, et prompt-history.md doit le référencer plutôt que le remplacer.

Par exemple :

docs/research/
├── prompts/
│   ├── P001-initialize-design-review.md
│   ├── P002-formalize-E001.md
│   └── P003-review-E001.md
├── reports/
│   ├── C001-codex-report.md
│   ├── E001-formalization-report.md
│   └── C002-codex-report.md
├── prompt-history.md
├── artifact-provenance.md
└── timeline.md

Le registre contiendrait alors :

### P003 — Revue critique de E001

- Source intégrale : `docs/research/prompts/P003-review-E001.md`
- Auteur : utilisateur
- Entrées : `E001`, `H001`, `P002`
- Sorties : `C002`
- Statut : exécuté

Et non plus une simple référence à une session externe.

Il faut aussi distinguer trois choses :

prompt original
    texte exact fourni à Codex

rapport de run
    réponse finale exacte de Codex

capitalisation
    chronologie, provenance, hypothèses et conclusions extraites

Aujourd’hui, Codex produit surtout la troisième couche. C’est utile, mais les deux premières constituent les preuves primaires et devraient également être préservées.

Donc mon verdict précis :

Codex respecte l’esprit de la mémoire de conception, mais pas encore complètement son exigence d’autonomie et de conservation des sources primaires.

Ce n’est pas une dérive grave. C’est une lacune de convention documentaire qu’il vaut mieux corriger dès P004, puis éventuellement rétrodocumenter P001 à P003 à partir des prompts encore disponibles.
