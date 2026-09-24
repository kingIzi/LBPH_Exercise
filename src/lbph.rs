use kornia::image::{Image, ImageSize, InterpolationMode};
use kornia::io::functional as F;
use kornia::tensor::CpuAllocator;

use kornia::imgproc::resize::resize_native;
use ndarray::Array1;

/// Computes the Local Binary Pattern codes for a 224x224 grayscale image.
///
/// For every pixel except the 1-pixel border, compare the center pixel
/// against its 8 neighbors (clockwise from top-left) and pack the
/// neighbor >= center bits into a single u8 (0..255).
pub fn lbph_algorithm(gray_resized: &Image<u8, 1, CpuAllocator>) -> Vec<u8> {
    let width = gray_resized.width();
    let height = gray_resized.height();
    let data = gray_resized.as_slice();

    let get_pixel = |x: usize, y: usize| -> u8 { data[y * width + x] };

    let mut lbp_results = Vec::with_capacity((width - 2) * (height - 2));

    // Iterate over every pixel except the 1-pixel border
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let center = get_pixel(x, y);

            // 8 neighbors clockwise from top-left
            let neighbors = [
                get_pixel(x - 1, y - 1), // 0: Top-Left  (bit weight 1)
                get_pixel(x, y - 1),     // 1: Top       (bit weight 2)
                get_pixel(x + 1, y - 1), // 2: Top-Right (bit weight 4)
                get_pixel(x + 1, y),     // 3: Right     (bit weight 8)
                get_pixel(x + 1, y + 1), // 4: Bottom-Right (bit weight 16)
                get_pixel(x, y + 1),     // 5: Bottom    (bit weight 32)
                get_pixel(x - 1, y + 1), // 6: Bottom-Left  (bit weight 64)
                get_pixel(x - 1, y),     // 7: Left      (bit weight 128)
            ];

            let mut code = 0u8;
            for (i, &neighbor) in neighbors.iter().enumerate() {
                if neighbor >= center {
                    code |= 1 << i;
                }
            }

            lbp_results.push(code);
        }
    }

    lbp_results
}

/// Loads an image from disk and returns its 49284 LBP codes (222 * 222).
pub fn compute_lbp_numbers(img_path: &str) -> Vec<u8> {
    // 1. Load RGB image from disk
    let img_rgb = F::read_image_any_rgb8(img_path)
        .unwrap_or_else(|e| panic!("failed to load image from {img_path}: {e}"));

    let width = img_rgb.width();
    let height = img_rgb.height();
    let rgb_data = img_rgb.as_slice();

    // 2. Exact u8 Grayscale conversion using standard ITU-R BT.601 formula
    let mut gray_u8_data = Vec::with_capacity(width * height);
    for chunk in rgb_data.chunks_exact(3) {
        let r = chunk[0] as u32;
        let g = chunk[1] as u32;
        let b = chunk[2] as u32;
        // Integer luma formula avoiding f32 rounding mismatches
        let luma = ((r * 299 + g * 587 + b * 114) / 1000) as u8;
        gray_u8_data.push(luma);
    }

    let gray_size = ImageSize { width, height };
    let gray_img = Image::<f32, 1, CpuAllocator>::new(
        gray_size,
        gray_u8_data.into_iter().map(|value| value as f32).collect(),
        CpuAllocator,
    )
    .expect("failed to create grayscale image");

    // 3. Resize grayscale image to 224x224 using kornia
    let target_size = ImageSize {
        width: 224,
        height: 224,
    };
    let mut gray_resized_f32 = Image::<f32, 1, CpuAllocator>::new(
        target_size,
        vec![0.0f32; target_size.width * target_size.height],
        CpuAllocator,
    )
    .expect("failed to allocate resized image");

    resize_native(
        &gray_img,
        &mut gray_resized_f32,
        InterpolationMode::Bilinear,
    )
    .expect("failed to resize image");

    let gray_resized = Image::<u8, 1, CpuAllocator>::new(
        target_size,
        gray_resized_f32
            .as_slice()
            .iter()
            .map(|&value| value.round().clamp(0.0, 255.0) as u8)
            .collect(),
        CpuAllocator,
    )
    .expect("failed to create resized grayscale image");

    // 4. Compute LBP on resized u8 image
    lbph_algorithm(&gray_resized)
}

/// Compute the bin count og the lbp_numbers
/// Return an ndarray
pub fn compute_histogram(lbp_numbers: &Vec<u8>) -> Array1<f32> {
    todo!("Not yet implemented");
}

/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////

use serde::Deserialize;
use std::fs;

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

/// Test Histogram
///
/// compute_histogram takes LBP numbers (each in 0..255) and returns a
/// 256-bin histogram where bin i holds how many times value i appeared.

#[test]
fn test_histogram_counts_occurrences() {
    let lbp_values: Vec<u8> = vec![0, 1, 2, 2, 3, 255, 255, 4, 3, 2];

    let histogram = compute_histogram(&lbp_values);

    // One bin for every possible LBP value
    assert_eq!(histogram.len(), 256);

    assert_eq!(histogram[0], 1.0);
    assert_eq!(histogram[1], 1.0);
    assert_eq!(histogram[2], 3.0);
    assert_eq!(histogram[3], 2.0);
    assert_eq!(histogram[4], 1.0);
    assert_eq!(histogram[255], 2.0);

    // A value that never occurred
    assert_eq!(histogram[200], 0.0);
}

#[test]
fn test_histogram_empty_input() {
    let histogram = compute_histogram(&Vec::<u8>::new());

    assert_eq!(histogram.len(), 256);
    assert!(histogram.iter().all(|&count| count == 0.0));
}

#[test]
fn test_histogram_single_repeated_value() {
    let lbp_values: Vec<u8> = vec![42, 42, 42, 42];

    let histogram = compute_histogram(&lbp_values);

    assert_eq!(histogram[42], 4.0);
    // Every other bin stays empty
    assert!(
        histogram
            .iter()
            .enumerate()
            .all(|(i, &count)| i == 42 || count == 0.0)
    );
}

#[test]
fn test_histogram_sum_equals_total() {
    let lbp_values: Vec<u8> = vec![7, 7, 200, 42, 200, 255, 0, 42];

    let histogram = compute_histogram(&lbp_values);

    // Summing every bin must give back the number of input values.
    let total: f32 = histogram.sum();
    assert!((total - lbp_values.len() as f32).abs() < 1e-4);
}
