use palimpsest::{Action, DecisionTrace, ExperimentError, decide, fixture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    ToggleAccess,
    Inspect,
    PredictCooperate,
    PredictRefuse,
    Reveal,
    Reset,
}

/// P2 adapter: the source of truth for decisions remains the P1 crate.
/// The player only chooses visibility and makes a prediction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scene {
    pub claim_visible: bool,
    pub inspected: bool,
    pub prediction: Option<Action>,
    pub result: Option<DecisionTrace>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            claim_visible: true,
            inspected: false,
            prediction: None,
            result: None,
        }
    }
}

impl Scene {
    pub fn apply(&mut self, command: Command) -> Result<(), ExperimentError> {
        if command == Command::Reset {
            *self = Self::default();
            return Ok(());
        }

        // No post-reveal changes; a second experiment requires a full reset.
        if self.result.is_some() {
            return Ok(());
        }

        match command {
            Command::ToggleAccess => self.claim_visible = !self.claim_visible,
            Command::Inspect => self.inspected = !self.inspected,
            Command::PredictCooperate => self.prediction = Some(Action::Cooperate),
            Command::PredictRefuse => self.prediction = Some(Action::Refuse),
            Command::Reveal if self.prediction.is_some() => {
                let mut world = fixture();
                if !self.claim_visible {
                    world.claim_hidden_from.push(world.decision.actor.clone());
                }
                // Do not infer the decision from the UI state. Execute E001.
                self.result = Some(decide(&world)?);
            }
            Command::Reveal | Command::Reset => {}
        }
        Ok(())
    }

    pub fn predicted_correctly(&self) -> Option<bool> {
        Some(self.prediction? == self.result.as_ref()?.chosen_action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_reveal_before_prediction() {
        let mut scene = Scene::default();
        scene.apply(Command::Reveal).unwrap();
        assert!(scene.result.is_none());
        assert!(scene.predicted_correctly().is_none());
    }

    #[test]
    fn visible_claim_is_resolved_by_e001_not_by_prediction() {
        let mut scene = Scene::default();
        scene.apply(Command::PredictCooperate).unwrap();
        scene.apply(Command::Reveal).unwrap();
        let result = scene.result.as_ref().unwrap();
        assert_eq!(result.chosen_action, Action::Refuse);
        assert_eq!(result.consulted_artifact_ids, ["claim-001"]);
        assert_eq!(scene.predicted_correctly(), Some(false));
    }

    #[test]
    fn masking_claim_changes_decision_and_trace() {
        let mut scene = Scene::default();
        scene.apply(Command::ToggleAccess).unwrap();
        scene.apply(Command::PredictCooperate).unwrap();
        scene.apply(Command::Reveal).unwrap();
        let result = scene.result.as_ref().unwrap();
        assert_eq!(result.chosen_action, Action::Cooperate);
        assert!(result.accessible_artifact_ids.is_empty());
        assert!(result.consulted_artifact_ids.is_empty());
        assert_eq!(scene.predicted_correctly(), Some(true));
    }

    #[test]
    fn inspection_is_observer_only() {
        let mut scene = Scene::default();
        scene.apply(Command::Inspect).unwrap();
        assert!(scene.inspected);
        scene.apply(Command::PredictRefuse).unwrap();
        scene.apply(Command::Reveal).unwrap();
        assert_eq!(scene.result.as_ref().unwrap().chosen_action, Action::Refuse);
    }

    #[test]
    fn cannot_change_claim_or_prediction_after_reveal() {
        let mut scene = Scene::default();
        scene.apply(Command::PredictRefuse).unwrap();
        scene.apply(Command::Reveal).unwrap();
        let prior = scene.clone();
        scene.apply(Command::ToggleAccess).unwrap();
        scene.apply(Command::PredictCooperate).unwrap();
        scene.apply(Command::Inspect).unwrap();
        assert_eq!(scene, prior);
    }

    #[test]
    fn reset_starts_a_fresh_run() {
        let mut scene = Scene::default();
        scene.apply(Command::ToggleAccess).unwrap();
        scene.apply(Command::PredictRefuse).unwrap();
        scene.apply(Command::Reveal).unwrap();
        scene.apply(Command::Reset).unwrap();
        assert_eq!(scene, Scene::default());
    }

    #[test]
    fn same_intervention_reproduces_same_trace() {
        let mut first = Scene::default();
        first.apply(Command::PredictRefuse).unwrap();
        first.apply(Command::Reveal).unwrap();
        let mut next = first.clone();
        next.apply(Command::Reset).unwrap();
        next.apply(Command::PredictRefuse).unwrap();
        next.apply(Command::Reveal).unwrap();
        assert_eq!(first.result, next.result);
    }
}
