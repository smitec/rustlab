
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
    let a : Vec<f64> = vec![1f64, 2f64, 3f64];
    let b : Vec<f64> = vec![3f64, 2f64, 1f64];
    let c : Vec<f64> = multiply_safe(a, b);
    let expected : Vec<f64> = vec![3f64, 4f64, 3f64];

    assert!(c.len() == expected.len());
    
    for i in 0..c.len() {
        assert!(c[i] == expected[i]);
    }
}
