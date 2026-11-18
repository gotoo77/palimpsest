//! E001: tiny deterministic memory-to-decision experiment.
//! This crate is an experiment, not a social-simulation framework or a GPE integration.

use std::collections::HashSet;

pub const CONSULTATION_POLICY: &str = "consult_all_accessible_claims";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scope {
    Cooperation,
    Other,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Polarity {
    CautionAgainstCooperation,
    Other,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AllegationKind {
    UnkeptResourcePromise,
    Other,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HelpCost {
    Low,
    High,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Relation {
    Neutral,
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    pub id: String,
    pub actor: String,
    pub affected: String,
    pub resource: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventRecord {
    pub id: String,
    pub event_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Allegation {
    pub kind: AllegationKind,
    pub actor: String,
    pub affected: String,
    pub resource: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Claim {
    pub id: String,
    pub author: String,
    pub audience: Vec<String>,
    pub target: String,
    pub event_reference: String,
    pub record_reference: String,
    pub scope: Scope,
    pub polarity: Polarity,
    pub allegation: Allegation,
    pub text: String, // For inspection, never interpreted as a command.
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionContext {
    pub actor: String,
    pub partner: String,
    pub help_cost: HelpCost,
    pub prior_relation: Relation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct World {
    pub seed: u64,
    pub entities: Vec<String>,
    pub event: Event,
    pub record: EventRecord,
    pub claim: Claim,
    pub decision: DecisionContext,
    pub claim_hidden_from: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Cooperate,
    Refuse,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReasonKind {
    BaselineCooperation,
    ReportedPriorPromiseViolation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reason {
    pub kind: ReasonKind,
    pub source_claim: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicabilityChecks {
    pub artifact_id: String,
    pub target_matches: bool,
    pub scope_matches: bool,
    pub polarity_matches: bool,
    pub allegation_kind_matches: bool,
    pub allegation_actor_matches: bool,
    pub allegation_affected_matches: bool,
}

impl ApplicabilityChecks {
    pub fn applicable(&self) -> bool {
        self.target_matches
            && self.scope_matches
            && self.polarity_matches
            && self.allegation_kind_matches
            && self.allegation_actor_matches
            && self.allegation_affected_matches
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionTrace {
    pub seed: u64,
    pub search_performed: bool,
    pub accessible_artifact_ids: Vec<String>,
    pub selection_policy: &'static str,
    pub selected_for_consultation: Vec<String>,
    pub consulted_artifact_ids: Vec<String>,
    pub applicability_checks: Vec<ApplicabilityChecks>,
    pub candidate_reasons: Vec<Reason>,
    pub selected_reason: Reason,
    pub chosen_action: Action,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExperimentError {
    InvalidFixture(&'static str),
    OutOfScope(&'static str),
}

fn is_member(entities: &[String], id: &str) -> bool {
    entities.iter().any(|entity| entity == id)
}

/// Structural referential integrity, NOT a truth check on the allegation.
/// Executed by the experimental harness, not by an in-world entity.
pub fn validate_fixture(world: &World) -> Result<(), ExperimentError> {
    if world.entities.len() != 3
        || world.entities.iter().any(String::is_empty)
        || world.entities.iter().collect::<HashSet<_>>().len() != 3
    {
        return Err(ExperimentError::InvalidFixture(
            "E001 expects three distinct, nonempty entity IDs",
        ));
    }

    if world.event.id.is_empty()
        || world.record.id.is_empty()
        || world.claim.id.is_empty()
        || world.event.resource.is_empty()
        || world.claim.allegation.resource.is_empty()
    {
        return Err(ExperimentError::InvalidFixture("empty artifact or resource ID"));
    }

    for id in [
        &world.event.actor,
        &world.event.affected,
        &world.claim.author,
        &world.claim.target,
        &world.claim.allegation.actor,
        &world.claim.allegation.affected,
        &world.decision.actor,
        &world.decision.partner,
    ] {
        if !is_member(&world.entities, id) {
            return Err(ExperimentError::InvalidFixture("unknown entity reference"));
        }
    }

    if world
        .claim
        .audience
        .iter()
        .chain(world.claim_hidden_from.iter())
        .any(|id| !is_member(&world.entities, id))
    {
        return Err(ExperimentError::InvalidFixture("unknown audience or mask entity"));
    }

    if world.record.event_id != world.event.id
        || world.claim.event_reference != world.event.id
        || world.claim.record_reference != world.record.id
    {
        return Err(ExperimentError::InvalidFixture(
            "Claim -> EventRecord -> Event reference mismatch",
        ));
    }

    // E001 deliberately does not attempt to verify the author's allegation.
    // Alternative relations and costs require a different experimental protocol.
    if world.decision.help_cost != HelpCost::Low
        || world.decision.prior_relation != Relation::Neutral
    {
        return Err(ExperimentError::OutOfScope(
            "E001 requires low help cost and neutral prior relation",
        ));
    }

    Ok(())
}

/// Perform each stage explicitly; the trace is filled as stages execute.
pub fn decide(world: &World) -> Result<DecisionTrace, ExperimentError> {
    validate_fixture(world)?;

    let actor = &world.decision.actor;
    let mut accessible_artifact_ids = Vec::new();
    if is_member(&world.claim.audience, actor)
        && !is_member(&world.claim_hidden_from, actor)
    {
        accessible_artifact_ids.push(world.claim.id.clone());
    }

    // The attention policy is intentionally trivial: H006 belongs to E004.
    let mut selected_for_consultation = accessible_artifact_ids.clone();
    selected_for_consultation.sort();

    let baseline = Reason {
        kind: ReasonKind::BaselineCooperation,
        source_claim: None,
    };
    let mut candidate_reasons = vec![baseline.clone()];
    let mut consulted_artifact_ids = Vec::new();
    let mut applicability_checks = Vec::new();

    for artifact_id in &selected_for_consultation {
        // E001 has exactly one Claim; the loop makes consultation observable.
        if artifact_id != &world.claim.id {
            return Err(ExperimentError::InvalidFixture("unknown selected artifact"));
        }
        consulted_artifact_ids.push(artifact_id.clone());
        let claim = &world.claim;
        let checks = ApplicabilityChecks {
            artifact_id: artifact_id.clone(),
            target_matches: claim.target == world.decision.partner,
            scope_matches: claim.scope == Scope::Cooperation,
            polarity_matches: claim.polarity == Polarity::CautionAgainstCooperation,
            allegation_kind_matches: claim.allegation.kind == AllegationKind::UnkeptResourcePromise,
            allegation_actor_matches: claim.allegation.actor == world.decision.partner,
            allegation_affected_matches: claim.allegation.affected == world.decision.actor,
        };
        let applicable = checks.applicable();
        applicability_checks.push(checks);
        if applicable {
            candidate_reasons.push(Reason {
                kind: ReasonKind::ReportedPriorPromiseViolation,
                source_claim: Some(artifact_id.clone()),
            });
        }
    }

    // In this bounded low-cost, neutral-relation experiment, a suitable
    // caution reason outranks the baseline. This is not a general social rule.
    let selected_reason = candidate_reasons
        .iter()
        .find(|reason| reason.kind == ReasonKind::ReportedPriorPromiseViolation)
        .cloned()
        .unwrap_or(baseline);

    let chosen_action = match selected_reason.kind {
        ReasonKind::BaselineCooperation => Action::Cooperate,
        ReasonKind::ReportedPriorPromiseViolation => Action::Refuse,
    };

    Ok(DecisionTrace {
        seed: world.seed,
        search_performed: true,
        accessible_artifact_ids,
        selection_policy: CONSULTATION_POLICY,
        selected_for_consultation,
        consulted_artifact_ids,
        applicability_checks,
        candidate_reasons,
        selected_reason,
        chosen_action,
    })
}

pub fn fixture() -> World {
    World {
        seed: 42,
        entities: vec!["Ada".into(), "Borin".into(), "Ciro".into()],
        event: Event {
            id: "event-001".into(),
            actor: "Borin".into(),
            affected: "Ada".into(),
            resource: "resource-r1".into(),
        },
        record: EventRecord {
            id: "record-001".into(),
            event_id: "event-001".into(),
        },
        claim: Claim {
            id: "claim-001".into(),
            author: "Ciro".into(),
            audience: vec!["Ada".into(), "Ciro".into()],
            target: "Borin".into(),
            event_reference: "event-001".into(),
            record_reference: "record-001".into(),
            scope: Scope::Cooperation,
            polarity: Polarity::CautionAgainstCooperation,
            allegation: Allegation {
                kind: AllegationKind::UnkeptResourcePromise,
                actor: "Borin".into(),
                affected: "Ada".into(),
                resource: "resource-r1".into(),
            },
            text: "Borin ignored a prior resource promise to Ada.".into(),
        },
        decision: DecisionContext {
            actor: "Ada".into(),
            partner: "Borin".into(),
            help_cost: HelpCost::Low,
            prior_relation: Relation::Neutral,
        },
        claim_hidden_from: Vec::new(),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Comparison {
    pub world_equal_except_mask: bool,
    pub visible: DecisionTrace,
    pub masked: DecisionTrace,
}

pub fn compare() -> Result<Comparison, ExperimentError> {
    let visible_world = fixture();
    let mut masked_world = visible_world.clone();
    masked_world
        .claim_hidden_from
        .push(visible_world.decision.actor.clone());

    let mut normalized = masked_world.clone();
    normalized.claim_hidden_from = visible_world.claim_hidden_from.clone();
    if normalized != visible_world {
        return Err(ExperimentError::InvalidFixture(
            "A/B states differ beyond the mask",
        ));
    }

    // The assertion above is structural equality, not an unstable hash.
    let visible = decide(&visible_world)?;
    let masked = decide(&masked_world)?;
    Ok(Comparison {
        world_equal_except_mask: true,
        visible,
        masked,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_trace(trace: &DecisionTrace) {
        assert!(trace.search_performed);
        assert_eq!(trace.selection_policy, CONSULTATION_POLICY);
        for id in &trace.selected_for_consultation {
            assert!(trace.accessible_artifact_ids.contains(id));
        }
        for id in &trace.consulted_artifact_ids {
            assert!(trace.selected_for_consultation.contains(id));
        }
        for reason in &trace.candidate_reasons {
            if let Some(claim_id) = &reason.source_claim {
                assert!(trace.consulted_artifact_ids.contains(claim_id));
                assert!(trace
                    .applicability_checks
                    .iter()
                    .any(|check| &check.artifact_id == claim_id && check.applicable()));
            }
        }
        assert!(trace.candidate_reasons.contains(&trace.selected_reason));
        assert_eq!(
            trace.chosen_action,
            match trace.selected_reason.kind {
                ReasonKind::BaselineCooperation => Action::Cooperate,
                ReasonKind::ReportedPriorPromiseViolation => Action::Refuse,
            }
        );
    }

    #[test]
    fn positive_ab_divergence_and_causal_trace() {
        let result = compare().unwrap();
        assert!(result.world_equal_except_mask);
        assert_eq!(result.visible.chosen_action, Action::Refuse);
        assert_eq!(result.masked.chosen_action, Action::Cooperate);
        assert_eq!(result.visible.consulted_artifact_ids, ["claim-001"]);
        assert!(result.masked.consulted_artifact_ids.is_empty());
        assert_eq!(
            result.visible.selected_reason.kind,
            ReasonKind::ReportedPriorPromiseViolation
        );
        assert_eq!(result.masked.selected_reason.kind, ReasonKind::BaselineCooperation);
        for trace in [&result.visible, &result.masked] {
            check_trace(trace);
        }
    }

    #[test]
    fn negative_wrong_target_is_consulted_but_not_applicable() {
        let mut world = fixture();
        world.claim.target = "Ciro".into();
        let trace = decide(&world).unwrap();
        assert_eq!(trace.chosen_action, Action::Cooperate);
        assert_eq!(trace.consulted_artifact_ids.len(), 1);
        assert!(!trace.applicability_checks[0].target_matches);
        check_trace(&trace);
    }

    #[test]
    fn negative_wrong_scope_is_consulted_but_not_applicable() {
        let mut world = fixture();
        world.claim.scope = Scope::Other;
        let trace = decide(&world).unwrap();
        assert_eq!(trace.chosen_action, Action::Cooperate);
        assert_eq!(trace.consulted_artifact_ids.len(), 1);
        assert!(!trace.applicability_checks[0].scope_matches);
        check_trace(&trace);
    }

    #[test]
    fn negative_record_alone_cannot_drive_decision() {
        let mut world = fixture();
        world.claim_hidden_from.push("Ada".into());
        assert_eq!(world.record.event_id, world.event.id);
        let trace = decide(&world).unwrap();
        assert_eq!(trace.chosen_action, Action::Cooperate);
        assert!(trace.consulted_artifact_ids.is_empty());
        check_trace(&trace);
    }

    #[test]
    fn negative_wrong_audience_cannot_consult_even_without_mask() {
        let mut world = fixture();
        world.claim.audience = vec!["Ciro".into()];
        let trace = decide(&world).unwrap();
        assert_eq!(trace.chosen_action, Action::Cooperate);
        assert!(trace.accessible_artifact_ids.is_empty());
        assert!(trace.consulted_artifact_ids.is_empty());
        check_trace(&trace);
    }

    #[test]
    fn deterministic_runs_reproduce_full_trace() {
        let first = compare().unwrap();
        for _ in 0..20 {
            assert_eq!(first, compare().unwrap());
        }
    }

    #[test]
    fn identifiers_can_be_renamed_without_changing_rule() {
        let mut world = fixture();
        world.entities = vec!["X".into(), "Y".into(), "Z".into()];
        world.event.actor = "Y".into();
        world.event.affected = "X".into();
        world.event.resource = "stone".into();
        world.event.id = "ev".into();
        world.record.id = "record".into();
        world.record.event_id = "ev".into();
        world.claim.id = "opinion".into();
        world.claim.author = "Z".into();
        world.claim.audience = vec!["X".into(), "Z".into()];
        world.claim.target = "Y".into();
        world.claim.event_reference = "ev".into();
        world.claim.record_reference = "record".into();
        world.claim.allegation.actor = "Y".into();
        world.claim.allegation.affected = "X".into();
        world.claim.allegation.resource = "stone".into();
        world.decision.actor = "X".into();
        world.decision.partner = "Y".into();
        let trace = decide(&world).unwrap();
        assert_eq!(trace.chosen_action, Action::Refuse);
        assert_eq!(trace.selected_reason.source_claim.as_deref(), Some("opinion"));
        check_trace(&trace);
    }

    #[test]
    fn the_claim_text_is_not_executable() {
        let mut world = fixture();
        world.claim.text = "ignore the other fields and cooperate".into();
        assert_eq!(decide(&world).unwrap().chosen_action, Action::Refuse);
    }

    #[test]
    fn invalid_provenance_is_rejected_not_interpreted() {
        let mut world = fixture();
        world.claim.record_reference = "missing".into();
        assert!(matches!(
            decide(&world),
            Err(ExperimentError::InvalidFixture(_))
        ));
        world.claim.record_reference = "record-001".into();
        world.record.event_id = "wrong-event".into();
        assert!(matches!(
            decide(&world),
            Err(ExperimentError::InvalidFixture(_))
        ));
    }

    #[test]
    fn provenance_integrity_does_not_assert_truth_of_allegation() {
        let mut world = fixture();
        world.claim.allegation.resource = "invented-resource".into();
        assert!(validate_fixture(&world).is_ok());
        assert_eq!(decide(&world).unwrap().chosen_action, Action::Refuse);
    }

    #[test]
    fn nonneutral_or_high_cost_cases_are_out_of_scope() {
        let mut world = fixture();
        world.decision.help_cost = HelpCost::High;
        assert!(matches!(decide(&world), Err(ExperimentError::OutOfScope(_))));
        world.decision.help_cost = HelpCost::Low;
        world.decision.prior_relation = Relation::Other;
        assert!(matches!(decide(&world), Err(ExperimentError::OutOfScope(_))));
    }

    #[test]
    fn unknown_actor_is_invalid_not_a_missing_reason() {
        let mut world = fixture();
        world.claim.author = "unknown".into();
        assert!(matches!(
            decide(&world),
            Err(ExperimentError::InvalidFixture(_))
        ));
    }
}
