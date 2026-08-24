use ferrolearn_core::traits::{Fit, Transform};
use ferrolearn_preprocess::LabelEncoder;
use ndarray::Array1;

pub fn label_encoder(labels: &[String]) -> Array1<usize> {
    let labels = Array1::from(labels.to_vec());
    let fitted = LabelEncoder::new().fit(&labels, &()).unwrap();
    fitted.transform(&labels).unwrap()
}
