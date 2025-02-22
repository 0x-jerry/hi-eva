pub fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let x = p1.0 - p2.0;
    let y = p1.1 - p2.1;

    return (x * x + y * y).sqrt();
}
