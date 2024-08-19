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

enum DataStructure {
    Default,
}

impl DataStructure {
    fn get_problem_ds(arg: anyhow::Result<String>) -> DataStructure {
        if let Ok(ds_choosen) = arg {
            match ds_choosen {
                _ => DataStructure::Default,
            }
        } else {
            DataStructure::Default
        }
    }

    fn get_source_with_ds(ds: &Self) -> anyhow::Result<String> {
        let content = match ds {
            DataStructure::Default => std::fs::read_to_string(Path::new("templates/default.txt"))?,
        };

        Ok(content)
    }
}

struct Scaffold {
    problem_id: u64,
    ds: DataStructure,
}

impl AppCommand for Scaffold {
    fn init(mut context: AppContext) -> anyhow::Result<Self> {
        let Some(problem_id) = context.args.next() else {
            anyhow::bail!("The problem id must exist");
        };

        let problem_id = problem_id
            .parse::<u64>()
            .context("The problem id must be a valid u64")?;

        let ds = DataStructure::get_problem_ds(context.next_arg());
        Ok(Scaffold { problem_id, ds })
    }

    fn run(&self) -> anyhow::Result<()> {
        let file_name = format!("src/solutions/s_{}.rs", self.problem_id);
        let path = Path::new(&file_name);
        let mut file = std::fs::File::create(path)?;

        file.write_all(DataStructure::get_source_with_ds(&self.ds)?.as_bytes())?;

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
            panic!("Must provide a command");
        };

        Self { args, command }
    }

    pub fn next_arg(&mut self) -> anyhow::Result<String> {
        self.args
            .next()
            .ok_or(anyhow::anyhow!("The argument wasn't passed"))
    }
}

fn start(context: AppContext) -> anyhow::Result<()> {
    match context.command.as_str() {
        //"latest" | "l" => run_latest_problem()?,
        //"problem" => run_problem(&mut args)?,
        "scaffold" => run_command::<Scaffold>(context)?,
        _ => anyhow::bail!("The command can't be recognized"),
    };

    Ok(())
}

fn run_command<T: AppCommand>(context: AppContext) -> anyhow::Result<()> {
    let command = T::init(context)?;
    command.run()?;
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

fn main() -> anyhow::Result<()> {
    let context = AppContext::new(std::env::args());
    start(context)?;
    Ok(())
}

//fn assing_vars(content: &str, vars: Vars) -> String {
//    content.replace("{{problem_id}}", &vars.problem_id.to_string())
//}
