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
    println!("Zadejte požadovanou přesnost: ");
    let epsilon = read_float();

    let c_1_exact = ((beta + 0.5) + f32::exp(-2.0 * PI) * (0.5 - alpha)) /
                    (2.0 * f32::sinh(2.0 * PI));
    let c_2_exact = alpha - 0.5 - c_1_exact;

    let (c_1_approx, c_2_approx) = halve(alpha, beta, epsilon);

    write_solution(c_1_exact, c_2_exact, c_1_approx, c_2_approx);
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

fn write_solution(c_1_exact: f32, c_2_exact: f32, c_1_approx: f32, c_2_approx: f32) {
    let head = "set terminal qt size 1000,1000 enhanced font 'Verdana,10' persist
# Line width of the axes
set border linewidth 1.5
# Line styles
set style line 1 linecolor rgb '#0060ad' linetype 1 linewidth 2
set style line 2 linecolor rgb '#dd181f' linetype 1 linewidth 2
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
c_1_approx = ";

    let line3 = "
c_2_approx = ";

    let tail = "
# Fuction
exact(x) = c_1_exact * exp(4 * x) + c_2_exact * exp(-4 * x) + 0.5
approx(x) = c_1_approx * exp(4 * x) + c_2_approx * exp(-4 * x) + 0.5
# Plot
plot exact(x) title 'exaktní řešení' with lines linestyle 1, \
     approx(x) title 'přibližné řešení' with lines linestyle 2";

    let path = Path::new("./solution.gnu");

    let mut file = File::create(&path).expect("Nepodařilo se otevřít soubor pro zápis.");
    file.write_all(head.as_bytes())
        .expect("Nepodařilo se zapsat do souboru.");
    file.write_all(c_1_exact.to_string().as_bytes())
        .expect("Nepodařilo se zapsat do souboru.");
    file.write_all(line1.as_bytes())
        .expect("Nepodařilo se zapsat do souboru.");
    file.write_all(c_2_exact.to_string().as_bytes())
        .expect("Nepodařilo se zapsat do souboru.");
    file.write_all(line2.as_bytes())
        .expect("Nepodařilo se zapsat do souboru.");
    file.write_all(c_1_approx.to_string().as_bytes())
        .expect("Nepodařilo se zapsat do souboru.");
    file.write_all(line3.as_bytes())
        .expect("Nepodařilo se zapsat do souboru.");
    file.write_all(c_2_approx.to_string().as_bytes())
        .expect("Nepodařilo se zapsat do souboru.");
    file.write_all(tail.as_bytes())
        .expect("Nepodařilo se zapsat do souboru.");
}

fn halve(alpha: f32, beta: f32, epsilon: f32) -> (f32, f32) {
    let c_1 = |a: f32, c: f32| -> f32 { 0.5 * a + 0.125 * c - 0.25 };
    let c_2 = |a: f32, c: f32| -> f32 { 0.5 * a - 0.125 * c + 0.25 };

    let f = |c: f32| -> f32 {
        c_1(alpha, c) * f32::exp(2.0 * PI) + c_2(alpha, c) * f32::exp(-2.0 * PI) + 0.5 - beta
    };

    let mut gamma_left = -1.0;
    let mut gamma_right = 1.0;

    for _ in 1..100000 {
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

    loop {
        let gamma = 0.5 * (gamma_left + gamma_right);
        let middle = f(gamma);
        if f32::abs(middle) < epsilon {
            return (c_1(alpha, gamma), c_2(alpha, gamma));
        }
        if middle * f(gamma_right) > 0.0 {
            gamma_right = gamma;
        } else {
            gamma_left = gamma;
        }
    }
}
