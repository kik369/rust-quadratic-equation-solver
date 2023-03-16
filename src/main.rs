use plotters::prelude::*;
use std::cmp::Ordering;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Quadratic equation solver\n");

    println!("User input");
    let a: &String = &args[1];
    let b: &String = &args[2];
    let c: &String = &args[3];

    let a: f64 = a.trim().parse().expect("Please type a number.");
    let b: f64 = b.trim().parse().expect("Please type a number.");
    let c: f64 = c.trim().parse().expect("Please type a number.");

    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);

    let solving_caption: String = format!("Solving {}x^2 {:+}x {:+} = 0", a, b, c);

    println!("{solving_caption}");

    let discriminant: f64 = calculate_discriminant(a, b, c).into();
    println!("\nDiscriminant = {}\n", discriminant);

    let vertex_x_and_y: (f64, f64) = calculate_vertices(a, b, c);

    let (x_1, x_2, _y_1, _y_2) = calculate_roots(a, b, c).into();

    println!("\nVertex ({}, {})", vertex_x_and_y.0, vertex_x_and_y.1);

    plot_this(x_1, x_2, vertex_x_and_y, a, b, c, solving_caption)
        .expect("This fixes the error for now");
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
    println!("Discriminant > 0\n");
    println!("Two real roots\n");
    let x_1: f64 = f64::from((-b + calculate_discriminant(a, b, c).sqrt()) / (2.0 * a));
    let x_2: f64 = f64::from((-b - calculate_discriminant(a, b, c).sqrt()) / (2.0 * a));
    println!(
        "x_1 = {}, x_2 = {}",
        round_to_two_decimal_places(x_1),
        round_to_two_decimal_places(x_2)
    );
    (x_1, x_2, 0.0, 0.0)
}

fn one_real_root(a: f64, b: f64, c: f64) -> (f64, f64, f64, f64) {
    println!("Discriminant = 0\n");
    println!("One real root\n");
    let x_1: f64 = f64::from({ -b + calculate_discriminant(a, b, c).sqrt() } / 2.0 * a);
    println!("x = {}", round_to_two_decimal_places(x_1));
    (x_1, 0.0, 0.0, 0.0)
}

fn two_complex_roots(a: f64, b: f64, c: f64) -> (f64, f64, f64, f64) {
    println!("Discriminant < 0\n");
    println!("Two complex roots\n");
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

fn plot_this(
    x_1: f64,
    x_2: f64,
    vertex_x_and_y: (f64, f64),
    a: f64,
    b: f64,
    c: f64,
    solving_caption: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a 800*600 bitmap and start drawing
    let root = BitMapBackend::new("parabola.png", (800, 700)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a chart builder and set ranges for x and y axes

    let y_vertex: f64 = a as f64 * (x_1.min(x_2) - (x_1.max(x_2) - vertex_x_and_y.0)).powi(2)
        + b as f64 * (x_1.min(x_2) - (x_1.max(x_2) - vertex_x_and_y.0))
        + c as f64;

    println!("y_vertex {}", y_vertex);
    println!("other y {}", vertex_x_and_y.1 * 1.1);

    let mut chart = ChartBuilder::on(&root)
        .caption(solving_caption, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            x_1.min(x_2) - (x_1.max(x_2) - vertex_x_and_y.0) as f64
                ..x_1.max(x_2) + (x_1.max(x_2) - vertex_x_and_y.0) as f64,
            (vertex_x_and_y.1 * 1.1 as f64).min(y_vertex * 1.1)
                ..(vertex_x_and_y.1 * 1.1 as f64).max(y_vertex * 1.1),
        )?;

    // Draw grid lines
    chart.configure_mesh().draw()?;

    // Draw the parabola using function_chart
    // ((x_1.min(x_2) * 10.1) as i32..=(x_1.max(x_2) * 10.1) as i32)

    let plot_from = x_1.min(x_2) - (x_1.max(x_2) - vertex_x_and_y.0);
    let plot_to = x_1.max(x_2) + (x_1.max(x_2) - vertex_x_and_y.0);

    let parabola_style = ShapeStyle {
        color: BLUE.mix(0.6),
        filled: false,
        stroke_width: 2,
    };

    chart.draw_series(LineSeries::new(
        ((plot_from * 10.1) as i32..=((plot_to) * 10.1) as i32)
            .map(|x| x as f64 / 10.0)
            .map(|x| (x, a * x * x + b * x + c)),
        parabola_style,
    ))?;

    // Create three points with different colors
    let point1 = Circle::new(
        (vertex_x_and_y.0, vertex_x_and_y.1),
        4,
        ShapeStyle::from(&RED).filled(),
    );
    let point2 = Circle::new((x_1, 0.0), 4, ShapeStyle::from(&RED).filled());
    let point3 = Circle::new((x_2, 0.0), 4, ShapeStyle::from(&RED).filled());

    // Draw the points on the chart
    chart.draw_series(std::iter::once(point1))?;
    chart.draw_series(std::iter::once(point2))?;
    chart.draw_series(std::iter::once(point3))?;

    // Create three texts with coordinates
    let text1 = Text::new(
        format!(
            "({}, {})",
            round_to_two_decimal_places(vertex_x_and_y.0),
            round_to_two_decimal_places(vertex_x_and_y.1),
        ),
        (vertex_x_and_y.0, vertex_x_and_y.1),
        ("sans-serif", 18).into_font(),
    );
    let text2 = Text::new(
        format!("({}, 0.0)", round_to_two_decimal_places(x_1)),
        (x_1, 0.0),
        ("sans-serif", 18).into_font(),
    );
    let text3 = Text::new(
        format!("({}, 0.0)", round_to_two_decimal_places(x_2)),
        (x_2, 0.0),
        ("sans-serif", 18).into_font(),
    );

    // Draw the texts on the chart
    chart.draw_series(std::iter::once(text1))?;
    chart.draw_series(std::iter::once(text2))?;
    chart.draw_series(std::iter::once(text3))?;

    Ok(())
}
