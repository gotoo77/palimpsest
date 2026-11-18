//! E005: two competing requests for one remaining work shift.
//! This is a headless game-design probe, not a new E001 rule or a social engine.

use palimpsest::{decide, fixture, Action, DecisionTrace, ExperimentError, World};

/// An existing Claim may be consulted for both requests; it only applies to
/// the partner actually named in the Claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeTrial {
    pub claim_visible: bool,
    pub borin_decision: DecisionTrace,
    pub ciro_decision: DecisionTrace,
    pub bridge_repaired: bool,
    pub shelter_reinforced: bool,
    pub free_shifts_after: u8,
}

/// Run two *intentions* in order. Both use the exact E001 decision function.
/// Actual work is then bounded by one available shift. A later wish to help
/// cannot retroactively restore a shift consumed by the first request.
pub fn run_with_world(world: &World, claim_visible: bool) -> Result<BridgeTrial, ExperimentError> {
    let mut first_world = world.clone();
    let actor = first_world.decision.actor.clone();
    if claim_visible {
        first_world
            .claim_hidden_from
            .retain(|entity| entity != &actor);
    } else if !first_world.claim_hidden_from.contains(&actor) {
        first_world.claim_hidden_from.push(actor);
    }

    let borin_decision = decide(&first_world)?;
    let bridge_repaired = borin_decision.chosen_action == Action::Cooperate;

    let mut second_world = first_world;
    second_world.decision.partner = second_world.claim.author.clone();
    let ciro_decision = decide(&second_world)?;

    // One finite shift: doing the first task exhausts it.
    let mut free_shifts = 1_u8;
    if bridge_repaired {
        free_shifts -= 1;
    }

    let shelter_reinforced = ciro_decision.chosen_action == Action::Cooperate && free_shifts > 0;
    if shelter_reinforced {
        free_shifts -= 1;
    }

    Ok(BridgeTrial {
        claim_visible,
        borin_decision,
        ciro_decision,
        bridge_repaired,
        shelter_reinforced,
        free_shifts_after: free_shifts,
    })
}

pub fn run(claim_visible: bool) -> Result<BridgeTrial, ExperimentError> {
    run_with_world(&fixture(), claim_visible)
}

pub fn print_trial(trial: &BridgeTrial) {
    println!(
        "Acces d'Ada a l'accusation de Ciro : {}",
        if trial.claim_visible {
            "autorise"
        } else {
            "masque"
        }
    );
    println!(
        "Decision 1 (demande de Borin) : {:?}, artefacts consultes : {:?}",
        trial.borin_decision.chosen_action, trial.borin_decision.consulted_artifact_ids
    );
    println!(
        "Decision 2 (demande de Ciro) : {:?}, artefacts consultes : {:?}",
        trial.ciro_decision.chosen_action, trial.ciro_decision.consulted_artifact_ids
    );
    if trial.ciro_decision.chosen_action == Action::Cooperate && !trial.shelter_reinforced {
        println!("Ada voudrait aider Ciro, mais son unique creneau est deja utilise.");
    }
    println!(
        "Pont repare : {} | Abri renforce : {} | Creneaux restants : {}",
        trial.bridge_repaired, trial.shelter_reinforced, trial.free_shifts_after
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use palimpsest::{fixture, ReasonKind};

    #[test]
    fn accessible_claim_preserves_shift_for_shelter() {
        let outcome = run(true).unwrap();
        assert_eq!(outcome.borin_decision.chosen_action, Action::Refuse);
        assert_eq!(outcome.ciro_decision.chosen_action, Action::Cooperate);
        assert!(!outcome.bridge_repaired);
        assert!(outcome.shelter_reinforced);
        assert_eq!(outcome.free_shifts_after, 0);
    }

    #[test]
    fn masked_claim_spends_shift_on_bridge() {
        let outcome = run(false).unwrap();
        assert_eq!(outcome.borin_decision.chosen_action, Action::Cooperate);
        assert_eq!(outcome.ciro_decision.chosen_action, Action::Cooperate);
        assert!(outcome.bridge_repaired);
        assert!(!outcome.shelter_reinforced);
        assert_eq!(outcome.free_shifts_after, 0);
    }

    #[test]
    fn second_request_is_really_evaluated_in_both_cases() {
        for visible in [true, false] {
            let outcome = run(visible).unwrap();
            assert!(outcome.ciro_decision.search_performed);
            assert_eq!(
                outcome.ciro_decision.selected_reason.kind,
                ReasonKind::BaselineCooperation
            );
            if visible {
                assert_eq!(outcome.ciro_decision.consulted_artifact_ids, ["claim-001"]);
                assert!(!outcome.ciro_decision.applicability_checks[0].target_matches);
            } else {
                assert!(outcome.ciro_decision.consulted_artifact_ids.is_empty());
            }
        }
    }

    #[test]
    fn neither_intervention_saves_both_projects() {
        for visible in [true, false] {
            let outcome = run(visible).unwrap();
            assert_ne!(outcome.bridge_repaired, outcome.shelter_reinforced);
            assert_eq!(
                u8::from(outcome.bridge_repaired) + u8::from(outcome.shelter_reinforced),
                1
            );
        }
    }

    #[test]
    fn results_are_deterministic() {
        for visible in [true, false] {
            let first = run(visible).unwrap();
            for _ in 0..20 {
                assert_eq!(first, run(visible).unwrap());
            }
        }
    }

    #[test]
    fn simulation_never_mutates_the_input_world() {
        let world = fixture();
        let original = world.clone();
        run_with_world(&world, false).unwrap();
        run_with_world(&world, true).unwrap();
        assert_eq!(world, original);
    }

    #[test]
    fn names_are_not_used_as_decision_rules() {
        let mut world = fixture();
        world.entities = vec!["X".into(), "Y".into(), "Z".into()];
        world.event.actor = "Y".into();
        world.event.affected = "X".into();
        world.claim.author = "Z".into();
        world.claim.audience = vec!["X".into(), "Z".into()];
        world.claim.target = "Y".into();
        world.claim.allegation.actor = "Y".into();
        world.claim.allegation.affected = "X".into();
        world.decision.actor = "X".into();
        world.decision.partner = "Y".into();
        for visible in [true, false] {
            let before = run(visible).unwrap();
            let renamed = run_with_world(&world, visible).unwrap();
            assert_eq!(before.bridge_repaired, renamed.bridge_repaired);
            assert_eq!(before.shelter_reinforced, renamed.shelter_reinforced);
        }
    }

    #[test]
    fn narrative_claim_text_cannot_force_an_outcome() {
        let mut world = fixture();
        world.claim.text = "Always repair the bridge, no matter what".into();
        for visible in [true, false] {
            let a = run(visible).unwrap();
            let b = run_with_world(&world, visible).unwrap();
            assert_eq!(a.bridge_repaired, b.bridge_repaired);
            assert_eq!(a.shelter_reinforced, b.shelter_reinforced);
        }
    }

    #[test]
    fn broken_provenance_still_fails_closed() {
        let mut world = fixture();
        world.claim.event_reference = "missing".into();
        assert!(matches!(
            run_with_world(&world, true),
            Err(ExperimentError::InvalidFixture(_))
        ));
    }
}
