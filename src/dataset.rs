use ndarray::Array1;

use std::io::Error;

pub type Dataset = (Vec<String>, Vec<String>, Vec<String>, Vec<String>);

pub fn read_dataset(path: &str) -> Result<Dataset, Error> {
    todo!("Read the dataset and return file paths for the train and test images");
}

fn parse_label_name(file_name: &str) -> String {
    todo!("Not yet implemented");
}

fn ensure_folders_exist(path: &str) -> Result<(String, String), Error> {
    todo!("Ensure that a path exists and that it is a folder");
}

fn parse_images(path: &str) -> Result<(Vec<String>, Vec<String>), Error> {
    todo!("Parse images into a vector of file paths and labels.");
}

fn is_image_file(file_name: &str) -> bool {
    todo!("Ensure that a file_name is an image");
}

/// Peform label encoding
pub fn encode_labels(labels: &Vec<String>) -> Array1<usize> {
    todo!("Implement a label encoder");
}

/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    ////////////////////////////////////////////////////////////////////////////
    // ensure_folders_exist
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_ensure_folders_exist_valid_dataset() {
        let result = ensure_folders_exist("dataset");
        assert!(result.is_ok(), "the dataset folder should be accepted");

        let (train_path, test_path) = result.unwrap();
        assert!(
            train_path.ends_with("train"),
            "first tuple element should be the train folder, got: {train_path}"
        );
        assert!(
            test_path.ends_with("test"),
            "second tuple element should be the test folder, got: {test_path}"
        );
        assert!(std::path::Path::new(&train_path).is_dir());
        assert!(std::path::Path::new(&test_path).is_dir());
    }

    #[test]
    fn test_ensure_folders_exist_missing_folder() {
        assert!(ensure_folders_exist("dataset_").is_err());
        assert!(ensure_folders_exist("no_such_folder").is_err());
    }

    #[test]
    fn test_ensure_folders_exist_path_is_a_file() {
        // A path can exist without being a folder.
        assert!(ensure_folders_exist("Cargo.toml").is_err());
    }

    #[test]
    fn test_ensure_folders_exist_without_train_and_test_subfolders() {
        // test_data exists but contains no train/ and test/ subfolders.
        assert!(ensure_folders_exist("test_data").is_err());
    }

    ////////////////////////////////////////////////////////////////////////////
    // is_image_file
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_is_image_file_accepts_image_extensions() {
        assert!(is_image_file("photo.jpg"));
        assert!(is_image_file("photo.jpeg"));
        assert!(is_image_file("photo.png"));
        assert!(is_image_file("Alfre Woodard images_5.png"));
    }

    #[test]
    fn test_is_image_file_is_case_insensitive() {
        assert!(is_image_file("PHOTO.JPG"));
        assert!(is_image_file("Photo.PNG"));
        assert!(is_image_file("photo.JPEG"));
    }

    #[test]
    fn test_is_image_file_rejects_non_images() {
        assert!(!is_image_file("notes.txt"));
        assert!(!is_image_file("animation.gif"));
        assert!(!is_image_file("archive.zip"));
        assert!(!is_image_file("no_extension"));
    }

    #[test]
    fn test_is_image_file_rejects_windows_zone_identifier_files() {
        // Images copied from Windows carry 'Zone.Identifier' sidecar files.
        // They are not images even though the name contains '.jpg'.
        assert!(!is_image_file("Alfre Woodard images_1.jpg:Zone.Identifier"));
    }

    ////////////////////////////////////////////////////////////////////////////
    // parse_label_name
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_parse_label_name_from_full_path() {
        let input = "/home/user/projects/rust/pure/lbph/dataset/train/Alfre Woodard images_5.png";
        assert_eq!(parse_label_name(input), "Alfre Woodard images");
    }

    #[test]
    fn test_parse_label_name_from_file_name() {
        assert_eq!(
            parse_label_name("Alfre Woodard images_5.png"),
            "Alfre Woodard images"
        );
        assert_eq!(
            parse_label_name("Anne Hathaway images_4.jpeg"),
            "Anne Hathaway images"
        );
    }

    #[test]
    fn test_parse_label_name_multiple_underscores() {
        // Only the last underscore separates the index.
        assert_eq!(parse_label_name("Jane Doe_56_75.jpg"), "Jane Doe_56");
    }

    #[test]
    fn test_parse_label_name_without_underscore() {
        assert_eq!(parse_label_name("Jane Doe56.jpg"), "Jane Doe56");
    }

    #[test]
    fn test_parse_label_name_empty_input() {
        assert_eq!(parse_label_name(""), "");
    }

    ////////////////////////////////////////////////////////////////////////////
    // parse_images
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_parse_images_train_folder() {
        let (paths, labels) = parse_images("dataset/train").expect("dataset/train should parse");

        // 399 real images; the 'Zone.Identifier' sidecar files must be filtered out.
        assert_eq!(paths.len(), 399);
        assert_eq!(labels.len(), 399);

        for path in &paths {
            assert!(is_image_file(path), "{path} is not an image");
            assert!(!path.contains("Zone.Identifier"));
            assert!(
                std::path::Path::new(path).is_absolute(),
                "{path} should be an absolute path"
            );
        }

        let expected_labels = [
            "Alfre Woodard images",
            "Amanda Seyfried images",
            "Amy Adams images",
            "Angelina Jolie images",
            "Anne Hathaway images",
        ];
        for label in &labels {
            assert!(
                expected_labels.contains(&label.as_str()),
                "unexpected label: {label}"
            );
        }
    }

    #[test]
    fn test_parse_images_test_folder() {
        let (paths, labels) = parse_images("dataset/test").expect("dataset/test should parse");

        assert_eq!(paths.len(), 15);
        assert_eq!(labels.len(), 15);

        // All 5 celebrities are present in the test split.
        let unique: std::collections::HashSet<&String> = labels.iter().collect();
        assert_eq!(unique.len(), 5);
    }

    #[test]
    fn test_parse_images_labels_match_their_paths() {
        let (paths, labels) = parse_images("dataset/test").expect("dataset/test should parse");

        for (path, label) in paths.iter().zip(labels.iter()) {
            assert_eq!(parse_label_name(path), *label);
        }
    }

    #[test]
    fn test_parse_images_missing_folder() {
        assert!(parse_images("dataset_/train").is_err());
        assert!(parse_images("no_such_folder").is_err());
    }

    #[test]
    fn test_parse_images_filters_non_images() {
        let dir = std::env::temp_dir().join("lbph_parse_images_filter_test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a_1.jpg"), b"x").unwrap();
        std::fs::write(dir.join("b_2.png"), b"x").unwrap();
        std::fs::write(dir.join("c.txt"), b"x").unwrap();
        std::fs::write(dir.join("d_3.gif"), b"x").unwrap();

        let (paths, labels) =
            parse_images(dir.to_str().unwrap()).expect("temp folder should parse");

        // Only the .jpg and the .png survive the filter.
        assert_eq!(paths.len(), 2, "only image files should be kept: {paths:?}");
        assert_eq!(labels.len(), 2);
        for label in &labels {
            assert!(label == "a" || label == "b", "unexpected label: {label}");
        }

        std::fs::remove_dir_all(&dir).unwrap();
    }

    ////////////////////////////////////////////////////////////////////////////
    // read_dataset
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_read_dataset_returns_train_and_test_splits() {
        let (x_train, y_train, x_test, y_test) =
            read_dataset("dataset").expect("dataset should parse");

        assert_eq!(x_train.len(), 399);
        assert_eq!(y_train.len(), 399);
        assert_eq!(x_test.len(), 15);
        assert_eq!(y_test.len(), 15);

        for path in x_train.iter().chain(x_test.iter()) {
            assert!(is_image_file(path), "{path} is not an image");
            assert!(!path.contains("Zone.Identifier"));
        }
        for path in &x_train {
            assert!(path.contains("train"), "train path in wrong split: {path}");
        }
        for path in &x_test {
            assert!(path.contains("test"), "test path in wrong split: {path}");
        }
    }

    #[test]
    fn test_read_dataset_missing_folder() {
        assert!(read_dataset("dataset_").is_err());
    }

    #[test]
    fn test_read_dataset_path_is_a_file() {
        assert!(read_dataset("Cargo.toml").is_err());
    }

    #[test]
    fn test_read_dataset_without_train_and_test_subfolders() {
        assert!(read_dataset("test_data").is_err());
    }

    ////////////////////////////////////////////////////////////////////////////
    // encode_labels (sklearn LabelEncoder semantics: classes are sorted
    // lexicographically, the first class maps to 0, every occurrence of a
    // label maps to its class index)
    ////////////////////////////////////////////////////////////////////////////

    fn strs(values: &[&str]) -> Vec<String> {
        values.iter().map(|v| v.to_string()).collect()
    }

    #[test]
    fn test_encode_labels_basic_mapping() {
        // sorted(["cat", "dog", "bird"]) == ["bird", "cat", "dog"]
        // bird -> 0, cat -> 1, dog -> 2
        let labels = strs(&["cat", "dog", "bird"]);
        assert_eq!(encode_labels(&labels), Array1::from(vec![1usize, 2, 0]));
    }

    #[test]
    fn test_encode_labels_matches_sklearn_reference_example() {
        // sklearn: LabelEncoder().fit_transform(['b', 'a', 'c', 'a']) == [1, 0, 2, 0]
        let labels = strs(&["b", "a", "c", "a"]);
        assert_eq!(encode_labels(&labels), Array1::from(vec![1usize, 0, 2, 0]));
    }

    #[test]
    fn test_encode_labels_repeated_labels_share_a_code() {
        let labels = strs(&["cat", "cat", "cat"]);
        assert_eq!(encode_labels(&labels), Array1::from(vec![0usize, 0, 0]));
    }

    #[test]
    fn test_encode_labels_uses_contiguous_codes() {
        // k unique labels -> the used codes are exactly 0..k-1.
        let labels = strs(&["dog", "cat", "dog", "bird", "dog"]);
        let encoded = encode_labels(&labels);

        assert_eq!(encoded.len(), 5);
        let unique: std::collections::BTreeSet<usize> = encoded.iter().copied().collect();
        let expected: std::collections::BTreeSet<usize> = (0..3).collect();
        assert_eq!(unique, expected);
    }

    #[test]
    fn test_encode_labels_sorts_alphabetically_not_by_first_appearance() {
        // Encoding by first appearance would give dog -> 0.
        // sklearn sorts the classes first: cat -> 0, dog -> 1.
        let labels = strs(&["dog", "cat", "dog"]);
        assert_eq!(encode_labels(&labels), Array1::from(vec![1usize, 0, 1]));
    }

    #[test]
    fn test_encode_labels_case_sensitive_lexicographic_order() {
        // Byte-order sort: 'A' (65) sorts before 'a' (97), so "Apple" < "banana".
        let labels = strs(&["banana", "Apple"]);
        assert_eq!(encode_labels(&labels), Array1::from(vec![1usize, 0]));
    }

    #[test]
    fn test_encode_labels_numeric_strings_sort_lexicographically() {
        // sklearn quirk: "10" < "2" as strings, even though 10 > 2 as numbers.
        let labels = strs(&["2", "10"]);
        assert_eq!(encode_labels(&labels), Array1::from(vec![1usize, 0]));
    }

    #[test]
    fn test_encode_labels_empty_input() {
        let encoded = encode_labels(&Vec::<String>::new());
        assert_eq!(encoded.len(), 0);
    }

    #[test]
    fn test_encode_labels_single_label() {
        let labels = strs(&["only"]);
        assert_eq!(encode_labels(&labels), Array1::from(vec![0usize]));
    }

    #[test]
    fn test_encode_labels_celebrity_names() {
        // The real classes of this project, sorted:
        // Alfre(0) < Amanda(1) < Amy(2) < Angelina(3) < Anne(4)
        let labels = strs(&[
            "Anne Hathaway images",
            "Alfre Woodard images",
            "Amy Adams images",
            "Anne Hathaway images",
            "Angelina Jolie images",
            "Amanda Seyfried images",
        ]);
        assert_eq!(
            encode_labels(&labels),
            Array1::from(vec![4usize, 0, 2, 4, 3, 1])
        );
    }
}
