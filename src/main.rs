extern crate rand;

use std::io;
use std::f32::consts::PI;

use std::fs::File;
use std::path::Path;
use std::io::Write;

use rand::Rng;

fn main() {
    println!("Zadejte parametr alfa: ");
    let alpha = read_float();
    println!("Zadejte parametr beta: ");
    let beta = read_float();
    println!("Zadejte počet iterací: ");
    let rounds = read_int();

    let (c_1_exact, c_2_exact) = exact(alpha, beta);
    let (c_1_halve, c_2_halve) = halve(alpha, beta, rounds);
    let (c_1_newton, c_2_newton) = newton(alpha, beta, rounds);

    write_solution(c_1_exact,
                   c_2_exact,
                   c_1_halve,
                   c_2_halve,
                   c_1_newton,
                   c_2_newton);
}

fn read_float() -> f32 {
    loop {
        let mut alpha_str = String::new();
        io::stdin()
            .read_line(&mut alpha_str)
            .expect("Čtení z příkazové řádky selhalo.");
        match alpha_str.trim().parse() {
            Ok(n) => {
                return n;
            }
            Err(_) => {
                println!("Musíte zadat číslo.");
            }
        }
    }
}

fn read_int() -> i32 {
    loop {
        let mut alpha_str = String::new();
        io::stdin()
            .read_line(&mut alpha_str)
            .expect("Čtení z příkazové řádky selhalo.");
        match alpha_str.trim().parse() {
            Ok(n) => {
                return n;
            }
            Err(_) => {
                println!("Musíte zadat číslo.");
            }
        }
    }
}

fn write_solution(c_1_exact: f32,
                  c_2_exact: f32,
                  c_1_halve: f32,
                  c_2_halve: f32,
                  c_1_newton: f32,
                  c_2_newton: f32) {
    let head = "set terminal qt size 1000,1000 enhanced font 'Verdana,10' persist
# Line width of the axes
set border linewidth 1.5
# Line styles
set style line 1 linecolor rgb '#0060ad' linetype 1 linewidth 2
set style line 2 linecolor rgb '#dd181f' linetype 1 linewidth 2
set style line 3 linecolor rgb '#00d100' linetype 1 linewidth 2
# Axes label
set xlabel 'x'
set ylabel 'y'
# Axes ranges
set xrange [0:0.5*pi]
set xtics ('0' 0, 'π/8' 0.125 * pi, 'π/4' 0.25 * pi, '3π/8' 0.375 * pi, 'π/2' 0.5 * pi)
# Paramters
c_1_exact = ";

    let line1 = "
c_2_exact = ";

    let line2 = "
c_1_halve = ";

    let line3 = "
c_2_halve = ";

    let line4 = "
c_1_newton = ";

    let line5 = "
c_2_newton = ";

    let tail = "
# Fuction
exact(x) = c_1_exact * exp(4 * x) + c_2_exact * exp(-4 * x) + 0.5
halve(x) = c_1_halve * exp(4 * x) + c_2_halve * exp(-4 * x) + 0.5
newton(x) = c_1_newton * exp(4 * x) + c_2_newton * exp(-4 * x) + 0.5
# Plot
plot exact(x) title 'exaktní řešení' with lines linestyle 1, \
     halve(x) title 'metoda půlení intervalů' with lines linestyle 2, \
     newton(x) title 'Newtonova metoda' with lines linestyle 3";

    let path = Path::new("./solution.gnu");

    let error = "Nepodařilo se zapsat do souboru.";

    let mut file = File::create(&path).expect("Nepodařilo se otevřít soubor pro zápis.");
    file.write_all(head.as_bytes()).expect(error);
    file.write_all(c_1_exact.to_string().as_bytes())
        .expect(error);
    file.write_all(line1.as_bytes()).expect(error);
    file.write_all(c_2_exact.to_string().as_bytes())
        .expect(error);
    file.write_all(line2.as_bytes()).expect(error);
    file.write_all(c_1_halve.to_string().as_bytes())
        .expect(error);
    file.write_all(line3.as_bytes()).expect(error);
    file.write_all(c_2_halve.to_string().as_bytes())
        .expect(error);
    file.write_all(line4.as_bytes()).expect(error);
    file.write_all(c_1_newton.to_string().as_bytes())
        .expect(error);
    file.write_all(line5.as_bytes()).expect(error);
    file.write_all(c_2_newton.to_string().as_bytes())
        .expect(error);
    file.write_all(tail.as_bytes()).expect(error);
}

fn exact(alpha: f32, beta: f32) -> (f32, f32) {
    let c_1 = ((2.0 * beta - 1.0) + f32::exp(-2.0 * PI) * (1.0 - 2.0 * alpha)) /
              (4.0 * f32::sinh(2.0 * PI));
    let c_2 = alpha - 0.5 - c_1;
    (c_1, c_2)
}


fn halve(alpha: f32, beta: f32, rounds: i32) -> (f32, f32) {
    let c_1 = |a: f32, c: f32| -> f32 { 0.5 * a + 0.125 * c - 0.25 };
    let c_2 = |a: f32, c: f32| -> f32 { 0.5 * a - 0.125 * c - 0.25 };

    let f = |c: f32| -> f32 {
        c_1(alpha, c) * f32::exp(2.0 * PI) + c_2(alpha, c) * f32::exp(-2.0 * PI) + 0.5 - beta
    };

    let mut gamma = 0.0;
    let mut gamma_left = -1.0;
    let mut gamma_right = 1.0;

    for _ in 0..100000 {
        gamma_left = rand::thread_rng().gen_range(-1000.0, 1000.0);
        gamma_right = rand::thread_rng().gen_range(-1000.0, 1000.0);
        if f(gamma_left) * f(gamma_right) < 0.0 {
            break;
        }
    }

    if f(gamma_left) * f(gamma_right) >= 0.0 {
        panic!("Nepodařilo se najít počáteční stav metody půlení intervalů");
    }

    println!("Půlím...");

    for _ in 0..rounds {
        gamma = 0.5 * (gamma_left + gamma_right);
        let middle = f(gamma);
        if middle * f(gamma_right) > 0.0 {
            gamma_right = gamma;
        } else {
            gamma_left = gamma;
        }
    }
    (c_1(alpha, gamma), c_2(alpha, gamma))
}

fn newton(alpha: f32, beta: f32, rounds: i32) -> (f32, f32) {
    let c_1 = |a: f32, c: f32| -> f32 { 0.5 * a + 0.125 * c - 0.25 };
    let c_2 = |a: f32, c: f32| -> f32 { 0.5 * a - 0.125 * c - 0.25 };

    let f = |c: f32| -> f32 {
        c_1(alpha, c) * f32::exp(2.0 * PI) + c_2(alpha, c) * f32::exp(-2.0 * PI) + 0.5 - beta
    };
    let f_der = |_: f32| -> f32 { 0.125 * f32::exp(2.0 * PI) - 0.125 * f32::exp(-2.0 * PI) };

    let mut gamma = rand::thread_rng().gen_range(-1000.0, 1000.0);

    for _ in 0..rounds {
        gamma = gamma - f(gamma) / f_der(gamma);
    }
    (c_1(alpha, gamma), c_2(alpha, gamma))
}
