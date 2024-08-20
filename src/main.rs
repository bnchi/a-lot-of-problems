use anyhow::Context;
use rev_buf_reader::RevBufReader;
use std::env::Args;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process::{exit, Command};

trait AppCommand {
    fn init(context: AppContext) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn run(&self) -> anyhow::Result<()>;
}

struct RunProblem {
    problem_id: u64,
}

impl AppCommand for RunProblem {
    fn init(mut context: AppContext) -> anyhow::Result<Self> {
        let problem_id = context.next_arg("problem id")?;
        let problem_id = problem_id
            .parse::<u64>()
            .context("The problem id must be a valid u64")?;
        Ok(RunProblem { problem_id })
    }

    fn run(&self) -> anyhow::Result<()> {
        let status = Command::new("cargo")
            .arg("test")
            .arg(format!("test_{}", self.problem_id))
            .status()?;

        if !status.success() {
            exit(1);
        }

        Ok(())
    }
}

struct Template;

impl Template {
    fn get(ds: &DataStructure, problem_id: u64) -> anyhow::Result<String> {
        let content = match ds {
            DataStructure::Default => std::fs::read_to_string(Path::new("templates/default.txt"))?,
        };
        Ok(Self::replace_mustache(content, problem_id))
    }

    fn replace_mustache(content: String, problem_id: u64) -> String {
        content.replace("{{problem_id}}", &problem_id.to_string())
    }
}

enum DataStructure {
    Default,
}

impl From<String> for DataStructure {
    fn from(value: String) -> Self {
        match value {
            _ => DataStructure::Default,
        }
    }
}

struct Scaffold {
    problem_id: u64,
    ds: DataStructure,
}

impl AppCommand for Scaffold {
    fn init(mut context: AppContext) -> anyhow::Result<Self> {
        let problem_id = context.next_arg("problem id")?;
        let problem_id = problem_id
            .parse::<u64>()
            .context("The problem id must be a valid u64")?;
        let ds = DataStructure::from(context.next_arg("data structure")?);
        Ok(Scaffold { problem_id, ds })
    }

    fn run(&self) -> anyhow::Result<()> {
        let file_name = format!("src/solutions/s_{}.rs", self.problem_id);
        let path = Path::new(&file_name);
        let mut file = std::fs::File::create(path)?;
        file.write_all(Template::get(&self.ds, self.problem_id)?.as_bytes())?;
        let mut solutions_mod = std::fs::File::options()
            .append(true)
            .open(Path::new("src/solutions.rs"))?;
        writeln!(&mut solutions_mod, "mod s_{};", self.problem_id)?;
        println!("Solution scaffolded!");
        Ok(())
    }
}

struct AppContext {
    args: Args,
    command: String,
}

impl AppContext {
    pub fn new(mut args: Args) -> Self {
        // discard the first argument
        args.next();

        let Some(command) = args.next() else {
            panic!("Must provide a valid command");
        };

        Self { args, command }
    }

    pub fn next_arg(&mut self, arg_name: &str) -> anyhow::Result<String> {
        self.args.next().ok_or(anyhow::anyhow!(
            "Must provide the {} as a an argument",
            arg_name
        ))
    }
}

fn start(context: AppContext) -> anyhow::Result<()> {
    match context.command.as_str() {
        //"latest" | "l" => run_latest_problem()?,
        "problem" => run_command::<RunProblem>(context)?,
        "scaffold" => run_command::<Scaffold>(context)?,
        _ => anyhow::bail!("The command {} can't be recognized", context.command),
    };

    Ok(())
}

fn run_command<T: AppCommand>(context: AppContext) -> anyhow::Result<()> {
    let command = T::init(context)?;
    command.run()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let context = AppContext::new(std::env::args());
    start(context)?;
    Ok(())
}

//fn run_latest_problem() -> anyhow::Result<()> {
//    let file =
//        std::fs::File::open(Path::new("src/solutions.rs")).expect("The solutions file must exist");
//    let buf = RevBufReader::new(file);
//    let last_line: String = buf
//        .lines()
//        .take(1)
//        .map(|value| value.expect("to parse the line"))
//        .collect();
//
//    let (_, problem_id) = last_line.split_once("_").unwrap();
//    let problem_id = &problem_id[..problem_id.len() - 1];
//
//    let status = Command::new("cargo")
//        .arg("test")
//        .arg(format!("test_{}", problem_id))
//        .status()?;
//
//    if !status.success() {
//        exit(1);
//    }
//
//    Ok(())
//}
