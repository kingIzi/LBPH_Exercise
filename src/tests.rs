use serde::Deserialize;
use std::fs;

use crate::compute_lbp_numbers;

#[derive(Deserialize)]
struct LbpEntry {
    #[serde(rename = "imagePath")]
    image_path: String,
    lbp: Vec<u8>,
}

fn load_expected_entries() -> Vec<LbpEntry> {
    let json = fs::read_to_string("test_data/lbp_expected.json")
        .expect("test_data/lbp_expected.json is missing; run `cargo run` first");
    serde_json::from_str(&json).expect("failed to parse JSON")
}

fn assert_lbp_matches_stored(image_name: &str) {
    let entries = load_expected_entries();
    let entry = entries
        .iter()
        .find(|e| e.image_path == image_name)
        .unwrap_or_else(|| panic!("no JSON entry found for {image_name}"));

    let computed = compute_lbp_numbers(&format!("test_data/{image_name}"));

    assert_eq!(
        computed.len(),
        49_284,
        "{image_name}: expected a 49284-integer vector"
    );
    assert_eq!(
        computed, entry.lbp,
        "{image_name}: LBP differs from stored result"
    );
}

/// If a test fails with "missing", run `cargo run` to (re)generate the fixture.
#[test]
fn lbp_matches_1_jpg() {
    assert_lbp_matches_stored("1.jpg");
}

#[test]
fn lbp_matches_2_jpg() {
    assert_lbp_matches_stored("2.jpg");
}

#[test]
fn lbp_matches_3_jpg() {
    assert_lbp_matches_stored("3.jpg");
}

#[test]
fn lbp_matches_4_jpg() {
    assert_lbp_matches_stored("4.jpg");
}

#[test]
fn lbp_matches_5_jpeg() {
    assert_lbp_matches_stored("5.jpeg");
}
