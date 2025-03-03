use plotters::prelude::*;
use std::cmp::Ordering;
use std::env;
use std::error::Error;
use std::fmt;
use std::process;

// Custom error type for the application
#[derive(Debug)]
enum QuadraticError {
    InvalidArguments(String),
    PlottingError(String),
}

impl fmt::Display for QuadraticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuadraticError::InvalidArguments(msg) => write!(f, "Invalid arguments: {}", msg),
            QuadraticError::PlottingError(msg) => write!(f, "Plotting error: {}", msg),
        }
    }
}

impl Error for QuadraticError {}

impl From<Box<dyn Error>> for QuadraticError {
    fn from(err: Box<dyn Error>) -> Self {
        QuadraticError::PlottingError(err.to_string())
    }
}

// Represents the roots of a quadratic equation
#[derive(Debug)]
struct Roots {
    x1: f64,
    x2: f64,
    // Complex part for complex roots
    imag: Option<f64>,
}

// Represents a quadratic equation
#[derive(Debug)]
struct QuadraticEquation {
    a: f64,
    b: f64,
    c: f64,
}

impl fmt::Display for QuadraticEquation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x^2 {:+}x {:+} = 0", self.a, self.b, self.c)
    }
}

impl QuadraticEquation {
    /// Creates a new quadratic equation from coefficients
    fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    /// Calculates the discriminant of the quadratic equation
    fn discriminant(&self) -> f64 {
        round_to_two_decimal_places(self.b.powi(2) - 4.0 * self.a * self.c)
    }

    /// Calculates the vertex of the parabola
    fn vertex(&self) -> (f64, f64) {
        let x_vertex = -self.b / (2.0 * self.a);
        let y_vertex = self.a * x_vertex.powi(2) + self.b * x_vertex + self.c;

        (
            round_to_two_decimal_places(x_vertex),
            round_to_two_decimal_places(y_vertex),
        )
    }

    /// Calculates the roots of the quadratic equation
    fn calculate_roots(&self) -> Roots {
        match self.discriminant().total_cmp(&0.0) {
            Ordering::Greater => self.two_real_roots(),
            Ordering::Equal => self.one_real_root(),
            Ordering::Less => self.two_complex_roots(),
        }
    }

    /// Calculates two real roots when discriminant > 0
    fn two_real_roots(&self) -> Roots {
        println!("Discriminant > 0\n");
        println!("Two real roots\n");

        let discriminant_sqrt = self.discriminant().sqrt();
        let x1 = (-self.b + discriminant_sqrt) / (2.0 * self.a);
        let x2 = (-self.b - discriminant_sqrt) / (2.0 * self.a);

        println!(
            "x_1 = {}, x_2 = {}",
            round_to_two_decimal_places(x1),
            round_to_two_decimal_places(x2)
        );

        Roots { x1, x2, imag: None }
    }

    /// Calculates one real root when discriminant = 0
    fn one_real_root(&self) -> Roots {
        println!("Discriminant = 0\n");
        println!("One real root\n");

        let x1 = -self.b / (2.0 * self.a);
        println!("x = {}", round_to_two_decimal_places(x1));

        Roots {
            x1,
            x2: x1,
            imag: None,
        }
    }

    /// Calculates two complex roots when discriminant < 0
    fn two_complex_roots(&self) -> Roots {
        println!("Discriminant < 0\n");
        println!("Two complex roots\n");

        let discriminant_abs = self.discriminant().abs();
        let imaginary = discriminant_abs.sqrt() / (2.0 * self.a);
        let real = -self.b / (2.0 * self.a);

        println!(
            "x_1 = {} {:+}i",
            round_to_two_decimal_places(real),
            round_to_two_decimal_places(imaginary)
        );
        println!(
            "x_2 = {} {:+}i",
            round_to_two_decimal_places(real),
            round_to_two_decimal_places(-imaginary)
        );

        Roots {
            x1: real,
            x2: real,
            imag: Some(imaginary),
        }
    }

