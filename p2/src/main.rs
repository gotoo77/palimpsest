mod scene;

use gotoo_pixel_engine::{
    EngineConfig, EngineError, Frame, Framebuffer, Game, GameResult, Key, Pixel, TextRenderer, run,
};
use palimpsest::Action;
use scene::{Command, Scene};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;

const BACKGROUND: Pixel = Pixel::rgb(13, 21, 33);
const PANEL: Pixel = Pixel::rgb(23, 36, 51);
const BORDER: Pixel = Pixel::rgb(75, 101, 119);
const TEXT: Pixel = Pixel::rgb(215, 228, 225);
const MUTED: Pixel = Pixel::rgb(149, 174, 180);
const CYAN: Pixel = Pixel::rgb(82, 225, 205);
const AMBER: Pixel = Pixel::rgb(243, 188, 92);
const PURPLE: Pixel = Pixel::rgb(181, 141, 234);
const RED: Pixel = Pixel::rgb(242, 127, 126);

struct PalimpsestGame {
    scene: Scene,
}

impl Game for PalimpsestGame {
    fn update(&mut self, frame: &mut Frame<'_>) -> GameResult {
        if frame.input.key(Key::Escape).pressed() {
            return GameResult::Exit;
        }
        let next = [
            (Key::R, Command::Reset),
            (Key::M, Command::ToggleAccess),
            (Key::E, Command::Inspect),
            (Key::C, Command::PredictCooperate),
            (Key::F, Command::PredictRefuse),
            (Key::Enter, Command::Reveal),
        ]
        .into_iter()
        .find_map(|(key, command)| frame.input.key(key).pressed().then_some(command));

        if let Some(command) = next {
            // The fixture is part of the tested E001 contract.
            self.scene
                .apply(command)
                .expect("the built-in E001 experiment must remain valid");
        }

        render(frame.framebuffer, &self.scene);
        GameResult::Continue
    }
}

fn label(fb: &mut Framebuffer, x: i32, y: i32, content: &str, color: Pixel) {
    TextRenderer::default().draw(fb, x, y, content, color);
}

fn panel(fb: &mut Framebuffer, x: i32, y: i32, w: u32, h: u32) {
    fb.fill_rect(x, y, w, h, PANEL);
    fb.draw_rect(x, y, w, h, BORDER);
}

fn person(fb: &mut Framebuffer, x: i32, name: &str, role: &str, color: Pixel) {
    let y = 67;
    panel(fb, x, y, 178, 89);
    fb.fill_circle(x + 24, y + 29, 12, color);
    fb.fill_rect(x + 9, y + 43, 31, 21, color);
    label(fb, x + 50, y + 18, name, TEXT);
    label(fb, x + 50, y + 33, role, MUTED);
}

fn render(fb: &mut Framebuffer, scene: &Scene) {
    fb.clear(BACKGROUND);
    fb.fill_rect(0, 0, WIDTH, 48, Pixel::rgb(7, 15, 25));
    fb.fill_rect(0, 46, WIDTH, 2, CYAN);
    label(fb, 18, 12, "PALIMPSEST  /  E001", CYAN);
    label(fb, 18, 29, "ONE MEMORY. TWO POSSIBLE DECISIONS.", TEXT);

    person(fb, 18, "ADA", "DECIDES", CYAN);
    person(fb, 231, "BORIN", "ASKS FOR HELP", AMBER);
    person(fb, 444, "CIRO", "CLAIM AUTHOR", PURPLE);

    panel(fb, 18, 165, 294, 91);
    label(fb, 29, 175, "EVENT RECORD  /  OBSERVER", CYAN);
    label(fb, 29, 194, "BORIN TOOK RESOURCE R1.", TEXT);
    label(fb, 29, 207, "R1 HAD BEEN PROMISED TO ADA.", TEXT);
    label(fb, 29, 233, "ADA CANNOT READ THIS RECORD.", MUTED);

    panel(fb, 325, 165, 297, 91);
    label(fb, 336, 175, "CIRO'S CLAIM  /  INTERPRETATION", PURPLE);
    if scene.inspected {
        label(fb, 336, 194, "CIRO ALLEGES A BROKEN PROMISE.", TEXT);
        label(fb, 336, 207, "ALLEGATION IS NOT A VERIFIED FACT.", MUTED);
    } else {
        label(fb, 336, 194, "PRESS E TO INSPECT THE CLAIM.", TEXT);
    }
    let access = if scene.claim_visible {
        "ADA ACCESS: VISIBLE"
    } else {
        "ADA ACCESS: MASKED"
    };
    label(
        fb,
        336,
        233,
        access,
        if scene.claim_visible { CYAN } else { RED },
    );

    panel(fb, 18, 266, 604, 77);
    label(
        fb,
        29,
        275,
        "M ACCESS   E INSPECT   C PREDICT HELP   F PREDICT REFUSE",
        TEXT,
    );

    if let Some(trace) = &scene.result {
        let outcome = match trace.chosen_action {
            Action::Cooperate => "ADA: COOPERATE",
            Action::Refuse => "ADA: REFUSE",
        };
        let verdict = if scene.predicted_correctly() == Some(true) {
            "PREDICTION MATCHED"
        } else {
            "PREDICTION MISSED"
        };
        label(fb, 29, 290, outcome, CYAN);
        label(fb, 180, 290, verdict, AMBER);
        label(
            fb,
            29,
            305,
            scene.causal_summary().unwrap_or("NO CAUSAL TRACE"),
            MUTED,
        );
        label(
            fb,
            29,
            328,
            "R RESET FOR ANOTHER EXPERIMENT  /  ESC QUIT",
            TEXT,
        );
    } else {
        let prediction = match scene.prediction {
            None => "PREDICTION: CHOOSE C OR F",
            Some(Action::Cooperate) => "PREDICTION: ADA WILL COOPERATE",
            Some(Action::Refuse) => "PREDICTION: ADA WILL REFUSE",
        };
        label(fb, 29, 292, prediction, AMBER);
        label(fb, 29, 311, "ENTER REVEAL (AFTER PREDICTION)", CYAN);
        label(
            fb,
            29,
            328,
            "OBSERVER SEES MORE THAN ADA  /  R RESET  /  ESC QUIT",
            MUTED,
        );
    }
}

fn main() -> Result<(), EngineError> {
    let (window_width, window_height) = if std::env::var_os("WSL_DISTRO_NAME").is_some() {
        // Match the native WSLg surface dimensions proven by GPE Arcade.
        (960, 612)
    } else {
        (1280, 720)
    };
    run(
        EngineConfig {
            title: "PALIMPSEST / E001 Causal Scene".into(),
            framebuffer_width: WIDTH,
            framebuffer_height: HEIGHT,
            window_width,
            window_height,
        },
        PalimpsestGame {
            scene: Scene::default(),
        },
    )
}
