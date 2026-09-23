// use serde::Serialize;
// use std::fs;
// use std::path::Path;

// mod lib;

// #[path = "lib.rs"]
// mod lbph_exercise;

// use lbph_exercise::compute_lbp_numbers;

// #[derive(Serialize)]
// struct LbpEntry {
//     #[serde(rename = "imagePath")]
//     image_path: String,
//     lbp: Vec<u8>,
// }

// fn main() {
//     let images = ["1.jpg", "2.jpg", "3.jpg", "4.jpg", "5.jpeg"];
//     let mut entries = Vec::new();

//     for img_name in &images {
//         let path_str = format!("test_data/{}", img_name);
//         if Path::new(&path_str).exists() {
//             println!("Processing {}...", img_name);
//             let lbp = compute_lbp_numbers(&path_str);
//             entries.push(LbpEntry {
//                 image_path: img_name.to_string(),
//                 lbp,
//             });
//         } else {
//             eprintln!("Warning: {} not found in test_data/", img_name);
//         }
//     }

//     let json_output = serde_json::to_string_pretty(&entries)
//         .expect("Failed to serialize LBP entries to JSON");

//     fs::write("test_data/lbp_expected.json", json_output)
//         .expect("Failed to write test_data/lbp_expected.json");

//     println!("Successfully regenerated test_data/lbp_expected.json!");
// }