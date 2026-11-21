//! Crash reports and issue trackers: the text of an issue a crash report would be filed as, and
//! the search for one already filed for the same problem. This build has no issue tracker of its
//! own (there is nothing on GitHub to file against), so [`find`] asks no server at all: the
//! report stays on this computer, in its .txt file, and nothing is uploaded anywhere.
//! [`new_issue_url`] is kept for the crash window's form link, built for the project's
//! placeholder address ([`REPO`]); a build with a real tracker points it there.

use serde::Deserialize;

pub const REPO: &str = "mattdownloadmanager/mattdownloadmanager";

/// GitHub turns away very long addresses: the form's address is kept under this, the report cut
/// to fit (the full one goes in as the attached file).
pub const URL_BUDGET: usize = 7000;

/// The form's fields MDM fills, besides the report: (field id, value).
pub type Fields = Vec<(&'static str, String)>;

fn enc(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

/// Where a report goes in this build: the site's support page. Nothing is uploaded and no
/// other site is opened -- the report itself is the .txt file beside the dump, which the
/// window offers to copy or attach.
pub const SUPPORT_URL: &str = "https://idm.teachermatt.com/help.html#support";

/// The address the crash window's button opens: this build's support page (the report's text
/// stays on this computer).
pub fn new_issue_url(_title: &str, _fields: &Fields, _report: &str, _cut_note: &str) -> String {
    SUPPORT_URL.to_owned()
}
/// An issue that matches.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Found {
    pub number: u64,
    pub title: String,
    pub html_url: String,
    pub state: String,
}

/// The issue already filed for `signature` (its body holds it: the form's Signature field), open
/// ones first. None when there is none or GitHub can't be asked.
pub async fn find(http: &reqwest::Client, signature: &str) -> Option<Found> {
    // This build has no issue tracker and must not send a crash report (or its signature)
    // anywhere by itself: nothing is asked, so nothing leaves the computer.
    let _ = (http, signature);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_long_report_is_cut_at_a_line_within_the_budget() {
        let fields = vec![
            ("version", "0.2.3".to_owned()),
            ("kind", "freeze".to_owned()),
        ];
        let report: String = (0..2000)
            .map(|i| format!("line {i} of the report\n"))
            .collect();
        let url = new_issue_url("Freeze in x", &fields, &report, "(cut: see the file)");
        assert!(url.len() <= URL_BUDGET);
        assert!(url.contains("template=crash.yml"));
        assert!(url.contains("&version=0.2.3"));
        let report_part = url.split("&report=").nth(1).unwrap();
        let decoded: String = url::form_urlencoded::parse(format!("r={report_part}").as_bytes())
            .next()
            .unwrap()
            .1
            .into_owned();
        assert!(decoded.starts_with("line 0 of the report\n"));
        assert!(decoded.trim_end().ends_with("(cut: see the file)"));
        // Whole lines only.
        assert!(
            decoded
                .lines()
                .all(|l| l.is_empty() || l.starts_with("line ") || l.starts_with('('))
        );
    }

    #[test]
    fn a_short_report_goes_whole() {
        let url = new_issue_url("t", &vec![], "short report", "cut");
        assert!(url.ends_with("&report=short+report"));
    }
}
