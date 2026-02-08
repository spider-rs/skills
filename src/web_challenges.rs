//! Built-in web challenge solving skills.
//!
//! Each function adds a specific skill to a registry. Use [`registry()`] to
//! get a pre-loaded registry with all built-in skills, or add individual
//! skills for a smaller footprint.

use crate::{Skill, SkillRegistry, SkillTrigger};

/// Create a registry pre-loaded with all built-in web challenge skills.
pub fn registry() -> SkillRegistry {
    let mut r = SkillRegistry::new();

    // ─── Original skills ─────────────────────────────────────────────
    add_image_grid(&mut r);
    add_text_captcha(&mut r);
    add_rotation_puzzle(&mut r);
    add_tic_tac_toe(&mut r);
    add_word_search(&mut r);
    add_slider_drag(&mut r);

    // ─── CAPTCHA variants ────────────────────────────────────────────
    add_audio_captcha(&mut r);
    add_puzzle_piece_captcha(&mut r);
    add_honeypot_captcha(&mut r);
    add_math_captcha(&mut r);
    add_visual_pattern_captcha(&mut r);
    add_recaptcha_v2(&mut r);
    add_recaptcha_v3(&mut r);
    add_hcaptcha(&mut r);
    add_cloudflare_turnstile(&mut r);
    add_geetest(&mut r);
    add_arkose_funcaptcha(&mut r);
    add_text_in_image_captcha(&mut r);
    add_sequence_ordering_captcha(&mut r);
    add_image_rotation_captcha(&mut r);
    add_icon_captcha(&mut r);
    add_semantic_captcha(&mut r);
    add_click_order_captcha(&mut r);
    add_captcha_3d_object(&mut r);
    add_captcha_audio_v2(&mut r);

    // ─── Puzzle games ────────────────────────────────────────────────
    add_jigsaw_puzzle(&mut r);
    add_sliding_tile_puzzle(&mut r);
    add_maze_solving(&mut r);
    add_sudoku(&mut r);
    add_crossword(&mut r);
    add_connect_the_dots(&mut r);
    add_pattern_matching(&mut r);
    add_memory_card_game(&mut r);
    add_number_sequence(&mut r);
    add_color_matching(&mut r);
    add_shape_sorting(&mut r);
    add_spot_the_difference(&mut r);
    add_object_counting(&mut r);

    // ─── Form interactions ───────────────────────────────────────────
    add_multi_step_form(&mut r);
    add_file_upload(&mut r);
    add_address_form(&mut r);
    add_payment_form(&mut r);
    add_form_validation(&mut r);
    add_otp_input(&mut r);
    add_web_form_autofill(&mut r);

    // ─── Navigation challenges ───────────────────────────────────────
    add_infinite_scroll(&mut r);
    add_popup_modal(&mut r);
    add_cookie_consent(&mut r);
    add_age_verification(&mut r);
    add_login_wall(&mut r);
    add_paywall_detection(&mut r);
    add_redirect_chain(&mut r);
    add_iframe_interaction(&mut r);
    add_lazy_loaded_content(&mut r);
    add_pagination_navigation(&mut r);

    // ─── Visual challenges ───────────────────────────────────────────
    add_drag_drop_sorting(&mut r);
    add_chart_data_extraction(&mut r);
    add_drag_to_target(&mut r);

    // ─── Data extraction ─────────────────────────────────────────────
    add_table_extraction(&mut r);
    add_product_listing(&mut r);
    add_contact_extraction(&mut r);
    add_search_results(&mut r);
    add_price_scraping(&mut r);

    // ─── Security challenges ─────────────────────────────────────────
    add_rate_limiting(&mut r);
    add_bot_detection(&mut r);
    add_fingerprint_challenge(&mut r);
    add_proof_of_work(&mut r);
    add_js_challenge_page(&mut r);
    add_device_verification(&mut r);

    // ─── Interactive challenge skills ──────────────────────────────────
    add_checkbox_click(&mut r);
    add_license_plate(&mut r);
    add_nested_grid(&mut r);
    add_whack_a_mole(&mut r);
    add_find_waldo(&mut r);
    add_chihuahua_muffin(&mut r);
    add_reverse_selection(&mut r);
    add_affirmations(&mut r);
    add_parking_challenge(&mut r);
    add_three_d_object(&mut r);
    add_draw_circle(&mut r);
    add_push_drag(&mut r);
    add_dark_hidden(&mut r);
    add_inkblot_choice(&mut r);
    add_crafting_recipe(&mut r);
    add_counting_items(&mut r);
    add_panorama_match(&mut r);
    add_eye_chart(&mut r);
    add_creative_draw(&mut r);
    add_network_connect(&mut r);
    add_trading_timing(&mut r);
    add_text_choice(&mut r);
    add_sliding_puzzle_v2(&mut r);
    add_traffic_signal(&mut r);
    add_rhythm_pattern(&mut r);
    add_brand_logo(&mut r);
    add_math_solver(&mut r);
    add_shuffle_tracking(&mut r);
    add_match3_game(&mut r);
    add_odd_one_out(&mut r);
    add_decision_choice(&mut r);
    add_face_matching(&mut r);
    add_slot_machine(&mut r);
    add_dig_find(&mut r);
    add_turing_text(&mut r);
    add_assembly_id(&mut r);
    add_chess_challenge(&mut r);
    add_find_person(&mut r);
    add_floor_nav(&mut r);
    add_bell_pattern(&mut r);
    add_final_creative(&mut r);

    r
}

// ─── Original 6 skills ──────────────────────────────────────────────────

/// Add image grid selection skill (e.g., "select all stop signs").
pub fn add_image_grid(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new(
            "image-grid-selection",
            "Select matching images from a grid challenge",
        )
        .with_trigger(SkillTrigger::html_contains("grid-item"))
        .with_trigger(SkillTrigger::html_contains("challenge-grid"))
        .with_trigger(SkillTrigger::title_contains("select all"))
        .with_priority(5)
        .with_content(include_str!("../skills/automation/image_grid.md")),
    );
}

/// Add text CAPTCHA / distorted text skill.
pub fn add_text_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new(
            "text-captcha",
            "Solve text-based CAPTCHAs, distorted text, and math challenges",
        )
        .with_trigger(SkillTrigger::html_contains("captcha-input"))
        .with_trigger(SkillTrigger::html_contains("captcha-text"))
        .with_trigger(SkillTrigger::title_contains("wiggles"))
        .with_priority(3)
        .with_content(include_str!("../skills/automation/text_captcha.md")),
    );
}

