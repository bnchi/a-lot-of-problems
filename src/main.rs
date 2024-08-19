use anyhow::Context;
use rev_buf_reader::RevBufReader;
use std::env::Args;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process::{exit, Command};

fn main() -> anyhow::Result<()> {
    start()?;
    Ok(())
}

struct Vars {
    problem_id: u64,
    data_structure: DataStructure,
}

enum DataStructure {
    Default,
}

fn start() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next();

    let Some(command) = args.next() else {
        panic!("Must provide command");
    };

    match command.as_str() {
        "latest" | "l" => run_latest_problem()?,
        "problem" => run_problem(&mut args)?,
        "scaffold" => scaffold_problem(&mut args)?,
        _ => panic!("The command is not correct"),
    };

    Ok(())
}

fn run_problem(args: &mut Args) -> anyhow::Result<()> {
    let problem_id = args
        .next()
        .unwrap()
        .parse::<u64>()
        .context("The problem id must be a valid u64")?;

    let status = Command::new("cargo")
        .arg("test")
        .arg(format!("test_{}", problem_id))
        .status()?;

    if !status.success() {
        exit(1);
    }

    Ok(())
}

fn run_latest_problem() -> anyhow::Result<()> {
    let file =
        std::fs::File::open(Path::new("src/solutions.rs")).expect("The solutions file must exist");
    let buf = RevBufReader::new(file);
    let last_line: String = buf
        .lines()
        .take(1)
        .map(|value| value.expect("to parse the line"))
        .collect();

    let (_, problem_id) = last_line.split_once("_").unwrap();
    let problem_id = &problem_id[..problem_id.len() - 1];

    let status = Command::new("cargo")
        .arg("test")
        .arg(format!("test_{}", problem_id))
        .status()?;

    if !status.success() {
        exit(1);
    }

    Ok(())
}

fn scaffold_problem(args: &mut Args) -> anyhow::Result<()> {
    let Some(problem_id) = args.next() else {
        panic!("Must provide the problem id");
    };

    let problem_id = problem_id
        .parse::<u64>()
        .context("The problem id must be a valid u64")?;

    let file_name = format!("src/solutions/s_{}.rs", problem_id);
    let path = Path::new(&file_name);
    let mut file = std::fs::File::create(path)?;

    let data_structure = get_data_structure_type(args.next());

    let vars = Vars {
        problem_id,
        data_structure,
    };

    file.write_all(get_source(vars)?.as_bytes())?;
    let mut solutions_mod = std::fs::File::options()
        .append(true)
        .open(Path::new("src/solutions.rs"))?;
    writeln!(&mut solutions_mod, "mod s_{};", problem_id)?;
    println!("Solution scaffolded!");
    Ok(())
}

fn get_data_structure_type(user_input: Option<String>) -> DataStructure {
    if let Some(user_input) = user_input {
        match user_input {
            _ => DataStructure::Default,
        }
    } else {
        DataStructure::Default
    }
}

fn get_source(vars: Vars) -> anyhow::Result<String> {
    let content = match vars.data_structure {
        DataStructure::Default => std::fs::read_to_string(Path::new("templates/default.txt"))?,
    };

    Ok(assing_vars(&content, vars))
}

fn assing_vars(content: &str, vars: Vars) -> String {
    content.replace("{{problem_id}}", &vars.problem_id.to_string())
}
