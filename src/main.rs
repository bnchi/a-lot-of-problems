use rev_buf_reader::RevBufReader;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process::{exit, Command};

use clap::{Args, Parser, Subcommand, ValueEnum};

/// Scaffold and track interview problems using cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Problem(Problem),
    Scaffold(Scaffold),
}

#[derive(Args)]
struct Problem {
    #[arg(long)]
    id: u64,
}

impl CommandRunner for Problem {
    fn run(&self) -> anyhow::Result<()> {
        let status = Command::new("cargo")
            .arg("test")
            .arg(format!("test_{}", self.id))
            .status()?;

        if !status.success() {
            exit(1);
        }

        Ok(())
    }
}

#[derive(Args)]
struct Scaffold {
    #[arg(long)]
    id: u64,
    #[arg(value_enum)]
    ds: DataStructure,
}

impl CommandRunner for Scaffold {
    fn run(&self) -> anyhow::Result<()> {
        let file_name = format!("src/solutions/s_{}.rs", self.id);
        let path = Path::new(&file_name);
        if path.exists() {
            anyhow::bail!("The problem was already solved!");
        }
        let mut file = std::fs::File::create(path)?;
        file.write_all(Template::get(&self.ds, self.id)?.as_bytes())?;
        let mut solutions_mod = std::fs::File::options()
            .append(true)
            .open(Path::new("src/solutions.rs"))?;
        writeln!(&mut solutions_mod, "mod s_{};", self.id)?;
        println!("Solution scaffolded!");
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DataStructure {
    Default,
}

trait CommandRunner {
    fn run(&self) -> anyhow::Result<()>;
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // we can use dynamic dispatch but ehh whatever
    match cli.cmd {
        Commands::Scaffold(scaffold) => scaffold.run()?,
        Commands::Problem(problem) => problem.run()?,
    };

    Ok(())
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