/// Add rotation puzzle skill.
pub fn add_rotation_puzzle(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new(
            "rotation-puzzle",
            "Rotate an image or element to the correct orientation",
        )
        .with_trigger(SkillTrigger::title_contains("rotat"))
        .with_trigger(SkillTrigger::html_contains("rotating-item"))
        .with_priority(5)
        .with_content(include_str!("../skills/automation/rotation.md")),
    );
}

/// Add tic-tac-toe / XOXO skill.
pub fn add_tic_tac_toe(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new(
            "tic-tac-toe",
            "Play tic-tac-toe (noughts and crosses) game",
        )
        .with_trigger(SkillTrigger::title_contains("xoxo"))
        .with_trigger(SkillTrigger::title_contains("tic-tac"))
        .with_trigger(SkillTrigger::title_contains("tic tac"))
        .with_trigger(SkillTrigger::html_contains("tic-tac"))
        .with_trigger(SkillTrigger::html_contains("cell-selected"))
        .with_trigger(SkillTrigger::html_contains("cell-disabled"))
        .with_priority(10)
        .with_content(include_str!("../skills/automation/tic_tac_toe.md")),
    );
}

/// Add word search skill.
pub fn add_word_search(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new(
            "word-search",
            "Find and select words in a letter grid",
        )
        .with_trigger(SkillTrigger::title_contains("word search"))
        .with_trigger(SkillTrigger::title_contains("wordsearch"))
        .with_trigger(SkillTrigger::html_contains("word-search-grid-item"))
        .with_trigger(SkillTrigger::html_contains("word-search"))
        .with_priority(8)
        .with_content(include_str!("../skills/automation/word_search.md")),
    );
}

/// Add slider / drag challenge skill.
pub fn add_slider_drag(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new(
            "slider-drag",
            "Solve slider and drag-to-position challenges",
        )
        .with_trigger(SkillTrigger::html_contains("slider-track"))
        .with_trigger(SkillTrigger::html_contains("slider-handle"))
        .with_trigger(SkillTrigger::html_contains("range-slider"))
        .with_priority(4)
        .with_content(include_str!("../skills/automation/slider_drag.md")),
    );
}

// ─── CAPTCHA Variants ───────────────────────────────────────────────────

/// Add audio CAPTCHA skill.
pub fn add_audio_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("audio-captcha", "Handle audio-based CAPTCHA challenges")
            .with_trigger(SkillTrigger::html_contains("audio-captcha"))
            .with_trigger(SkillTrigger::html_contains("audiochallenge"))
            .with_trigger(SkillTrigger::title_contains("audio captcha"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/audio_captcha.md")),
    );
}

/// Add puzzle piece / jigsaw CAPTCHA skill.
pub fn add_puzzle_piece_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("puzzle-piece-captcha", "Solve puzzle piece slide-to-fit CAPTCHAs")
            .with_trigger(SkillTrigger::html_contains("puzzle-piece"))
            .with_trigger(SkillTrigger::html_contains("jigsaw-captcha"))
            .with_trigger(SkillTrigger::html_contains("slide-captcha"))
            .with_trigger(SkillTrigger::html_contains("captcha-slider"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/puzzle_piece_captcha.md")),
    );
}

/// Add honeypot CAPTCHA detection skill.
pub fn add_honeypot_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("honeypot-captcha", "Detect and avoid honeypot form traps")
            .with_trigger(SkillTrigger::html_contains("honeypot"))
            .with_trigger(SkillTrigger::html_contains("hp-field"))
            .with_priority(2)
            .with_content(include_str!("../skills/automation/honeypot_captcha.md")),
    );
}

/// Add math CAPTCHA skill.
pub fn add_math_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("math-captcha", "Solve math-based CAPTCHA challenges")
            .with_trigger(SkillTrigger::html_contains("math-captcha"))
            .with_trigger(SkillTrigger::title_contains("math captcha"))
            .with_trigger(SkillTrigger::html_contains("arithmetic"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/math_captcha.md")),
    );
}

/// Add visual pattern CAPTCHA skill.
pub fn add_visual_pattern_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("visual-pattern-captcha", "Solve visual pattern recognition CAPTCHAs")
            .with_trigger(SkillTrigger::html_contains("pattern-captcha"))
            .with_trigger(SkillTrigger::title_contains("odd one out"))
            .with_trigger(SkillTrigger::title_contains("which is different"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/visual_pattern_captcha.md")),
    );
}

/// Add reCAPTCHA v2 skill.
pub fn add_recaptcha_v2(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("recaptcha-v2", "Handle reCAPTCHA v2 checkbox and image challenges")
            .with_trigger(SkillTrigger::html_contains("g-recaptcha"))
            .with_trigger(SkillTrigger::html_contains("recaptcha"))
            .with_trigger(SkillTrigger::html_contains("rc-anchor"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/recaptcha_v2.md")),
    );
}

/// Add reCAPTCHA v3 skill.
pub fn add_recaptcha_v3(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("recaptcha-v3", "Handle invisible reCAPTCHA v3 score-based checks")
            .with_trigger(SkillTrigger::html_contains("grecaptcha-badge"))
            .with_trigger(SkillTrigger::html_contains("recaptcha/api.js?render"))
            .with_priority(3)
            .with_content(include_str!("../skills/automation/recaptcha_v3.md")),
    );
}

/// Add hCaptcha skill.
pub fn add_hcaptcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("hcaptcha", "Handle hCaptcha challenges")
            .with_trigger(SkillTrigger::html_contains("hcaptcha"))
            .with_trigger(SkillTrigger::html_contains("h-captcha"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/hcaptcha.md")),
    );
}

