pub fn square(s: f64) -> f64 {
    s * s
}

pub fn rectangle(a: f64, b: f64) -> f64 {
    a * b
}

pub fn triangle(b: f64, h: f64) -> f64 {
    (b * h) / 2.0
}

/// In geometry, Heron's formula (or Hero's formula) gives the area of a triangle in terms of the three side lengths
/// s = (1(a+b+c))/2, the area is:
///
/// A = sqrt(s(s-a)(s-b)(s-c))
///
/// https://en.wikipedia.org/wiki/Heron%27s_formula
pub fn triangle_heron(a: f64, b: f64, c: f64) -> f64 {
    let s = (a + b + c) / 2.0;
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_area_of_a_square() {
        assert_eq!(square(2.0), 4.0);
    }

    #[test]
    fn calc_area_of_a_rectangle() {
        assert_eq!(rectangle(7.1, 10.5), 74.55);
    }

    #[test]
    fn calc_area_of_a_triangle() {
        assert_eq!(triangle(6.8, 15.8), 53.72);
    }

    #[test]
    fn calc_area_of_a_triangle_using_heron() {
        assert_eq!(triangle_heron(10.0, 5.0, 10.0), 24.206145913796355);
    }
}
