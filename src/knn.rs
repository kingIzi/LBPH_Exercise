use ndarray::{Array1, Array2, Axis};
use std::collections::HashMap;

pub struct KNN {
    k: usize,
    x_train: Option<Array2<f32>>,
    y_train: Option<Array1<usize>>,
}

impl KNN {
    fn euclidean_distance(x1: &Array1<f32>, x2: &Array1<f32>) -> f32 {
        todo!("Calculate the distance between two vectors");
    }
    pub fn new(k: usize) -> Self {
        todo!("Assign k={} and initialize x_train/y_train to None", k);
    }
    pub fn fit(&mut self, x: Array2<f32>, y: Array1<usize>) {
        todo!("Assign x_train={x:?} y_train={y:?}");
    }
    pub fn predict(&self, x: &Array2<f32>) -> Array1<usize> {
        todo!("Predict the class for each sample in x");
    }
    fn calculate_distance_to_all(&self, x: &Array1<f32>) -> Array1<f32> {
        todo!("Calculate the distance between x and all training samples");
    }
    fn sort_distances(&self, distances: &Array1<f32>) -> Array1<f32> {
        todo!("Sort the distances and return the indices of the k nearest neighbors");
    }
    fn take_closest(&self, distances: &Array1<f32>) -> Vec<usize> {
        todo!("Take the k closest neighbors");
    }
    fn majority_vote(&self, closest: &[usize]) -> usize {
        todo!("Take the majority vote among the closest neighbors and return its index");
    }
    fn _predict(&self, x: &Array1<f32>) -> usize {
        todo!("Predict the class for a single sample");
    }
}

/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////𝗧𝗘𝗦𝗧/////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    ////////////////////////////////////////////////////////////////////////////
    // euclidean_distance
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_euclidean_distance_known_values() {
        // A classic 3-4-5 triangle: the distance is exactly 5.0.
        let a = Array1::from(vec![0.0, 0.0]);
        let b = Array1::from(vec![3.0, 4.0]);
        assert_eq!(KNN::euclidean_distance(&a, &b), 5.0);

        // sqrt((2-1)^2 * 3) = sqrt(3)
        let a = Array1::from(vec![1.0, 1.0, 1.0]);
        let b = Array1::from(vec![2.0, 2.0, 2.0]);
        assert!(approx(KNN::euclidean_distance(&a, &b), 1.7320508));

        // The distance from a vector to itself is zero.
        let a = Array1::from(vec![1.5, -2.0, 0.25]);
        assert_eq!(KNN::euclidean_distance(&a, &a), 0.0);
    }

    ////////////////////////////////////////////////////////////////////////////
    // new
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_new_assigns_k_and_defaults_to_none() {
        let model = KNN::new(5);

        assert_eq!(model.k, 5);
        assert!(model.x_train.is_none());
        assert!(model.y_train.is_none());
    }

    ////////////////////////////////////////////////////////////////////////////
    // fit
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_fit_stores_x_train_and_y_train() {
        let mut model = KNN::new(3);

        let x = Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let y = Array1::from(vec![0usize, 1]);

        model.fit(x.clone(), y.clone());

        assert_eq!(model.x_train, Some(x));
        assert_eq!(model.y_train, Some(y));
    }

    ////////////////////////////////////////////////////////////////////////////
    // calculate_distance_to_all
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_calculate_distance_to_all_distances() {
        // Training samples: [0,0], [3,4], [1,1]. Query: [0,0].
        // Distances: 0.0, 5.0 (3-4-5 triangle), sqrt(2).
        let mut model = KNN::new(3);
        let x = Array2::from_shape_vec((3, 2), vec![0.0, 0.0, 3.0, 4.0, 1.0, 1.0]).unwrap();
        let y = Array1::from(vec![0usize, 1, 0]);
        model.fit(x, y);

        let query = Array1::from(vec![0.0, 0.0]);
        let distances = model.calculate_distance_to_all(&query);

        assert_eq!(distances.len(), 3);
        assert!(approx(distances[0], 0.0));
        assert!(approx(distances[1], 5.0));
        assert!(approx(distances[2], 1.4142135));
    }

    ////////////////////////////////////////////////////////////////////////////
    // sort_distances
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_sort_distances_sorts_ascending() {
        // The signature returns Array1<f32>: the distances sorted ascending.
        let model = KNN::new(4);

        let distances = Array1::from(vec![5.0, 1.0, 3.0, 2.0]);
        let sorted = model.sort_distances(&distances);

        assert_eq!(sorted, Array1::from(vec![1.0, 2.0, 3.0, 5.0]));
    }

    ////////////////////////////////////////////////////////////////////////////
    // take_closest
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_take_closest_returns_k_smallest_indices() {
        // k is taken from the model. The two smallest distances are
        // 1.0 (index 1) and 2.0 (index 3), in ascending distance order.
        let model = KNN::new(2);

        let distances = Array1::from(vec![5.0, 1.0, 3.0, 2.0]);
        let closest = model.take_closest(&distances);

        assert_eq!(closest, vec![1, 3]);
    }

    ////////////////////////////////////////////////////////////////////////////
    // majority_vote
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_majority_vote_returns_most_common_label() {
        //  holds indices into y_train.
        let mut model = KNN::new(3);
        let x = Array2::from_shape_vec((5, 1), vec![0.0, 1.0, 2.0, 3.0, 4.0]).unwrap();
        let y = Array1::from(vec![10usize, 20, 10, 20, 10]);
        model.fit(x, y);

        // indices 0, 2, 4 -> labels [10, 10, 10]
        assert_eq!(model.majority_vote(&[0, 2, 4]), 10);
        // indices 1, 3 -> labels [20, 20]
        assert_eq!(model.majority_vote(&[1, 3]), 20);
        // a single neighbor votes for itself
        assert_eq!(model.majority_vote(&[4]), 10);
    }

    ////////////////////////////////////////////////////////////////////////////
    // _predict
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test__predict_classifies_one_sample() {
        // Two clusters on the number line: class 0 near 0, class 1 near 10.
        let mut model = KNN::new(3);
        let x = Array2::from_shape_vec((4, 1), vec![0.0, 0.1, 10.0, 10.1]).unwrap();
        let y = Array1::from(vec![0usize, 0, 1, 1]);
        model.fit(x, y);

        // The 3 nearest neighbors of 0.05 are [0.0, 0.1, 10.0] -> labels [0, 0, 1].
        let query = Array1::from(vec![0.05]);
        assert_eq!(model._predict(&query), 0);

        // The 3 nearest neighbors of 10.0 are [10.1, 10.0, 0.1] -> labels [1, 1, 0].
        let query = Array1::from(vec![10.0]);
        assert_eq!(model._predict(&query), 1);
    }

    ////////////////////////////////////////////////////////////////////////////
    // predict
    ////////////////////////////////////////////////////////////////////////////

    #[test]
    fn test_predict_classifies_each_row() {
        // Same setup as _predict: predict must run _predict for every row.
        let mut model = KNN::new(3);
        let x = Array2::from_shape_vec((4, 1), vec![0.0, 0.1, 10.0, 10.1]).unwrap();
        let y = Array1::from(vec![0usize, 0, 1, 1]);
        model.fit(x, y);

        let x_test = Array2::from_shape_vec((2, 1), vec![0.05, 10.0]).unwrap();
        let predictions = model.predict(&x_test);

        assert_eq!(predictions, Array1::from(vec![0usize, 1]));
    }
}