/// Add Cloudflare Turnstile skill.
pub fn add_cloudflare_turnstile(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("cloudflare-turnstile", "Handle Cloudflare Turnstile challenges")
            .with_trigger(SkillTrigger::html_contains("cf-turnstile"))
            .with_trigger(SkillTrigger::html_contains("challenges.cloudflare"))
            .with_trigger(SkillTrigger::html_contains("turnstile"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/cloudflare_turnstile.md")),
    );
}

/// Add GeeTest CAPTCHA skill.
pub fn add_geetest(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("geetest", "Handle GeeTest slide/click/match CAPTCHAs")
            .with_trigger(SkillTrigger::html_contains("geetest"))
            .with_trigger(SkillTrigger::html_contains("gt_slider"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/geetest.md")),
    );
}

/// Add Arkose Labs / FunCaptcha skill.
pub fn add_arkose_funcaptcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("arkose-funcaptcha", "Handle Arkose Labs / FunCaptcha game challenges")
            .with_trigger(SkillTrigger::html_contains("arkoselabs"))
            .with_trigger(SkillTrigger::html_contains("funcaptcha"))
            .with_trigger(SkillTrigger::html_contains("arkose"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/arkose_funcaptcha.md")),
    );
}

/// Add text-in-image CAPTCHA skill.
pub fn add_text_in_image_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("text-in-image-captcha", "Solve CAPTCHAs with text embedded in noisy images")
            .with_trigger(SkillTrigger::html_contains("captcha-image"))
            .with_trigger(SkillTrigger::html_contains("captchaImg"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/text_in_image_captcha.md")),
    );
}

/// Add sequence ordering CAPTCHA skill.
pub fn add_sequence_ordering_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("sequence-ordering-captcha", "Solve ordering/sequencing CAPTCHA challenges")
            .with_trigger(SkillTrigger::html_contains("sortable-captcha"))
            .with_trigger(SkillTrigger::title_contains("arrange in order"))
            .with_trigger(SkillTrigger::title_contains("put in order"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/sequence_ordering_captcha.md")),
    );
}

/// Add image rotation CAPTCHA skill.
pub fn add_image_rotation_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("image-rotation-captcha", "Solve image rotation CAPTCHA challenges")
            .with_trigger(SkillTrigger::html_contains("rotate-captcha"))
            .with_trigger(SkillTrigger::title_contains("rotate the image"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/image_rotation_captcha.md")),
    );
}

/// Add icon/symbol CAPTCHA skill.
pub fn add_icon_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("icon-captcha", "Solve icon/symbol selection CAPTCHA challenges")
            .with_trigger(SkillTrigger::html_contains("icon-captcha"))
            .with_trigger(SkillTrigger::html_contains("IconCaptcha"))
            .with_trigger(SkillTrigger::title_contains("click the icon"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/icon_captcha.md")),
    );
}

/// Add semantic/question CAPTCHA skill.
pub fn add_semantic_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("semantic-captcha", "Solve question-based semantic CAPTCHAs")
            .with_trigger(SkillTrigger::html_contains("question-captcha"))
            .with_trigger(SkillTrigger::html_contains("text-captcha-question"))
            .with_priority(3)
            .with_content(include_str!("../skills/automation/semantic_captcha.md")),
    );
}

/// Add click-in-order CAPTCHA skill.
pub fn add_click_order_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("click-order-captcha", "Solve click-in-sequence CAPTCHA challenges")
            .with_trigger(SkillTrigger::title_contains("click in order"))
            .with_trigger(SkillTrigger::html_contains("click-order"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/click_order_captcha.md")),
    );
}

