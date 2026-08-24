use ndarray::arr1;
use serde::Deserialize;
use std::fs;

use crate::logic::{
    dataset::{parse_label_name, read_dataset},
    lbph::{compute_histogram, compute_lbp_numbers},
};

// use crate::logic::{
//     compute_histogram, compute_lbp_numbers,
//     dataset::{parse_label_name, read_dataset},
// };

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

///Test LBP values

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

/// Test Histogram

#[test]
fn test_lbp_histogram_counts() {
    let lbp_values: Vec<u8> = vec![0, 1, 2, 2, 3, 255, 255, 4, 3, 2];

    let histogram = compute_histogram(&lbp_values);

    // Total number of LBP values = 10
    assert_eq!(histogram.len(), 256);

    // 0 appears once  -> 1 / 10
    assert!((histogram[0] - 0.1).abs() < 1e-6);

    // 1 appears once
    assert!((histogram[1] - 0.1).abs() < 1e-6);

    // 2 appears three times -> 3 / 10
    assert!((histogram[2] - 0.3).abs() < 1e-6);

    // 3 appears twice -> 2 / 10
    assert!((histogram[3] - 0.2).abs() < 1e-6);

    // 4 appears once
    assert!((histogram[4] - 0.1).abs() < 1e-6);

    // 255 appears twice
    assert!((histogram[255] - 0.2).abs() < 1e-6);
}

#[test]
fn test_lbp_histogram_normalization() {
    let lbp_values: Vec<u8> = vec![0, 1, 2, 2, 3, 255, 255, 4, 3, 2];

    let histogram = compute_histogram(&lbp_values);

    // A normalized histogram should sum to 1.
    let sum: f32 = histogram.sum();

    assert!((sum - 1.0).abs() < 1e-6);
}

#[test]
fn test_lbp_histogram_zero_bins() {
    let lbp_values: Vec<u8> = vec![0, 1, 2];

    let histogram = compute_histogram(&lbp_values);

    // Values that never occurred should have frequency 0.
    assert_eq!(histogram[3], 0.0);
    assert_eq!(histogram[100], 0.0);
    assert_eq!(histogram[200], 0.0);
    assert_eq!(histogram[255], 0.0);
}

#[test]
fn test_lbp_histogram_single_value() {
    let lbp_values: Vec<u8> = vec![42, 42, 42, 42];

    let histogram = compute_histogram(&lbp_values);

    // 42 occurs 100% of the time.
    assert!((histogram[42] - 1.0).abs() < 1e-6);

    // Everything else should be zero.
    assert_eq!(histogram[0], 0.0);
    assert_eq!(histogram[41], 0.0);
    assert_eq!(histogram[43], 0.0);
    assert_eq!(histogram[255], 0.0);
}

/// Test label names
#[test]
fn test_split_name_empty() {
    let input = "";
    let output = "";
    let result = parse_label_name(input);
    assert_eq!(result, output, "Got {input}\n Expected {output}")
}

#[test]
fn test_split_name_no_underscore_found() {
    let input = "Jane Doe56.jpg";
    let output = "Jane Doe56";
    let result = parse_label_name(input);
    assert_eq!(result, output, "Got {input}\n Expected {output}")
}

#[test]
fn test_split_name_one_underscore_found() {
    let input = "Jane Doe_56.jpg";
    let output = "Jane Doe";
    let result = parse_label_name(input);
    assert_eq!(result, output, "Got {input}\n Expected {output}")
}

#[test]
fn test_split_name_multiple_underscore_found() {
    let input = "Jane Doe_56_75.jpg";
    let output = "Jane Doe_56";
    let result = parse_label_name(input);
    assert_eq!(result, output, "Got {input}\n Expected {output}")
}

/// Test dataset for how to get dataset in x_train, y_train, x_test, y_test format
#[test]
fn test_read_dataset_folder_not_exist() {
    let input = "dataset_";
    let result = read_dataset(input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "folder does not exist");
}

#[test]
fn test_read_dataset_invalid_format() {
    let input = "test_data";
    let result = read_dataset(input);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Could not find /train or /test folders"
    );
}

#[test]
fn test_read_dataset_correct_format() {
    let input = "dataset";
    let result = read_dataset(input);
    assert!(result.is_ok());
    let (x_train, y_train, x_test, y_test) = result.unwrap();
    assert_eq!(x_train.len(), 399);
    assert_eq!(y_train.len(), 399);
    assert_eq!(x_test.len(), 15);
    assert_eq!(y_test.len(), 15);
}
