use ndarray::Array2;
use ndarray::Axis;
use crate::{adam::Adam, llm::Layer};
use rand_distr::{Normal, Distribution};


pub struct FeedForward {
    w1 : Array2<f32>,
    b1 : Array2<f32>,
    w2 : Array2<f32>,
    b2 : Array2<f32>,


    //Cached values for the backward pass
    input : Option<Array2<f32>>,
    hidden_pre_activation: Option<Array2<f32>>,
    hidden_post_activation: Option<Array2<f32>>,


    optimizer_w1 : Adam,
    optimizer_b1 : Adam,
    optimizer_w2 : Adam,
    optimizer_b2 : Adam,
}


impl FeedForward {
    // Initialize a feedforward layer with random weights
    pub fn new(embedding_dim:usize, hidden_dim: usize) -> Self {
        let mut rng  = rand::rng();

        //Init for w1 : std = sqrt(2 / fan_in)
        let std_w1 = (2.0 / embedding_dim as f32).sqrt();
        let normal_w1 = Normal::new(0.0, std_w1).unwrap();

        
    }
}
