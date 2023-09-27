// rustc ma.rs && ./ma
// rustfmt ma.rs

// y''' = - cos(x), y(0) = 0, y'(0) = 1, y''(0) = 0,
// y(x) = sin(x)

use std::f64;

fn main() {
    const N: f64 = 3.0;
    let h = 0.1;

    let mut x = 0.0;
    let mut y0 = 0.0;
    let mut p0 = 1.0; // y' = p
    let mut q0 = 0.0; // y'' = q || p' = q

    while x < N {
        let q1 = q0 + h * fun(x);
        let p1 = p0 + h * q0;
        let y1 = y0 + h * p0;
        x += h;

        //println!("x = {:.1}, y = {:.3}", x, y1);
        println!("({:.1}, {:.1}),", x, y1);

        q0 = q1;
        p0 = p1;
        y0 = y1;
    }

    x = 0.;
    print!("\nTrue solve:\n");

    while x < N {
        x += h;
        let y = fun_solve(x);
        //println!("x_acc = {:.1}, y_acc = {:.3}", x, y);
        println!("({:.1}, {:.1}),", x, y);
    }
}

fn fun(x: f64) -> f64 {
    // y''' = fun(x)
    return -f64::cos(x);
}

fn fun_solve(x: f64) -> f64 {
    return f64::sin(x);
}
