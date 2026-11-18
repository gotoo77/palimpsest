//! Small, P2-specific French/English copy table.
//! This is presentation text only: no language-dependent simulation rules.

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum Language {
    #[default]
    French,
    English,
}

impl Language {
    pub const fn toggle(self) -> Self {
        match self {
            Self::French => Self::English,
            Self::English => Self::French,
        }
    }

    pub const fn copy(self) -> &'static UiText {
        match self {
            Self::French => &FRENCH,
            Self::English => &ENGLISH,
        }
    }
}

pub struct UiText {
    pub language_hint: &'static str,
    pub subtitle: &'static str,
    pub ada_role: &'static str,
    pub borin_role: &'static str,
    pub ciro_role: &'static str,
    pub record_title: &'static str,
    pub past_event: &'static str,
    pub past_context: &'static str,
    pub record_limit: &'static str,
    pub claim_title: &'static str,
    pub claim_line1: &'static str,
    pub claim_line2: &'static str,
    pub inspect_prompt: &'static str,
    pub inspect_explainer: &'static str,
    pub access_visible: &'static str,
    pub access_masked: &'static str,
    pub controls: &'static str,
    pub predict_prompt: &'static str,
    pub predict_help: &'static str,
    pub predict_refuse: &'static str,
    pub reveal_prompt: &'static str,
    pub result_help: &'static str,
    pub result_refuse: &'static str,
    pub prediction_right: &'static str,
    pub prediction_wrong: &'static str,
    pub reason_baseline: &'static str,
    pub reason_claim: &'static str,
    pub footer_before: &'static str,
    pub footer_after: &'static str,
}

pub const FRENCH: UiText = UiText {
    language_hint: "L: ENGLISH",
    subtitle: "BORIN DEMANDE DE L'AIDE A ADA. QUE VA-T-ELLE FAIRE ?",
    ada_role: "DECIDE",
    borin_role: "DEMANDE DE L'AIDE",
    ciro_role: "AUTEUR D'UN AVIS",
    record_title: "FAIT PASSE / POUR L'OBSERVATEUR",
    past_event: "BORIN A PRIS LA RESSOURCE R1.",
    past_context: "R1 ETAIT PROMISE A ADA.",
    record_limit: "ADA NE PEUT PAS LIRE CE COMPTE RENDU.",
    claim_title: "AVIS DE CIRO / PAS UNE PREUVE",
    claim_line1: "CIRO ACCUSE BORIN D'AVOIR ROMPU",
    claim_line2: "UNE PROMESSE FAITE A ADA.",
    inspect_prompt: "E : LIRE L'ACCUSATION DE CIRO.",
    inspect_explainer: "CET AVIS N'EST PAS UN FAIT VERIFIE.",
    access_visible: "ADA PEUT LIRE CET AVIS",
    access_masked: "AVIS CACHE A ADA",
    controls: "E LIRE   M ACCES ADA   C PREDIRE OUI   F PREDIRE NON   L EN",
    predict_prompt: "TON PRONOSTIC : C (OUI) OU F (NON)",
    predict_help: "TON PRONOSTIC : ADA AIDERA BORIN",
    predict_refuse: "TON PRONOSTIC : ADA REFUSERA",
    reveal_prompt: "ENTREE : REVELER LA DECISION",
    result_help: "ADA ACCEPTE",
    result_refuse: "ADA REFUSE",
    prediction_right: "PRONOSTIC JUSTE",
    prediction_wrong: "PRONOSTIC FAUX",
    reason_baseline: "SANS CET AVIS, ADA ACCEPTE L'AIDE PAR DEFAUT.",
    reason_claim: "ADA A LU L'AVIS : LA PRUDENCE L'EMPORTE.",
    footer_before: "TU CHOISIS L'ACCES A L'INFO, PAS LE PASSE. R REJOUER",
    footer_after: "R REJOUER   L ENGLISH   ESC QUITTER",
};

pub const ENGLISH: UiText = UiText {
    language_hint: "L: FRANCAIS",
    subtitle: "BORIN ASKS ADA FOR HELP. WHAT WILL SHE DO?",
    ada_role: "DECIDES",
    borin_role: "ASKS FOR HELP",
    ciro_role: "CLAIM AUTHOR",
    record_title: "PAST EVENT / OBSERVER ONLY",
    past_event: "BORIN TOOK RESOURCE R1.",
    past_context: "R1 HAD BEEN PROMISED TO ADA.",
    record_limit: "ADA CANNOT READ THIS EVENT RECORD.",
    claim_title: "CIRO'S CLAIM / NOT PROOF",
    claim_line1: "CIRO CLAIMS BORIN BROKE",
    claim_line2: "A PROMISE MADE TO ADA.",
    inspect_prompt: "E: READ CIRO'S ACCUSATION.",
    inspect_explainer: "THIS CLAIM IS NOT A VERIFIED FACT.",
    access_visible: "ADA CAN READ THIS CLAIM",
    access_masked: "CLAIM HIDDEN FROM ADA",
    controls: "E READ   M ADA ACCESS   C PREDICT YES   F PREDICT NO   L FR",
    predict_prompt: "YOUR PREDICTION: C (YES) OR F (NO)",
    predict_help: "YOUR PREDICTION: ADA WILL HELP",
    predict_refuse: "YOUR PREDICTION: ADA WILL REFUSE",
    reveal_prompt: "ENTER: REVEAL ADA'S DECISION",
    result_help: "ADA HELPS",
    result_refuse: "ADA REFUSES",
    prediction_right: "PREDICTION CORRECT",
    prediction_wrong: "PREDICTION WRONG",
    reason_baseline: "WITHOUT THIS CLAIM, ADA ACCEPTS BY DEFAULT.",
    reason_claim: "ADA READ THE CLAIM: CAUTION PREVAILS.",
    footer_before: "YOU CONTROL ACCESS TO INFO, NOT THE PAST. R RESTART",
    footer_after: "R RESTART   L FRANCAIS   ESC QUIT",
};

#[cfg(test)]
mod tests {
    use super::*;
    use gotoo_pixel_engine::TextRenderer;

    #[test]
    fn french_is_default_and_language_toggle_is_reversible() {
        let initial = Language::default();
        assert_eq!(initial, Language::French);
        assert_eq!(initial.toggle(), Language::English);
        assert_eq!(initial.toggle().toggle(), initial);
    }

    #[test]
    fn both_languages_fit_existing_pixel_panels() {
        let font = TextRenderer::default();
        for lang in [Language::French, Language::English] {
            let text = lang.copy();
            for value in [
                text.ada_role,
                text.borin_role,
                text.ciro_role,
            ] {
                assert!(font.text_size(value, 1).0 <= 119, "role label too long: {value}");
            }
            for value in [
                text.record_title,
                text.past_event,
                text.past_context,
                text.record_limit,
                text.claim_title,
                text.claim_line1,
                text.claim_line2,
                text.inspect_prompt,
                text.inspect_explainer,
                text.access_visible,
                text.access_masked,
            ] {
                assert!(font.text_size(value, 1).0 <= 272, "panel label too long: {value}");
            }
            for value in [
                text.subtitle,
                text.controls,
                text.predict_prompt,
                text.predict_help,
                text.predict_refuse,
                text.reveal_prompt,
                text.reason_baseline,
                text.reason_claim,
                text.footer_before,
                text.footer_after,
            ] {
                assert!(font.text_size(value, 1).0 <= 582, "screen label too long: {value}");
            }
            assert!(font.text_size(text.result_refuse, 1).0 < 150);
            assert!(font.text_size(text.result_help, 1).0 < 150);
        }
    }
}
