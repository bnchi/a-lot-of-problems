use std::io::Write;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    scaffolde_solution()?;
    println!("Solution created!");
    Ok(())
}

enum DataStructure {
    Default,
}

// @todo add more commands to get stats about the problems i've solved and a search to find a
// solved problem
fn scaffolde_solution() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next();

    let Some(problem_number) = args.next() else {
        panic!("Must provide the problem number as a first argument");
    };

    let file_name = format!("src/solutions/s_{}.rs", problem_number);
    let path = Path::new(&file_name);
    let mut file = std::fs::File::create(path)?;
    let data_structure = get_data_structure_type(args.next());
    file.write_all(get_source(data_structure)?.as_bytes())?;
    let mut solutions_mod = std::fs::File::options()
        .append(true)
        .open(Path::new("src/solutions.rs"))?;
    writeln!(&mut solutions_mod, "mod s_{};", problem_number)?;

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

fn get_source(data_structure: DataStructure) -> anyhow::Result<String> {
    let content = match data_structure {
        DataStructure::Default => std::fs::read_to_string(Path::new("src/templates/default.txt"))?,
    };

    Ok(content)
}
