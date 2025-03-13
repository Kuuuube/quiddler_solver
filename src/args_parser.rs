pub fn parse_args(args: Vec<String>) -> QuiddlerSolverArgs {
    let mut quiddler_solver_args = QuiddlerSolverArgs::default();

    for arg in &args[1..] {
        match arg.as_str() {
            "--skip_solving" => quiddler_solver_args.skip_solving = true,
            "--skip_sorting" => quiddler_solver_args.skip_sorting = true,
            _ => (),
        }
    }

    return quiddler_solver_args;
}

#[derive(Debug)]
pub struct QuiddlerSolverArgs {
    pub skip_solving: bool,
    pub skip_sorting: bool,
}
impl Default for QuiddlerSolverArgs {
    fn default() -> Self {
        QuiddlerSolverArgs {
            skip_solving: false,
            skip_sorting: false,
        }
    }
}
