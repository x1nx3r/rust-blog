//! Type-setting scrambler for the masthead.
//!
//! Computes per-letter glyph sequences once at process start and bakes
//! them into the HTML as data attributes. The client script only replays
//! the sequences, so there is no randomness or layout cost at runtime.
//! Without JS (or under reduced motion) the plain settled headline shows.

use std::sync::OnceLock;

/// The masthead copy that gets the scramble treatment.
const MASTHEAD_TEXT: &str = "A Wandering Mind's Journal";

/// How many decoy glyphs each letter cycles through before settling.
const MIN_STATES: u32 = 5;
const MAX_STATES: u32 = 9;

/// Stagger between consecutive letters, plus random jitter for a
/// Masthead pacing: the reveal spreads wide because nothing competes
/// with the nameplate at that point in the timeline.
const MASTHEAD_SPREAD_MS: u64 = 1100;
const MASTHEAD_JITTER_MS: u64 = 40;

/// Headline pacing: titles resolve sooner, so a headline is assembled
/// roughly when its wired block lands in the cascade.
const HEADLINE_SPREAD_MS: u64 = 700;
const HEADLINE_JITTER_MS: u64 = 25;

/// Print-shop debris banks, split by case so a capital shuffles through
/// capitals and a lowercase through lowercase. HTML-escapable characters
/// are excluded so generated attributes stay safe unescaped.
const UPPER_BANK: &[u8] = b"ACEHKMNQRSTVWXZJXYB";
const LOWER_BANK: &[u8] = b"acehkqmnorstvwxzybg";
const DIGIT_BANK: &[u8] = b"1234567890";
const PUNCT_BANK: &[u8] = b"!?#%*+=/-";

/// Minimal HTML escaper for attribute and text values we generate.
fn escape_char(c: char, out: &mut String) {
    match c {
        '&' => out.push_str("&amp;"),
        '<' => out.push_str("&lt;"),
        '>' => out.push_str("&gt;"),
        '"' => out.push_str("&quot;"),
        other => out.push(other),
    }
}

/// xorshift64* — tiny PRNG, no dependency. Seeded from the clock so the
/// whole site draws from one deterministic stream per process.
struct Ink {
    state: u64,
}

impl Ink {
    fn new() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9E3779B97F4A7C15);
        Ink {
            state: nanos | 1,
        }
    }

    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }

    fn pick(&mut self, bank: &[u8]) -> char {
        let idx = self.below(bank.len() as u64) as usize;
        bank[idx] as char
    }
}

fn glyph_bank(c: char) -> &'static [u8] {
    if c.is_ascii_uppercase() {
        UPPER_BANK
    } else if c.is_ascii_lowercase() {
        LOWER_BANK
    } else if c.is_ascii_digit() {
        DIGIT_BANK
    } else {
        PUNCT_BANK
    }
}

/// Renders the masthead as word-wrapped letter spans:
/// `<span class="np-sc" data-g="…" data-c="…" data-d="…">A</span>`
/// inside `<span class="np-w">word</span>` groups joined by plain
/// spaces, so line wrapping keeps behaving like normal text.
pub fn masthead_html() -> String {
    static SCRAMBLE: OnceLock<String> = OnceLock::new();
    SCRAMBLE
        .get_or_init(|| build_scramble(MASTHEAD_TEXT, MASTHEAD_SPREAD_MS, MASTHEAD_JITTER_MS))
        .clone()
}

/// Same treatment for story headlines: quicker reveal spread so the
/// text is fully typeset soon after its block arrives.
pub fn headline_html(text: &str) -> String {
    build_scramble(text, HEADLINE_SPREAD_MS, HEADLINE_JITTER_MS)
}

fn build_scramble(text: &str, spread_ms: u64, jitter_ms: u64) -> String {
    let letter_count = text.chars().filter(|c| !c.is_whitespace()).count().max(1) - 1;
    let step = spread_ms as f64 / letter_count.max(1) as f64;
    let mut ink = Ink::new();
    let mut out = String::with_capacity(text.len() * 12);
    let mut delay: u64 = 0;
    let mut letter_idx: u64 = 0;

    for (word_idx, word) in text.split(' ').enumerate() {
        if word_idx > 0 {
            out.push(' ');
        }
        out.push_str("<span class=\"np-w\">");

        for c in word.chars() {
            let states = (ink.below((MAX_STATES - MIN_STATES + 1) as u64) as u32) + MIN_STATES;

            out.push_str("<span class=\"np-sc\" data-c=\"");
            escape_char(c, &mut out);
            out.push_str("\" data-d=\"");
            out.push_str(&delay.to_string());
            out.push_str("\" data-g=\"");
            for _ in 0..states {
                escape_char(ink.pick(glyph_bank(c)), &mut out);
            }
            out.push_str("\">");
            escape_char(c, &mut out);
            out.push_str("</span>");

            delay = (step * letter_idx as f64) as u64 + ink.below(jitter_ms);
            letter_idx += 1;
        }

        out.push_str("</span>");
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrambles_to_legal_html() {
        let html = build_scramble(MASTHEAD_TEXT, MASTHEAD_SPREAD_MS, MASTHEAD_JITTER_MS);
        assert!(html.contains("<span class=\"np-w\">"));
        assert!(html.contains("data-c=\"'\""));
        assert!(!html.contains("data-c=\"&\""));
        assert!(html.matches("<span class=\"np-sc\"").count() == 23); // 24 chars minus 1 word gap
    }
}
