
extern crate rusty_machine;

use rusty_machine::prelude::*;

mod ann;

use ann::*;

fn main() {
    
    let a = Matrix::new(2,3,vec![1.,2.,3.,4.,5.,6.]);
    let b = Matrix::new(2,1,vec![1.,-1.]);
    
    let mut p = Perceptron::new(0.2f64,10i64);
    println!("Init perceptron:{:?}",p);
    p.fit(&a,&b);
    
    println!("predict:{:?}",p.predict(&a.select_rows(&[1])));
    
    // println!("{:?}",a.select_cols(&[0]));
    // println!("{:?}",a[[0,0]]);
    // println!("{:?}",&b + 2f64);
    // println!("{:?}",&b * 2f64);
    // println!("{:?}",(0usize..10usize).collect::<Vec<usize>>());
}
