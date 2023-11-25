use candle_core::{Device, Result, Tensor};
use candle_nn::Optimizer;

struct Trainer {
    compiled: bool,
    loss: f32,
}

impl Trainer {
    fn compile(&self, optimizer:str, loss:Tensor){
        
    }

    fn compute_loss(&self){

    }

    fn fit(&self, x: Tensor, y: Tensor, batch_size:i32, epochs:i32){

    }

    fn evaluate(&self, x: Tensor, y: Tensor, batch_size:i32){

    }

    fn predict(&self, x: Tensor, batch_size:i32, callbacks:str){
        
    }
}