# RP008 - Revue adversariale de la revision E001 P0

- Date : 2026-11-18
- Statut : revue documentaire de preparation ; aucun resultat de simulation.
- Sources : E001, C002, C003, P009 ; cible H001 seulement.

## Matrice de resolution des critiques

| Observation | Revision apportee | Validation restante |
| --- | --- | --- |
| C002 F001 : force du Claim pre-cablee | Suppression de la force ; priorite de raisons dans une regle commune | Verifier l'absence de branche specifique a une fixture |
| F002 : choix par defaut passif | Baseline positive de cooperation sous faible cout et relation neutre | Tester la baseline sans Claim applicable |
| F003-F004 et C003 : acces et consultation confondus | Enumeration, selection triviale, consultation, applicabilite distinctes | Traces d'etapes executees |
| F005 : scenario encode | Champs structures et permutation d'identifiants | Executer le test de permutation |
| F006 : trace declarative | Trace atomique construite pendant la decision | Audit des references et de l'ordre d'execution |
| F007 : seuil de revue absent | Gate P0 avec criteres GO / REVISE / NO-GO | Revue humaine independante |

## Attaques methodologiques

1. **Une priorite explicite peut encore rendre la divergence attendue trop facile.** Le protocole teste une causalite locale dans un domaine borne, pas l'emergence. Les controles negatifs et la permutation doivent empecher les cas particuliers.
2. **Un Claim structure reste proche d'une raison de refus.** L'implementation devra prouver la chaine accessibilite -> consultation -> checks -> raison -> decision et non simplement comparer des boolens.
3. **Une allegation fausse peut influencer Ada.** C'est permis dans cette experience : E001 ne verifie ni la verite ni la credibilite.
4. **H006 n'est pas testee.** Une selection triviale stabilise seulement la variable de consultation ; les politiques d'attention demeurent candidates pour E004.
5. **La lisibilite humaine ne prouve pas le fun.** Une future interface GPE aura besoin d'une gate de gameplay autonome.
6. **La generalite n'est pas acquise.** Une micro-simulation n'autorise aucun moteur generique ni extraction en crate partage.

## Controles a effectuer dans la PR

- Diff limite a la documentation P0 ; C002 et C003 conserves.
- Coherence E001, H001, H006 ; toutes les references vers P009/RP008 resolues.
- Aucun vocabulaire ne presente les oracles comme des traces deja executees.
- Aucun code, runtime, test simule ou dependance.
- Checks de whitespace et de structure documentaire, puis revue humaine des hypotheses.

## Verdict

**REVISE : gate P0 encore ouverte.** Le protocole parait mieux defini, mais les controles conceptuels doivent etre relus independamment. Aucune preuve de H001 n'est produite. Meme un futur GO de P0 autorisera uniquement une micro-implementation, pas le choix automatique de Rust, l'integration GPE ou la validation ludique.
