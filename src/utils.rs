pub fn quadratic_solver(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    // b^2 - 4ac
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let common = discriminant.sqrt();
    return Some(((-b - common) / (2.0 * a), (-b + common) / (2.0 * a)));
}
