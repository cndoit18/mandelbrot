use num::Complex;

fn main() {
    render_mandelbrot(calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 150, 80));
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> Option<usize> {
    let mut z = Complex::new(0_f64, 0_f64);
    let c = Complex::new(cx, cy);
    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<Option<usize>>> {
    let mut rows = Vec::with_capacity(width);
    for img_y in 0..=height {
        let mut row: Vec<Option<usize>> = Vec::with_capacity(height);
        for img_x in 0..=width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            row.push(mandelbrot_at_point(cx, cy, max_iters));
        }
        rows.push(row);
    }
    rows
}

fn render_mandelbrot(escape_vals: Vec<Vec<Option<usize>>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                Some(0..=2) => ' ',
                Some(3..=5) => '.',
                Some(6..=10) => '·',
                Some(11..=30) => '*',
                Some(31..=100) => '+',
                Some(101..=200) => 'x',
                Some(201..=400) => '$',
                Some(401..=700) => '#',
                None | Some(_) => '%',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}
