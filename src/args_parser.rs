pub fn parse_args(args: Vec<String>) -> QuiddlerSolverArgs {
    let mut quiddler_solver_args = QuiddlerSolverArgs::default();

    for arg in &args[1..] {
        match arg.to_lowercase().as_str() {
            "-h" | "--help" => {
                quiddler_solver_args.help = true;
                return quiddler_solver_args
            },
            "--skip_solving" => quiddler_solver_args.skip_solving = true,
            "--skip_sorting" => quiddler_solver_args.skip_sorting = true,
            "--no_moving" => quiddler_solver_args.no_moving = true,
            _ => (),
        }
    }

    return quiddler_solver_args;
}

#[derive(Debug)]
pub struct QuiddlerSolverArgs {
    pub help: bool,
    pub skip_solving: bool,
    pub skip_sorting: bool,
    pub no_moving: bool,
}
impl Default for QuiddlerSolverArgs {
    fn default() -> Self {
        QuiddlerSolverArgs {
            help: false,
            skip_solving: false,
            skip_sorting: false,
            no_moving: false,
        }
    }
}
