use ndarray::{Array1, Array2, Axis};
use std::collections::HashMap;

pub struct KNN {
    k: usize,
    x_train: Option<Array2<f32>>,
    y_train: Option<Array1<usize>>,
}

impl KNN {
    fn euclidean_distance(x1: &Array1<f32>, x2: &Array1<f32>) -> f32 {
        todo!("Not yet implemented");
    }
    pub fn new(k: usize) -> Self {
        todo!("Not yet implemented");
    }
    pub fn fit(&mut self, x: Array2<f32>, y: Array1<usize>) {
        todo!("Not yet implemented");
    }
    pub fn predict(&self, x: &Array2<f32>) -> Array1<usize> {
        todo!("Not yet implemented");
    }
    fn _predict(&self, x: &Array1<f32>) -> usize {
        todo!("Not yet implemented");
    }
}
