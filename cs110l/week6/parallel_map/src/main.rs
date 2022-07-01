extern crate threadpool;

use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<Option<U>>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let len = input_vec.len();
    
    let (prod_input, cons_input) = crossbeam_channel::unbounded();
    let (prod_output, cons_output) = crossbeam_channel::unbounded();
    let mut thread_pool = Vec::new();
    for _ in 0..num_threads {
        let cons_input = cons_input.clone();
        let prod_output = prod_output.clone();
        thread_pool.push(thread::spawn( move || {
            while let Ok((index, input)) = cons_input.recv() {
                prod_output.send((index, f(input))).unwrap();
            }
        }));
    }
    drop(cons_input);
    drop(prod_output);
    // produce input
    let thread_in = thread::spawn( move || {
        for i in (0..len).rev() {
            prod_input.send((i, input_vec.pop().unwrap())).unwrap();
        }
    });
    // consume output
    let thread_out = thread::spawn( move || -> Vec<Option<U>> {
        let mut output_vec: Vec<Option<U>> = Vec::with_capacity(len);
        output_vec.resize_with(len, || -> Option<U> { None });
        while let Ok((index, output)) = cons_output.recv() {
            output_vec[index] = Some(output);
        }
        output_vec
    });
    // wait for computing
    thread_in.join().unwrap();
    for i in thread_pool {
        i.join().unwrap();
    }
    thread_out.join().unwrap()
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 8, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
