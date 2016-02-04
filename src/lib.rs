extern crate libc;
use libc::{c_double, c_long};

#[no_mangle]
pub extern fn multiply(a_double : *mut c_double, 
                    b_double : *mut c_double, 
                    c_double : *mut c_double,
                    elements : c_long) {
    
    let size : usize = elements as usize;
    let mut a : Vec<f64> = vec![0f64; size];
    let mut b : Vec<f64> = vec![0f64; size];

    for i in 0..size {
        unsafe {
            a[i] = *(a_double.offset(i as isize)) as f64;
            b[i] = *(b_double.offset(i as isize)) as f64;
        }
    }

    let c : Vec<f64> = multiply_safe(a, b);

    for i in 0..size {
        unsafe {
            *c_double.offset(i as isize) = c[i];
        }
    }
}

fn multiply_safe(a : Vec<f64>, b : Vec<f64>) -> Vec<f64> {
    if a.len() != b.len() {
        panic!("The two vectors differ in length!");
    }

    let mut result : Vec<f64> = vec![0f64; a.len()];
    
    for i in 0..a.len() {
        result[i] = a[i]*b[i];
    }

    return result;
}

#[test]
fn it_works() {
    let a : vec<f64> = vec![1f64, 2f64, 3f64];
    let b : vec<f64> = vec![3f64, 2f64, 1f64];
    let c : vec<f64> = multiply_safe(a, b);
    let expected : vec<f64> = vec![3f64, 4f64, 3f64];

    assert!(c.len() == expected.len());
    
    for i in 0..c.len() {
        assert!(c[i] == expected[i]);
    }
}