/// Add 3D object CAPTCHA skill.
pub fn add_captcha_3d_object(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("captcha-3d-object", "Solve 3D object rotation/identification CAPTCHAs")
            .with_trigger(SkillTrigger::html_contains("3d-captcha"))
            .with_trigger(SkillTrigger::html_contains("captcha-3d"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/captcha_3d_object.md")),
    );
}

/// Add reCAPTCHA audio fallback skill.
pub fn add_captcha_audio_v2(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("captcha-audio-v2", "Handle reCAPTCHA audio mode fallback")
            .with_trigger(SkillTrigger::html_contains("rc-audiochallenge"))
            .with_trigger(SkillTrigger::html_contains("audio-response"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/captcha_audio_v2.md")),
    );
}

// ─── Puzzle Games ───────────────────────────────────────────────────────

/// Add jigsaw puzzle skill.
pub fn add_jigsaw_puzzle(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("jigsaw-puzzle", "Solve jigsaw puzzle challenges")
            .with_trigger(SkillTrigger::title_contains("jigsaw"))
            .with_trigger(SkillTrigger::html_contains("jigsaw"))
            .with_trigger(SkillTrigger::html_contains("puzzle-piece"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/jigsaw_puzzle.md")),
    );
}

/// Add sliding tile puzzle skill.
pub fn add_sliding_tile_puzzle(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("sliding-tile-puzzle", "Solve sliding tile (15-puzzle) challenges")
            .with_trigger(SkillTrigger::title_contains("sliding puzzle"))
            .with_trigger(SkillTrigger::title_contains("15 puzzle"))
            .with_trigger(SkillTrigger::title_contains("8 puzzle"))
            .with_trigger(SkillTrigger::html_contains("sliding-puzzle"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/sliding_tile_puzzle.md")),
    );
}

/// Add maze solving skill.
pub fn add_maze_solving(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("maze-solving", "Navigate and solve maze challenges")
            .with_trigger(SkillTrigger::title_contains("maze"))
            .with_trigger(SkillTrigger::html_contains("maze"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/maze_solving.md")),
    );
}

/// Add sudoku skill.
pub fn add_sudoku(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("sudoku", "Solve sudoku puzzle challenges")
            .with_trigger(SkillTrigger::title_contains("sudoku"))
            .with_trigger(SkillTrigger::html_contains("sudoku"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/sudoku.md")),
    );
}

/// Add crossword puzzle skill.
pub fn add_crossword(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("crossword", "Solve crossword puzzle challenges")
            .with_trigger(SkillTrigger::title_contains("crossword"))
            .with_trigger(SkillTrigger::html_contains("crossword"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/crossword.md")),
    );
}

/// Add connect-the-dots skill.
pub fn add_connect_the_dots(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("connect-the-dots", "Solve connect-the-dots challenges")
            .with_trigger(SkillTrigger::title_contains("connect the dots"))
            .with_trigger(SkillTrigger::html_contains("connect-dots"))
            .with_trigger(SkillTrigger::title_contains("dot to dot"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/connect_the_dots.md")),
    );
}

/// Add pattern matching skill.
pub fn add_pattern_matching(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("pattern-matching", "Solve pattern matching/pair challenges")
            .with_trigger(SkillTrigger::title_contains("match"))
            .with_trigger(SkillTrigger::title_contains("pattern"))
            .with_trigger(SkillTrigger::html_contains("matching-game"))
            .with_priority(3)
            .with_content(include_str!("../skills/automation/pattern_matching.md")),
    );
}

/// Add memory card game skill.
pub fn add_memory_card_game(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("memory-card-game", "Solve memory card matching game challenges")
            .with_trigger(SkillTrigger::title_contains("memory game"))
            .with_trigger(SkillTrigger::title_contains("memory card"))
            .with_trigger(SkillTrigger::html_contains("memory-game"))
            .with_trigger(SkillTrigger::html_contains("card-flip"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/memory_card_game.md")),
    );
}

/// Add number sequence skill.
pub fn add_number_sequence(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("number-sequence", "Solve number sequence/pattern challenges")
            .with_trigger(SkillTrigger::title_contains("number sequence"))
            .with_trigger(SkillTrigger::title_contains("next number"))
            .with_trigger(SkillTrigger::html_contains("sequence-puzzle"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/number_sequence.md")),
    );
}

/// Add color matching skill.
pub fn add_color_matching(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("color-matching", "Solve color matching/identification challenges")
            .with_trigger(SkillTrigger::title_contains("color match"))
            .with_trigger(SkillTrigger::title_contains("colour match"))
            .with_trigger(SkillTrigger::html_contains("color-match"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/color_matching.md")),
    );
}

/// Add shape sorting skill.
pub fn add_shape_sorting(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("shape-sorting", "Solve shape sorting/categorization challenges")
            .with_trigger(SkillTrigger::title_contains("shape sort"))
            .with_trigger(SkillTrigger::html_contains("shape-sort"))
            .with_trigger(SkillTrigger::title_contains("sort the shapes"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/shape_sorting.md")),
    );
}

/// Add spot the difference skill.
pub fn add_spot_the_difference(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("spot-the-difference", "Find differences between two images")
            .with_trigger(SkillTrigger::title_contains("spot the difference"))
            .with_trigger(SkillTrigger::title_contains("find the difference"))
            .with_trigger(SkillTrigger::html_contains("spot-difference"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/spot_the_difference.md")),
    );
}

/// Add object counting skill.
pub fn add_object_counting(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("object-counting", "Count objects in an image challenge")
            .with_trigger(SkillTrigger::title_contains("how many"))
            .with_trigger(SkillTrigger::title_contains("count the"))
            .with_trigger(SkillTrigger::html_contains("counting-challenge"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/object_counting.md")),
    );
}

// ─── Form Interactions ──────────────────────────────────────────────────

/// Add multi-step form wizard skill.
pub fn add_multi_step_form(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("multi-step-form", "Navigate multi-step form wizards")
            .with_trigger(SkillTrigger::html_contains("step-wizard"))
            .with_trigger(SkillTrigger::html_contains("form-wizard"))
            .with_trigger(SkillTrigger::html_contains("multi-step"))
            .with_priority(3)
            .with_content(include_str!("../skills/automation/multi_step_form.md")),
    );
}

/// Add file upload skill.
pub fn add_file_upload(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("file-upload", "Handle file upload dialog interactions")
            .with_trigger(SkillTrigger::html_contains("file-upload"))
            .with_trigger(SkillTrigger::html_contains("dropzone"))
            .with_trigger(SkillTrigger::html_contains("upload-area"))
            .with_priority(3)
            .with_content(include_str!("../skills/automation/file_upload.md")),
    );
}

/// Add address form skill.
pub fn add_address_form(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("address-form", "Complete address forms with proper field mapping")
            .with_trigger(SkillTrigger::html_contains("address-form"))
            .with_trigger(SkillTrigger::html_contains("shipping-address"))
            .with_trigger(SkillTrigger::html_contains("billing-address"))
            .with_priority(3)
            .with_content(include_str!("../skills/automation/address_form.md")),
    );
}

/// Add payment form skill.
pub fn add_payment_form(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("payment-form", "Handle payment form and card input interactions")
            .with_trigger(SkillTrigger::html_contains("payment-form"))
            .with_trigger(SkillTrigger::html_contains("StripeElement"))
            .with_trigger(SkillTrigger::html_contains("card-number"))
            .with_trigger(SkillTrigger::html_contains("braintree"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/payment_form.md")),
    );
}

/// Add form validation handling skill.
pub fn add_form_validation(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("form-validation", "Handle form validation errors and fix them")
            .with_trigger(SkillTrigger::html_contains("field-error"))
            .with_trigger(SkillTrigger::html_contains("validation-error"))
            .with_trigger(SkillTrigger::html_contains("form-error"))
            .with_priority(2)
            .with_content(include_str!("../skills/automation/form_validation.md")),
    );
}

/// Add OTP input skill.
pub fn add_otp_input(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("otp-input", "Handle OTP/verification code input fields")
            .with_trigger(SkillTrigger::html_contains("otp-input"))
            .with_trigger(SkillTrigger::html_contains("verification-code"))
            .with_trigger(SkillTrigger::title_contains("enter code"))
            .with_trigger(SkillTrigger::title_contains("verification"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/otp_input.md")),
    );
}

/// Add web form auto-fill skill.
pub fn add_web_form_autofill(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("web-form-autofill", "Systematically fill web forms using field metadata")
            .with_trigger(SkillTrigger::html_contains("registration-form"))
            .with_trigger(SkillTrigger::html_contains("signup-form"))
            .with_priority(2)
            .with_content(include_str!("../skills/automation/web_form_autofill.md")),
    );
}

// ─── Navigation Challenges ──────────────────────────────────────────────

/// Add infinite scroll skill.
pub fn add_infinite_scroll(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("infinite-scroll", "Handle infinite scroll and lazy-loaded content pages")
            .with_trigger(SkillTrigger::html_contains("infinite-scroll"))
            .with_trigger(SkillTrigger::html_contains("load-more"))
            .with_trigger(SkillTrigger::html_contains("infinite-loading"))
            .with_priority(3)
            .with_content(include_str!("../skills/automation/infinite_scroll.md")),
    );
}

/// Add popup/modal handling skill.
pub fn add_popup_modal(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("popup-modal", "Handle and dismiss popups, modals, and overlays")
            .with_trigger(SkillTrigger::html_contains("modal-open"))
            .with_trigger(SkillTrigger::html_contains("modal-backdrop"))
            .with_trigger(SkillTrigger::html_contains("popup-overlay"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/popup_modal.md")),
    );
}

/// Add cookie consent banner skill.
pub fn add_cookie_consent(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("cookie-consent", "Handle cookie consent banners and GDPR notices")
            .with_trigger(SkillTrigger::html_contains("cookie-consent"))
            .with_trigger(SkillTrigger::html_contains("cookie-banner"))
            .with_trigger(SkillTrigger::html_contains("gdpr"))
            .with_trigger(SkillTrigger::html_contains("onetrust"))
            .with_trigger(SkillTrigger::html_contains("cookiebot"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/cookie_consent.md")),
    );
}

/// Add age verification gate skill.
pub fn add_age_verification(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("age-verification", "Handle age verification gate pages")
            .with_trigger(SkillTrigger::html_contains("age-gate"))
            .with_trigger(SkillTrigger::html_contains("age-verification"))
            .with_trigger(SkillTrigger::title_contains("age verification"))
            .with_trigger(SkillTrigger::title_contains("are you 21"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/age_verification.md")),
    );
}

/// Add login wall skill.
pub fn add_login_wall(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("login-wall", "Handle login walls and authentication gates")
            .with_trigger(SkillTrigger::html_contains("login-form"))
            .with_trigger(SkillTrigger::html_contains("signin-form"))
            .with_trigger(SkillTrigger::title_contains("sign in"))
            .with_trigger(SkillTrigger::title_contains("log in"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/login_wall.md")),
    );
}

/// Add paywall detection skill.
pub fn add_paywall_detection(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("paywall-detection", "Detect and navigate around paywalls")
            .with_trigger(SkillTrigger::html_contains("paywall"))
            .with_trigger(SkillTrigger::html_contains("subscribe-wall"))
            .with_trigger(SkillTrigger::html_contains("premium-content"))
            .with_priority(3)
            .with_content(include_str!("../skills/automation/paywall_detection.md")),
    );
}

/// Add redirect chain handling skill.
pub fn add_redirect_chain(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("redirect-chain", "Handle redirect chains and interstitial pages")
            .with_trigger(SkillTrigger::html_contains("redirect"))
            .with_trigger(SkillTrigger::title_contains("redirecting"))
            .with_trigger(SkillTrigger::title_contains("please wait"))
            .with_priority(3)
            .with_content(include_str!("../skills/automation/redirect_chain.md")),
    );
}

/// Add iframe interaction skill.
pub fn add_iframe_interaction(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("iframe-interaction", "Interact with content inside iframes")
            .with_trigger(SkillTrigger::html_contains("<iframe"))
            .with_priority(2)
            .with_content(include_str!("../skills/automation/iframe_interaction.md")),
    );
}

/// Add lazy-loaded content skill.
pub fn add_lazy_loaded_content(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("lazy-loaded-content", "Handle lazy-loaded images and content")
            .with_trigger(SkillTrigger::html_contains("loading=\"lazy\""))
            .with_trigger(SkillTrigger::html_contains("data-src"))
            .with_trigger(SkillTrigger::html_contains("lazyload"))
            .with_priority(2)
            .with_content(include_str!("../skills/automation/lazy_loaded_content.md")),
    );
}

/// Add pagination navigation skill.
pub fn add_pagination_navigation(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("pagination-navigation", "Navigate paginated content")
            .with_trigger(SkillTrigger::html_contains("pagination"))
            .with_trigger(SkillTrigger::html_contains("page-nav"))
            .with_priority(2)
            .with_content(include_str!("../skills/automation/pagination_navigation.md")),
    );
}

// ─── Visual Challenges ──────────────────────────────────────────────────

/// Add drag-and-drop sorting skill.
pub fn add_drag_drop_sorting(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("drag-drop-sorting", "Handle drag-and-drop sorting/reordering")
            .with_trigger(SkillTrigger::html_contains("sortable"))
            .with_trigger(SkillTrigger::html_contains("draggable"))
            .with_trigger(SkillTrigger::title_contains("drag and drop"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/drag_drop_sorting.md")),
    );
}

/// Add chart data extraction skill.
pub fn add_chart_data_extraction(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("chart-data-extraction", "Extract data from charts and graphs")
            .with_trigger(SkillTrigger::html_contains("highcharts"))
            .with_trigger(SkillTrigger::html_contains("chart-container"))
            .with_trigger(SkillTrigger::html_contains("chartjs"))
            .with_priority(3)
            .with_content(include_str!("../skills/automation/chart_data_extraction.md")),
    );
}

/// Add drag-to-target skill.
pub fn add_drag_to_target(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("drag-to-target", "Drag items to specific drop zone targets")
            .with_trigger(SkillTrigger::html_contains("drop-zone"))
            .with_trigger(SkillTrigger::html_contains("dropzone"))
            .with_trigger(SkillTrigger::html_contains("drag-target"))
            .with_priority(4)
            .with_content(include_str!("../skills/automation/drag_to_target.md")),
    );
}

// ─── Data Extraction ────────────────────────────────────────────────────

/// Add structured table extraction skill.
pub fn add_table_extraction(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("table-extraction", "Extract structured data from tables")
            .with_trigger(SkillTrigger::html_contains("<table"))
            .with_priority(1)
            .with_content(include_str!("../skills/automation/table_extraction.md")),
    );
}

/// Add product listing extraction skill.
pub fn add_product_listing(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("product-listing", "Extract product listing data")
            .with_trigger(SkillTrigger::html_contains("product-card"))
            .with_trigger(SkillTrigger::html_contains("product-list"))
            .with_trigger(SkillTrigger::html_contains("product-grid"))
            .with_priority(2)
            .with_content(include_str!("../skills/automation/product_listing.md")),
    );
}

/// Add contact info extraction skill.
pub fn add_contact_extraction(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("contact-extraction", "Extract contact information from pages")
            .with_trigger(SkillTrigger::title_contains("contact"))
            .with_trigger(SkillTrigger::html_contains("contact-info"))
            .with_trigger(SkillTrigger::url_contains("contact"))
            .with_priority(2)
            .with_content(include_str!("../skills/automation/contact_extraction.md")),
    );
}

/// Add search results extraction skill.
pub fn add_search_results(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("search-results", "Navigate and extract search results")
            .with_trigger(SkillTrigger::html_contains("search-results"))
            .with_trigger(SkillTrigger::html_contains("search-listing"))
            .with_trigger(SkillTrigger::title_contains("search results"))
            .with_priority(2)
            .with_content(include_str!("../skills/automation/search_results.md")),
    );
}

/// Add price scraping skill.
pub fn add_price_scraping(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("price-scraping", "Extract price and currency data")
            .with_trigger(SkillTrigger::html_contains("price-tag"))
            .with_trigger(SkillTrigger::html_contains("product-price"))
            .with_priority(2)
            .with_content(include_str!("../skills/automation/price_scraping.md")),
    );
}

// ─── Security Challenges ────────────────────────────────────────────────

/// Add rate limiting detection skill.
pub fn add_rate_limiting(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("rate-limiting", "Detect and handle rate limiting")
            .with_trigger(SkillTrigger::title_contains("429"))
            .with_trigger(SkillTrigger::title_contains("too many requests"))
            .with_trigger(SkillTrigger::title_contains("rate limit"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/rate_limiting.md")),
    );
}

/// Add bot detection avoidance skill.
pub fn add_bot_detection(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("bot-detection", "Detect and handle bot detection systems")
            .with_trigger(SkillTrigger::html_contains("datadome"))
            .with_trigger(SkillTrigger::html_contains("perimeter"))
            .with_trigger(SkillTrigger::html_contains("kasada"))
            .with_trigger(SkillTrigger::html_contains("distil"))
            .with_trigger(SkillTrigger::title_contains("access denied"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/bot_detection.md")),
    );
}

/// Add browser fingerprint challenge skill.
pub fn add_fingerprint_challenge(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("fingerprint-challenge", "Handle browser fingerprint verification")
            .with_trigger(SkillTrigger::html_contains("fingerprintjs"))
            .with_trigger(SkillTrigger::html_contains("fpjs"))
            .with_priority(3)
            .with_content(include_str!("../skills/automation/fingerprint_challenge.md")),
    );
}

/// Add proof-of-work challenge skill.
pub fn add_proof_of_work(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("proof-of-work", "Handle proof-of-work browser challenges")
            .with_trigger(SkillTrigger::html_contains("proof-of-work"))
            .with_trigger(SkillTrigger::html_contains("hashcash"))
            .with_trigger(SkillTrigger::title_contains("computing"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/proof_of_work.md")),
    );
}

/// Add JavaScript challenge page skill.
pub fn add_js_challenge_page(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("js-challenge-page", "Handle JavaScript challenge interstitial pages")
            .with_trigger(SkillTrigger::title_contains("checking your browser"))
            .with_trigger(SkillTrigger::title_contains("just a moment"))
            .with_trigger(SkillTrigger::html_contains("cf-browser-verification"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/js_challenge_page.md")),
    );
}

/// Add device verification skill.
pub fn add_device_verification(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("device-verification", "Handle device verification and trusted device challenges")
            .with_trigger(SkillTrigger::html_contains("device-verification"))
            .with_trigger(SkillTrigger::html_contains("trusted-device"))
            .with_trigger(SkillTrigger::title_contains("verify your device"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/device_verification.md")),
    );
}

// ─── Interactive Challenge Skills ────────────────────────────────────────

/// Add simple checkbox click skill.
pub fn add_checkbox_click(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("checkbox-click", "Click a checkbox to prove you are human")
            .with_trigger(SkillTrigger::html_contains("captcha-checkbox"))
            .with_trigger(SkillTrigger::html_contains("checkbox-label"))
            .with_priority(2)
            .with_content(include_str!("../skills/automation/checkbox_click.md")),
    );
}

/// Add license plate reading skill.
pub fn add_license_plate(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("license-plate", "Read a license plate from an image and type it")
            .with_trigger(SkillTrigger::title_contains("license"))
            .with_trigger(SkillTrigger::title_contains("plate"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/license_plate.md")),
    );
}

/// Add nested grid subdivision skill.
pub fn add_nested_grid(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("nested-grid", "Recursive grid where clicking correct squares subdivides them")
            .with_trigger(SkillTrigger::title_contains("nested"))
            .with_trigger(SkillTrigger::html_contains("nested-container"))
            .with_priority(9)
            .with_content(include_str!("../skills/automation/nested_grid.md")),
    );
}

/// Add whack-a-mole skill.
pub fn add_whack_a_mole(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("whack-a-mole", "Click moles as they pop up in a grid")
            .with_trigger(SkillTrigger::title_contains("whack"))
            .with_trigger(SkillTrigger::title_contains("mole"))
            .with_priority(9)
            .with_content(include_str!("../skills/automation/whack_a_mole.md")),
    );
}

/// Add find Waldo skill.
pub fn add_find_waldo(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("find-waldo", "Find Waldo in a crowded Where's Waldo scene")
            .with_trigger(SkillTrigger::title_contains("waldo"))
            .with_priority(9)
            .with_content(include_str!("../skills/automation/find_waldo.md")),
    );
}

/// Add chihuahua vs muffin skill.
pub fn add_chihuahua_muffin(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("chihuahua-muffin", "Distinguish chihuahuas from muffins in image grid")
            .with_trigger(SkillTrigger::title_contains("muffin"))
            .with_trigger(SkillTrigger::title_contains("chihuahua"))
            .with_priority(9)
            .with_content(include_str!("../skills/automation/chihuahua_muffin.md")),
    );
}

/// Add reverse selection skill.
pub fn add_reverse_selection(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("reverse-selection", "Select images that do NOT contain the specified object")
            .with_trigger(SkillTrigger::title_contains("reverse"))
            .with_priority(9)
            .with_content(include_str!("../skills/automation/reverse_selection.md")),
    );
}

/// Add affirmations skill.
pub fn add_affirmations(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("affirmations", "Find and select the captcha with specific text")
            .with_trigger(SkillTrigger::title_contains("affirm"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/affirmations.md")),
    );
}

/// Add parking challenge skill.
pub fn add_parking_challenge(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("parking-challenge", "Navigate or drag an object into a target zone")
            .with_trigger(SkillTrigger::title_contains("parking"))
            .with_trigger(SkillTrigger::html_contains("parking"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/parking_challenge.md")),
    );
}

/// Add 3D object identification skill.
pub fn add_three_d_object(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("3d-object", "Identify objects from 3D perspective")
            .with_trigger(SkillTrigger::title_contains("3d"))
            .with_trigger(SkillTrigger::title_contains("3D"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/three_d_object.md")),
    );
}

/// Add draw circle skill.
pub fn add_draw_circle(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("draw-circle", "Draw a shape by tracing a mouse path")
            .with_trigger(SkillTrigger::title_contains("circle"))
            .with_trigger(SkillTrigger::title_contains("draw"))
            .with_priority(8)
            .with_content(include_str!("../skills/automation/draw_circle.md")),
    );
}

/// Add push/drag skill.
pub fn add_push_drag(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("push-drag", "Repeatedly drag an object in a direction")
            .with_trigger(SkillTrigger::title_contains("sisyphus"))
            .with_trigger(SkillTrigger::html_contains("boulder"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/push_drag.md")),
    );
}

/// Add dark/hidden element skill.
pub fn add_dark_hidden(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("dark-hidden", "Find elements hidden in darkness")
            .with_trigger(SkillTrigger::title_contains("dark"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/dark_hidden.md")),
    );
}

/// Add inkblot/interpretation skill.
pub fn add_inkblot_choice(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("inkblot-choice", "Interpret or choose from visual prompts")
            .with_trigger(SkillTrigger::title_contains("rorschach"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/inkblot_choice.md")),
    );
}

/// Add crafting recipe skill.
pub fn add_crafting_recipe(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("crafting-recipe", "Solve crafting or assembly challenges")
            .with_trigger(SkillTrigger::title_contains("craft"))
            .with_trigger(SkillTrigger::html_contains("craft"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/crafting_recipe.md")),
    );
}

/// Add counting/arrangement skill.
pub fn add_counting_items(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("counting-items", "Count or arrange items in correct order")
            .with_trigger(SkillTrigger::title_contains("duck"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/counting_items.md")),
    );
}

/// Add panorama matching skill.
pub fn add_panorama_match(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("panorama-match", "Match or reorder panoramic image segments")
            .with_trigger(SkillTrigger::title_contains("panora"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/panorama_match.md")),
    );
}

/// Add eye chart skill.
pub fn add_eye_chart(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("eye-chart", "Read text from decreasing-size visual display")
            .with_trigger(SkillTrigger::title_contains("eye"))
            .with_trigger(SkillTrigger::title_contains("exam"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/eye_chart.md")),
    );
}

/// Add creative drawing skill.
pub fn add_creative_draw(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("creative-draw", "Draw or create something original")
            .with_trigger(SkillTrigger::title_contains("creativ"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/creative_draw.md")),
    );
}

/// Add network connection skill.
pub fn add_network_connect(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("network-connect", "Connect nodes or items together")
            .with_trigger(SkillTrigger::title_contains("network"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/network_connect.md")),
    );
}

/// Add trading/timing skill.
pub fn add_trading_timing(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("trading-timing", "Buy and sell at the right time")
            .with_trigger(SkillTrigger::title_contains("trader"))
            .with_trigger(SkillTrigger::title_contains("trading"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/trading_timing.md")),
    );
}

/// Add text choice skill.
pub fn add_text_choice(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("text-choice", "Make a text-based choice or response")
            .with_trigger(SkillTrigger::title_contains("soul"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/text_choice.md")),
    );
}

/// Add sliding puzzle v2 skill.
pub fn add_sliding_puzzle_v2(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("sliding-puzzle", "Solve a sliding tile puzzle")
            .with_trigger(SkillTrigger::title_contains("sliding"))
            .with_trigger(SkillTrigger::html_contains("sliding"))
            .with_trigger(SkillTrigger::html_contains("puzzle-grid"))
            .with_priority(8)
            .with_content(include_str!("../skills/automation/sliding_puzzle_v2.md")),
    );
}

/// Add traffic signal skill.
pub fn add_traffic_signal(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("traffic-signal", "Interact with traffic signals or trees")
            .with_trigger(SkillTrigger::title_contains("traffic"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/traffic_signal.md")),
    );
}

/// Add rhythm/drum pattern skill.
pub fn add_rhythm_pattern(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("rhythm-pattern", "Reproduce a rhythm or sound pattern")
            .with_trigger(SkillTrigger::title_contains("drum"))
            .with_trigger(SkillTrigger::title_contains("rhythm"))
            .with_priority(7)
            .with_content(include_str!("../skills/automation/rhythm_pattern.md")),
    );
}

/// Add brand logo identification skill.
pub fn add_brand_logo(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("brand-logo", "Identify brand logos from images")
            .with_trigger(SkillTrigger::title_contains("brand"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/brand_logo.md")),
    );
}

/// Add math solver skill.
pub fn add_math_solver(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("math-solver", "Solve mathematical equations or expressions")
            .with_trigger(SkillTrigger::title_contains("math"))
            .with_trigger(SkillTrigger::title_contains("equation"))
            .with_priority(7)
            .with_content(include_str!("../skills/automation/math_solver.md")),
    );
}

/// Add shuffle tracking skill.
pub fn add_shuffle_tracking(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("card-tracking", "Track an object through movement or shuffle")
            .with_trigger(SkillTrigger::title_contains("shuffle"))
            .with_priority(7)
            .with_content(include_str!("../skills/automation/shuffle_tracking.md")),
    );
}

/// Add match-3 game skill.
pub fn add_match3_game(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("match3-game", "Solve a match-3 or tile-matching puzzle")
            .with_trigger(SkillTrigger::title_contains("candy"))
            .with_trigger(SkillTrigger::title_contains("crush"))
            .with_trigger(SkillTrigger::title_contains("match"))
            .with_priority(7)
            .with_content(include_str!("../skills/automation/match3_game.md")),
    );
}

/// Add odd-one-out / imposter skill.
pub fn add_odd_one_out(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("odd-one-out", "Find the imposter or odd item among similar ones")
            .with_trigger(SkillTrigger::title_contains("imposter"))
            .with_trigger(SkillTrigger::title_contains("impostor"))
            .with_priority(7)
            .with_content(include_str!("../skills/automation/odd_one_out.md")),
    );
}

/// Add decision/choice skill.
pub fn add_decision_choice(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("decision-choice", "Make a decision between presented options")
            .with_trigger(SkillTrigger::title_contains("decision"))
            .with_trigger(SkillTrigger::title_contains("choose"))
            .with_priority(5)
            .with_content(include_str!("../skills/automation/decision_choice.md")),
    );
}

/// Add face matching skill.
pub fn add_face_matching(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("face-matching", "Match or compare faces")
            .with_trigger(SkillTrigger::title_contains("facial"))
            .with_trigger(SkillTrigger::title_contains("face"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/face_matching.md")),
    );
}

/// Add slot machine skill.
pub fn add_slot_machine(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("slot-machine", "Stop a spinning element at the right moment")
            .with_trigger(SkillTrigger::title_contains("slot"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/slot_machine.md")),
    );
}

/// Add dig/find skill.
pub fn add_dig_find(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("dig-find", "Dig or search for a hidden element")
            .with_trigger(SkillTrigger::title_contains("grave"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/dig_find.md")),
    );
}

/// Add reverse Turing text skill.
pub fn add_turing_text(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("turing-text", "Type a convincing human-like text response")
            .with_trigger(SkillTrigger::title_contains("turing"))
            .with_priority(7)
            .with_content(include_str!("../skills/automation/turing_text.md")),
    );
}

/// Add assembly/identification skill.
pub fn add_assembly_id(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("assembly-id", "Identify items from assembly instructions")
            .with_trigger(SkillTrigger::title_contains("ikea"))
            .with_trigger(SkillTrigger::title_contains("assembl"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/assembly_id.md")),
    );
}

/// Add chess challenge skill.
pub fn add_chess_challenge(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("chess-challenge", "Make the best chess move")
            .with_trigger(SkillTrigger::title_contains("chess"))
            .with_trigger(SkillTrigger::title_contains("grandmaster"))
            .with_trigger(SkillTrigger::html_contains("chess"))
            .with_priority(8)
            .with_content(include_str!("../skills/automation/chess_challenge.md")),
    );
}

/// Add find person skill.
pub fn add_find_person(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("find-person", "Find a specific person in a crowd or grid")
            .with_trigger(SkillTrigger::title_contains("jessica"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/find_person.md")),
    );
}

/// Add floor navigation skill.
pub fn add_floor_nav(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("floor-nav", "Navigate through building floors")
            .with_trigger(SkillTrigger::title_contains("floor"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/floor_nav.md")),
    );
}

/// Add bell/sound pattern skill.
pub fn add_bell_pattern(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("bell-pattern", "Reproduce a bell or sound sequence")
            .with_trigger(SkillTrigger::title_contains("din"))
            .with_trigger(SkillTrigger::title_contains("don"))
            .with_trigger(SkillTrigger::title_contains("dan"))
            .with_priority(7)
            .with_content(include_str!("../skills/automation/bell_pattern.md")),
    );
}

/// Add final creative challenge skill.
pub fn add_final_creative(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new("final-creative", "Complete a creative final challenge")
            .with_trigger(SkillTrigger::title_contains("inventor"))
            .with_priority(6)
            .with_content(include_str!("../skills/automation/final_creative.md")),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_loads_all_skills() {
        let r = registry();
        // 6 original + 19 CAPTCHA + 13 puzzle + 7 form + 10 navigation
        // + 3 visual + 5 data extraction + 6 security + 41 interactive = 110 total skills
        assert!(
            r.len() >= 110,
            "Expected at least 110 skills, got {}",
            r.len()
        );
    }

    #[test]
    fn test_image_grid_matches() {
        let r = registry();
        let ctx = r.match_context("", "", "<div class='grid-item'>img</div>");
        assert!(ctx.contains("image-grid-selection"));
    }

    #[test]
    fn test_tic_tac_toe_matches() {
        let r = registry();
        let ctx = r.match_context("", "XOXO Game", "");
        assert!(ctx.contains("tic-tac-toe"));
    }

    #[test]
    fn test_word_search_matches() {
        let r = registry();
        let ctx = r.match_context("", "Word Search Puzzle", "");
        assert!(ctx.contains("word-search"));
    }

    #[test]
    fn test_text_captcha_matches_wiggles() {
        let r = registry();
        let ctx = r.match_context("", "Wiggles Level", "");
        assert!(ctx.contains("text-captcha"));
    }

    #[test]
    fn test_no_match_on_unrelated_page() {
        let r = registry();
        let ctx = r.match_context("https://example.com", "Home", "<div>hello</div>");
        assert!(ctx.is_empty());
    }

    // ─── CAPTCHA variant tests ───────────────────────────────────────

    #[test]
    fn test_recaptcha_v2_matches() {
        let r = registry();
        let ctx = r.match_context("", "", "<div class='g-recaptcha' data-sitekey='abc'></div>");
        assert!(ctx.contains("recaptcha-v2"));
    }

    #[test]
    fn test_hcaptcha_matches() {
        let r = registry();
        let ctx = r.match_context("", "", "<div class='h-captcha'></div>");
        assert!(ctx.contains("hcaptcha"));
    }

    #[test]
    fn test_cloudflare_turnstile_matches() {
        let r = registry();
        let ctx = r.match_context("", "", "<div class='cf-turnstile'></div>");
        assert!(ctx.contains("cloudflare-turnstile"));
    }

    #[test]
    fn test_geetest_matches() {
        let r = registry();
        let ctx = r.match_context("", "", "<div class='geetest_panel'></div>");
        assert!(ctx.contains("geetest"));
    }

    // ─── Puzzle game tests ───────────────────────────────────────────

    #[test]
    fn test_sudoku_matches() {
        let r = registry();
        let ctx = r.match_context("", "Sudoku Puzzle", "");
        assert!(ctx.contains("sudoku"));
    }

    #[test]
    fn test_maze_matches() {
        let r = registry();
        let ctx = r.match_context("", "Maze Challenge", "");
        assert!(ctx.contains("maze-solving"));
    }

    #[test]
    fn test_crossword_matches() {
        let r = registry();
        let ctx = r.match_context("", "", "<div class='crossword-grid'></div>");
        assert!(ctx.contains("crossword"));
    }

    // ─── Navigation tests ────────────────────────────────────────────

    #[test]
    fn test_cookie_consent_matches() {
        let r = registry();
        let ctx = r.match_context("", "", "<div class='cookie-consent'>Accept cookies?</div>");
        assert!(ctx.contains("cookie-consent"));
    }

    #[test]
    fn test_popup_modal_matches() {
        let r = registry();
        let ctx = r.match_context("", "", "<div class='modal-backdrop'></div>");
        assert!(ctx.contains("popup-modal"));
    }

    #[test]
    fn test_login_wall_matches() {
        let r = registry();
        let ctx = r.match_context("", "Sign In", "");
        assert!(ctx.contains("login-wall"));
    }

    // ─── Security challenge tests ────────────────────────────────────

    #[test]
    fn test_rate_limiting_matches() {
        let r = registry();
        let ctx = r.match_context("", "429 Too Many Requests", "");
        assert!(ctx.contains("rate-limiting"));
    }

    #[test]
    fn test_js_challenge_matches() {
        let r = registry();
        let ctx = r.match_context("", "Just a moment...", "");
        assert!(ctx.contains("js-challenge-page"));
    }

    #[test]
    fn test_bot_detection_matches() {
        let r = registry();
        let ctx = r.match_context("", "Access Denied", "");
        assert!(ctx.contains("bot-detection"));
    }

    // ─── Data extraction tests ───────────────────────────────────────

    #[test]
    fn test_table_extraction_matches() {
        let r = registry();
        let ctx = r.match_context("", "", "<table><tr><td>data</td></tr></table>");
        assert!(ctx.contains("table-extraction"));
    }

    #[test]
    fn test_contact_extraction_matches() {
        let r = registry();
        let ctx = r.match_context("https://example.com/contact", "Contact Us", "");
        assert!(ctx.contains("contact-extraction"));
    }

    // ─── Form interaction tests ──────────────────────────────────────

    #[test]
    fn test_payment_form_matches() {
        let r = registry();
        let ctx = r.match_context("", "", "<div class='StripeElement'></div>");
        assert!(ctx.contains("payment-form"));
    }

    // ─── Visual challenge tests ──────────────────────────────────────

    #[test]
    fn test_drag_drop_sorting_matches() {
        let r = registry();
        let ctx = r.match_context("", "", "<div class='sortable-list'><div draggable></div></div>");
        assert!(ctx.contains("drag-drop-sorting"));
    }

    #[test]
    fn test_all_skills_have_content() {
        let r = registry();
        // Verify no skill has empty content
        for name in [
            "image-grid-selection", "text-captcha", "rotation-puzzle",
            "tic-tac-toe", "word-search", "slider-drag",
            "audio-captcha", "puzzle-piece-captcha", "honeypot-captcha",
            "recaptcha-v2", "hcaptcha", "cloudflare-turnstile",
            "geetest", "arkose-funcaptcha", "sudoku", "crossword",
            "maze-solving", "jigsaw-puzzle", "memory-card-game",
            "cookie-consent", "popup-modal", "login-wall",
            "rate-limiting", "js-challenge-page", "bot-detection",
        ] {
            let skill = r.get(name);
            assert!(
                skill.is_some(),
                "Skill '{}' not found in registry",
                name
            );
            assert!(
                !skill.unwrap().content.is_empty(),
                "Skill '{}' has empty content",
                name
            );
        }
    }
}
