extern crate lp_modeler;

use lp_modeler::dsl::*;
use lp_modeler::format::lp_format::LpFileFormat;
use lp_modeler::solvers::{SolverTrait, CbcSolver};

// fn solve_problem(name: &'static str) {
fn solve_problem(name: &'static str) {
    let ref a = LpInteger::new("a").lower_bound(1.0);
    let ref b = LpInteger::new("b").upper_bound(10.0);
    let ref c = LpInteger::new("c").lower_bound(2.0).upper_bound(8.5);
    let ref d = LpBinary::new("d");
    let ref e = LpContinuous::new("e");

    // let mut problem = LpProblem::new("One Problem", LpObjective::Maximize);
    let mut problem = LpProblem::new(name, LpObjective::Maximize);
    problem += a + b + c + d + e;

    problem += (a + b + c + d + e).le(100.0);

    let output1 = problem.to_lp_file_format();
    println!("output1:\n{}", output1);

    let solver = CbcSolver::new();

    match solver.run(&problem) {
        Ok( solution ) => {
            println!("Status {:?}", solution.status);
            for (name, value) in solution.results.iter() {
                println!("value of {} = {}", name, value);
            }
        }
        Err(msg) => println!("{}", msg),
    }

    for expr in vec!("e free", "1 <= a", "2 <= c <= 8.5", "b <= 10") {
        // assert!(output1.contains(expr), format!("{} is not present",expr));
        assert!(output1.contains(expr));
    }
    ()
}

fn main() {
    solve_problem("problem one");

    // E0597 below, how do I fix this?
    // println!("input the problem name:");
    // let mut name = String::new();
    // std::io::stdin().read_line(&mut name);
    // let n = name.as_str();
    // solve_problem(&n);
    
}