    /// Plots the quadratic equation
    fn plot(&self, roots: &Roots, vertex: (f64, f64)) -> Result<(), QuadraticError> {
        let solving_caption = self.to_string();

        // Create a bitmap and drawing area
        let root = BitMapBackend::new("parabola.png", (800, 700)).into_drawing_area();

        root.fill(&WHITE)
            .map_err(|e| QuadraticError::PlottingError(e.to_string()))?;

        // Calculate plot boundaries
        // For complex roots or when x1 and x2 are both 0, use vertex-centered boundaries
        let (x_min, x_max) = if roots.imag.is_some() || (roots.x1 == 0.0 && roots.x2 == 0.0) {
            // Center on vertex with a reasonable range
            (vertex.0 - 5.0, vertex.0 + 5.0)
        } else {
            // For real roots, include both roots and the vertex
            let min_x = roots.x1.min(roots.x2).min(vertex.0);
            let max_x = roots.x1.max(roots.x2).max(vertex.0);

            // Add some padding
            let padding = (max_x - min_x) * 0.2;
            (min_x - padding, max_x + padding)
        };

        // Calculate y-range based on the function values within our x-range
        let y_at_min = self.a * x_min.powi(2) + self.b * x_min + self.c;
        let y_at_max = self.a * x_max.powi(2) + self.b * x_max + self.c;

        let min_y = vertex.1.min(y_at_min).min(y_at_max);
        let max_y = vertex.1.max(y_at_min).max(y_at_max);

        // Add padding to y-range
        let y_padding = (max_y - min_y) * 0.2;
        let y_min = min_y - y_padding;
        let y_max = max_y + y_padding;

        // Create chart
        let mut chart = ChartBuilder::on(&root)
            .caption(solving_caption, ("Arial", 30).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .map_err(|e| QuadraticError::PlottingError(e.to_string()))?;

        // Draw grid lines
        chart
            .configure_mesh()
            .draw()
            .map_err(|e| QuadraticError::PlottingError(e.to_string()))?;

        // Draw the parabola
        let parabola_style = ShapeStyle {
            color: BLUE.mix(0.6),
            filled: false,
            stroke_width: 2,
        };

        // Generate 100 points for the parabola
        let points: Vec<(f64, f64)> = (0..=100)
            .map(|i| {
                let x = x_min + (x_max - x_min) * (i as f64 / 100.0);
                let y = self.a * x * x + self.b * x + self.c;
                (x, y)
            })
            .collect();

        chart
            .draw_series(LineSeries::new(points, parabola_style))
            .map_err(|e| QuadraticError::PlottingError(e.to_string()))?;

        // Draw vertex point and label
        let vertex_point = Circle::new(vertex, 4, ShapeStyle::from(&RED).filled());
        chart
            .draw_series(std::iter::once(vertex_point))
            .map_err(|e| QuadraticError::PlottingError(e.to_string()))?;

        let vertex_text = Text::new(
            format!("({}, {})", vertex.0, vertex.1),
            vertex,
            ("Arial", 15).into_font(),
        );
        chart
            .draw_series(std::iter::once(vertex_text))
            .map_err(|e| QuadraticError::PlottingError(e.to_string()))?;

        // Draw root points and labels if real roots exist
        if roots.imag.is_none() {
            // First root
            if roots.x1 != 0.0 {
                let root1_point = Circle::new((roots.x1, 0.0), 4, ShapeStyle::from(&RED).filled());
                chart
                    .draw_series(std::iter::once(root1_point))
                    .map_err(|e| QuadraticError::PlottingError(e.to_string()))?;

                let root1_text = Text::new(
                    format!("({}, 0.0)", round_to_two_decimal_places(roots.x1)),
                    (roots.x1, 0.0),
                    ("Arial", 15).into_font(),
                );
                chart
                    .draw_series(std::iter::once(root1_text))
                    .map_err(|e| QuadraticError::PlottingError(e.to_string()))?;
            }

            // Second root (if different from first)
            if roots.x2 != 0.0 && roots.x2 != roots.x1 {
                let root2_point = Circle::new((roots.x2, 0.0), 4, ShapeStyle::from(&RED).filled());
                chart
                    .draw_series(std::iter::once(root2_point))
                    .map_err(|e| QuadraticError::PlottingError(e.to_string()))?;

                let root2_text = Text::new(
                    format!("({}, 0.0)", round_to_two_decimal_places(roots.x2)),
                    (roots.x2, 0.0),
                    ("Arial", 15).into_font(),
                );
                chart
                    .draw_series(std::iter::once(root2_text))
                    .map_err(|e| QuadraticError::PlottingError(e.to_string()))?;
            }
        } else {
            // For complex roots, we'll just draw the parabola without text labels
            // This avoids potential font rendering issues
            println!("Note: Complex roots are not displayed on the plot");
        }

        Ok(())
    }
}

/// Rounds a floating point number to two decimal places
fn round_to_two_decimal_places(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

/// Parses command line arguments and returns a QuadraticEquation
fn parse_args() -> Result<QuadraticEquation, QuadraticError> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        return Err(QuadraticError::InvalidArguments(
            "Expected 3 arguments: a, b, c".to_string(),
        ));
    }

    let parse_coefficient = |s: &String, name: &str| -> Result<f64, QuadraticError> {
        s.trim().parse::<f64>().map_err(|_| {
            QuadraticError::InvalidArguments(format!(
                "Could not parse '{}' as a number for coefficient {}",
                s, name
            ))
        })
    };

    let a = parse_coefficient(&args[1], "a")?;
    let b = parse_coefficient(&args[2], "b")?;
    let c = parse_coefficient(&args[3], "c")?;

    // Check if 'a' is zero, which would make this a linear equation, not quadratic
    if a == 0.0 {
        return Err(QuadraticError::InvalidArguments(
            "Coefficient 'a' cannot be zero (would make this a linear equation)".to_string(),
        ));
    }

    Ok(QuadraticEquation::new(a, b, c))
}

fn main() -> Result<(), QuadraticError> {
    println!("Quadratic equation solver\n");

    // Parse command line arguments
    let equation = match parse_args() {
        Ok(eq) => eq,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!(
                "Usage: {} a b c",
                env::args()
                    .next()
                    .unwrap_or_else(|| String::from("program"))
            );
            process::exit(1);
        }
    };

    // Display the coefficients
    println!("a = {}", equation.a);
    println!("b = {}", equation.b);
    println!("c = {}", equation.c);

    // Display the equation being solved
    println!("{}", equation);

    // Calculate and display the discriminant
    let discriminant = equation.discriminant();
    println!("\nDiscriminant = {}\n", discriminant);

    // Calculate the vertex
    let vertex = equation.vertex();

    // Calculate the roots
    let roots = equation.calculate_roots();

    // Display the vertex
    println!("\nVertex ({}, {})", vertex.0, vertex.1);

    // Plot the equation
    equation.plot(&roots, vertex)?;

    Ok(())
}
