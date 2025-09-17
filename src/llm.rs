use ndarray::Array1;
use ndarray::{Array2, Axis};


pub trait Layer {
    fn layer_type(&self) -> &str;

    fn forward(&mut self, input: &Array2<f32>) -> Array2<f32>;

    fn backward(&mut self, grads: &Array2<f32>, lr : f32) -> Array2<f32>
}

