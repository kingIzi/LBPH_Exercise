use ferrolearn_preprocess::LabelEncoder;
use ndarray::{Array1, Array2};
use std::collections::HashMap;

use crate::logic::{
    dataset::{Dataset, read_dataset},
    knn::KNN,
    lbph::{compute_histogram, compute_lbp_numbers},
    preprocess::label_encoder,
};

mod logic;

#[cfg(test)]
mod tests;

fn main() {
    if let Ok((x_train, y_train, x_test, y_test)) = read_dataset("dataset") {
        let encoder_train = label_encoder(&y_train);
        let (x_train, y_train) = x_train.iter().enumerate().fold(
            (
                Array2::<f32>::zeros((x_train.len(), 256)),
                Array1::<usize>::zeros(y_train.len()),
            ),
            |(mut x_acc, mut y_acc), (i, path)| {
                let lbp_numbers = compute_lbp_numbers(path);
                let histogram = compute_histogram(&lbp_numbers);
                x_acc.row_mut(i).assign(&histogram);
                y_acc[i] = encoder_train[i];
                (x_acc, y_acc)
            },
        );
        let mut p = KNN::new(3);
        p.fit(x_train, y_train);

        // NOTE: this works because LabelEncoder sorts labels alphabetically
        // and both splits contain all 5 classes. If a class could be missing
        // from one split, the index mappings would diverge.
        let encoder_test = label_encoder(&y_test);
        let (x_test, y_test) = x_test.iter().enumerate().fold(
            (
                Array2::<f32>::zeros((x_test.len(), 256)),
                Array1::<usize>::zeros(y_test.len()),
            ),
            |(mut x_acc, mut y_acc), (i, path)| {
                let lbp_numbers = compute_lbp_numbers(path);
                let histogram = compute_histogram(&lbp_numbers);
                x_acc.row_mut(i).assign(&histogram);
                y_acc[i] = encoder_test[i];
                (x_acc, y_acc)
            },
        );
        let result = p.predict(&x_test);

        // Accuracy: fraction of predictions matching the true labels.
        let correct = result
            .iter()
            .zip(y_test.iter())
            .filter(|(predicted, actual)| predicted == actual)
            .count();
        let accuracy = correct as f32 / y_test.len() as f32;
        println!(
            "Accuracy: {}/{} = {:.2}%",
            correct,
            y_test.len(),
            accuracy * 100.0
        );
    }
}
