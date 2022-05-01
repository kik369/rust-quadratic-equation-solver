use std::cmp::Ordering;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Quadratic equation solver");

    println!("\nUser input");
    let a: &String = &args[1];
    let b: &String = &args[2];
    let c: &String = &args[3];

    let a: f64 = a.trim().parse().expect("Please type a number.");
    let b: f64 = b.trim().parse().expect("Please type a number.");
    let c: f64 = c.trim().parse().expect("Please type a number.");

    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("\nSolving {}x^2 {:+}x {:+} = 0", a, b, c);

    let discriminant: f64 = calculate_discriminant(a, b, c).into();
    println!("\nDiscriminant = {}", discriminant);

    let x_and_y_vertices = calculate_vertices(a, b, c);

    calculate_roots(a, b, c);

    println!("\nVertex ({}, {})", x_and_y_vertices.0, x_and_y_vertices.1);
}

fn calculate_discriminant(a: f64, b: f64, c: f64) -> f64 {
    round_to_two_decimal_places(b.powf(2.0) - 4.0 * a * c)
}

fn calculate_vertices(a: f64, b: f64, c: f64) -> (f64, f64) {
    let x_vertex: f64 = f64::from(-b / { 2.0 * a });
    let y_vertex: f64 = a as f64 * x_vertex.powi(2) + b as f64 * x_vertex + c as f64;
    (
        round_to_two_decimal_places(x_vertex),
        round_to_two_decimal_places(y_vertex),
    )
}

fn calculate_roots(a: f64, b: f64, c: f64) -> (f64, f64, f64, f64) {
    match calculate_discriminant(a, b, c).total_cmp(&0.0) {
        Ordering::Greater => two_real_roots(a, b, c),
        Ordering::Equal => one_real_root(a, b, c),
        Ordering::Less => two_complex_roots(a, b, c),
    }
}

fn two_real_roots(a: f64, b: f64, c: f64) -> (f64, f64, f64, f64) {
    println!("Discriminant > 0. Two real roots\n");
    let x_1: f64 = f64::from({ -b + calculate_discriminant(a, b, c) } / 2.0 * a);
    let x_2: f64 = f64::from({ -b - calculate_discriminant(a, b, c) } / 2.0 * a);
    println!(
        "x_1 = {}, x_2 = {}",
        round_to_two_decimal_places(x_1),
        round_to_two_decimal_places(x_2)
    );
    (x_1, x_2, 0.0, 0.0)
}

fn one_real_root(a: f64, b: f64, c: f64) -> (f64, f64, f64, f64) {
    println!("Discriminant = 0. One real root\n");
    let x_1: f64 = f64::from({ -b + calculate_discriminant(a, b, c) } / 2.0 * a);
    println!("x = {}", round_to_two_decimal_places(x_1));
    (x_1, 0.0, 0.0, 0.0)
}

fn two_complex_roots(a: f64, b: f64, c: f64) -> (f64, f64, f64, f64) {
    println!("Discriminant < 0. Two complex roots\n");
    let discriminant: f64 = f64::from(calculate_discriminant(a, b, c).abs());
    let imaginary: f64 = discriminant.sqrt() / { 2_f64 * a as f64 };
    let real: f64 = f64::from(-b / { 2.0 * a });
    println!(
        "x_1 = {} {:+}i",
        round_to_two_decimal_places(real),
        round_to_two_decimal_places(imaginary)
    );
    println!(
        "x_2 = {} {:+}i",
        round_to_two_decimal_places(real),
        round_to_two_decimal_places(imaginary) * -1.0
    );
    (0.0, 0.0, real, imaginary)
}

fn round_to_two_decimal_places(n: f64) -> f64 {
    f64::trunc(n * 100.0) / 100.0
}
