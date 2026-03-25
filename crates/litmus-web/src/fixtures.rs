use std::sync::OnceLock;

use litmus_model::term_output::TermOutput;

/// Embedded fixture output JSON data.
static FIXTURE_DATA: &[(&str, &str)] = &[
    ("git-diff", include_str!("../../../fixtures/git-diff/output.json")),
    ("git-log", include_str!("../../../fixtures/git-log/output.json")),
    ("ls-color", include_str!("../../../fixtures/ls-color/output.json")),
    ("cargo-build", include_str!("../../../fixtures/cargo-build/output.json")),
    ("shell-prompt", include_str!("../../../fixtures/shell-prompt/output.json")),
    ("python-repl", include_str!("../../../fixtures/python-repl/output.json")),
    ("htop", include_str!("../../../fixtures/htop/output.json")),
    ("color-showcase", include_str!("../../../fixtures/color-showcase/output.json")),
];

static PARSED_FIXTURES: OnceLock<Vec<TermOutput>> = OnceLock::new();

fn parsed() -> &'static Vec<TermOutput> {
    PARSED_FIXTURES.get_or_init(|| {
        FIXTURE_DATA
            .iter()
            .filter_map(|(id, json)| {
                serde_json::from_str::<TermOutput>(json)
                    .map_err(|e| web_sys_log(&format!("Warning: failed to parse fixture {id}: {e}")))
                    .ok()
            })
            .collect()
    })
}

/// Get all embedded fixtures.
#[cfg(test)]
fn all_fixtures() -> &'static Vec<TermOutput> {
    parsed()
}

/// Find a fixture by ID.
pub fn fixture_by_id(id: &str) -> Option<&'static TermOutput> {
    parsed().iter().find(|f| f.id == id)
}

/// Get the first fixture (for preview cards).
pub fn default_fixture() -> Option<&'static TermOutput> {
    parsed().first()
}

/// Log to browser console (no-op if web_sys isn't available).
fn web_sys_log(_msg: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::warn_1(&_msg.into());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_fixtures_parse_successfully() {
        let fixtures = all_fixtures();
        assert_eq!(
            fixtures.len(),
            FIXTURE_DATA.len(),
            "all embedded fixtures should parse"
        );
    }

    #[test]
    fn fixtures_have_nonempty_lines() {
        for f in all_fixtures() {
            assert!(!f.id.is_empty(), "fixture id should not be empty");
            assert!(!f.lines.is_empty(), "fixture {} has no lines", f.id);
        }
    }

    #[test]
    fn fixture_by_id_finds_git_diff() {
        assert!(fixture_by_id("git-diff").is_some());
        assert!(fixture_by_id("nonexistent").is_none());
    }

    #[test]
    fn default_fixture_returns_first() {
        let first = default_fixture().unwrap();
        assert_eq!(first.id, all_fixtures()[0].id);
    }
}
