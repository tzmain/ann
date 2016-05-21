
use rusty_machine::prelude::*;

#[derive(Debug)]
pub struct Perceptron {
    w : Matrix<f64>,
    b : f64,
    eta : f64,
    n_iter : i64
}

impl Perceptron {
    
    pub fn new(eta : f64,n_iter : i64) -> Self {
        return Perceptron {
            w : Matrix::zeros(1,1),
            b : 0f64,
            eta : eta,
            n_iter : n_iter
        }
    }
    
    pub fn newDefaut() -> Self {
        Perceptron::new(0.01f64,10i64)
    }
    
    pub fn fit(&mut self, X : &Matrix<f64>, y : &Matrix<f64>) {
        self.w = Matrix::<f64>::zeros(X.rows() + 1usize,1);
        self.b = 0f64;
        
        for _ in 0..self.n_iter {
            for i in 0..X.rows() {
                let row = X.select_rows(&[i as usize]);
                let y_row = y.select_rows(&[i as usize]);
                
                let update = (y_row[[0,0]] - self.predict(&row)) * self.eta;
                println!("update:{:?}",update);
                self.b += update;
                println!("before:\nw:{:?},b:{:?},row:{:?},y:{:?}",self.w,self.b,&row,&y_row);
                self.w +=  &row * update;
                println!("after:\nw:{:?},b:{:?},row:{:?},y:{:?}",self.w,self.b,&row,&y_row);
            }
        }
    }
    
    pub fn net_input(&mut self, X : &Matrix<f64>) -> f64 {
        (X * self.w.select_cols(&[0usize]))[[0,0]] + self.b
    }
    
    pub fn predict(&mut self, X : &Matrix<f64>) -> f64 {
        if self.net_input(X) >= 0f64 {
            1f64
        }else{
            -1f64
        }
    }
    
    
    
}