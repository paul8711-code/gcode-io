pub fn subtract(b: [f64; 3], a: [f64; 3]) -> [f64; 3] {
    [
        b[0] - a[0],
        b[1] - a[1],
        b[2] - a[2],
    ]
}

pub fn cross_product(u: [f64; 3], v: [f64; 3]) -> [f64; 3] {
    [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] - v[0],
    ]
}
