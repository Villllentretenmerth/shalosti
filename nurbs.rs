use std::fs::File;
use std::io::{Read, Write};

#[derive(Clone, Copy)]
struct Span {
    l: f64,
    r: f64,
    x: [f64; 4],
    y: [f64; 4],
    z: [f64; 4],
}

impl Span {
    fn speed(&self, t: f64) -> f64 {
        let dt = t - self.l;

        let dx = self.x[1]
            + 2.0 * self.x[2] * dt
            + 3.0 * self.x[3] * dt * dt;

        let dy = self.y[1]
            + 2.0 * self.y[2] * dt
            + 3.0 * self.y[3] * dt * dt;

        let dz = self.z[1]
            + 2.0 * self.z[2] * dt
            + 3.0 * self.z[3] * dt * dt;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn simpson<F: Fn(f64) -> f64>(f: &F, a: f64, b: f64) -> f64 {
    let c = (a + b) / 2.0;
    (b - a) / 6.0 * (f(a) + 4.0 * f(c) + f(b))
}

fn adaptive_simpson<F: Fn(f64) -> f64>(
    f: &F,
    a: f64,
    b: f64,
    eps: f64,
    whole: f64,
    depth: u32,
) -> f64 {
    let c = (a + b) / 2.0;
    let left = simpson(f, a, c);
    let right = simpson(f, c, b);

    if depth == 0 || (left + right - whole).abs() < 15.0 * eps {
        return left + right + (left + right - whole) / 15.0;
    }

    adaptive_simpson(f, a, c, eps / 2.0, left, depth - 1)
        + adaptive_simpson(f, c, b, eps / 2.0, right, depth - 1)
}

fn main() {
    // Чтение из input.txt
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut spans = Vec::with_capacity(n);

    for _ in 0..n {
        let l: f64 = iter.next().unwrap().parse().unwrap();
        let r: f64 = iter.next().unwrap().parse().unwrap();

        let mut x = [0.0; 4];
        let mut y = [0.0; 4];
        let mut z = [0.0; 4];

        for i in 0..4 {
            x[i] = iter.next().unwrap().parse().unwrap();
        }
        for i in 0..4 {
            y[i] = iter.next().unwrap().parse().unwrap();
        }
        for i in 0..4 {
            z[i] = iter.next().unwrap().parse().unwrap();
        }

        spans.push(Span { l, r, x, y, z });
    }

    let mut total_length = 0.0;

    for span in spans {
        let f = |t: f64| span.speed(t);
        let whole = simpson(&f, span.l, span.r);
        total_length += adaptive_simpson(&f, span.l, span.r, 1e-10, whole, 20);
    }

    // Запись в output.txt
    let mut output = File::create("output.txt").unwrap();
    write!(output, "{:.15}", total_length).unwrap();
}