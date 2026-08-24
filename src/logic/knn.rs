use ndarray::{Array1, Array2, Axis};
use std::collections::HashMap;

pub struct KNN {
    k: usize,
    x_train: Option<Array2<f32>>,
    y_train: Option<Array1<usize>>,
}

fn euclidean_distance(x1: &Array1<f32>, x2: &Array1<f32>) -> f32 {
    (x1 - x2).mapv(|x| x * x).sum().sqrt()
}

impl KNN {
    pub fn new(k: usize) -> Self {
        assert!(k > 0);

        Self {
            k,
            x_train: None,
            y_train: None,
        }
    }
    pub fn fit(&mut self, x: Array2<f32>, y: Array1<usize>) {
        assert_eq!(
            x.nrows(),
            y.len(),
            "Number of training samples must equal number of labels"
        );

        self.x_train = Some(x);
        self.y_train = Some(y);
    }
    pub fn predict(&self, x: &Array2<f32>) -> Array1<usize> {
        let predictions: Vec<usize> = x
            .axis_iter(Axis(0))
            .map(|sample| self._predict(&sample.to_owned()))
            .collect();

        Array1::from(predictions)
    }
    fn _predict(&self, x: &Array1<f32>) -> usize {
        let x_train = self.x_train.as_ref().unwrap();
        let y_train = self.y_train.as_ref().unwrap();

        // Calculate distance from the new sample
        // to every training sample.
        let mut distances = x_train
            .axis_iter(Axis(0))
            .enumerate()
            .map(|(index, train_sample)| {
                let distance = euclidean_distance(x, &train_sample.to_owned());
                (distance, index)
            })
            .collect::<Vec<_>>();

        // Smallest distances first.
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Take the k closest samples.
        let nearest = distances.iter().take(self.k);

        // Majority vote.
        let mut votes: HashMap<usize, usize> = HashMap::new();

        for (_, index) in nearest {
            let label = y_train[*index];

            *votes.entry(label).or_insert(0) += 1;
        }

        // Return label with most votes.
        votes.into_iter().max_by_key(|(_, count)| *count).unwrap().0
    }
}
