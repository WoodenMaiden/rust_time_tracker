mod args;
use clap::{Parser};
use crate::args::{CLIArgs, Commands};

fn main() {
    let cli = CLIArgs::parse();

    match &cli.command {
        Commands::Create { project, task } => {
            println!("'myapp test' was used, project is: {:?}, task is {:?}", project, task)
        },
        Commands::Delete { project, force, task } => {
            println!("'myapp delete' was used, project is: {:?}, task is {:?}, force is {:?}", project, task, force)
        },
        Commands::Show { project, task, pretty } => {
            println!("'myapp show' was used, project is: {:?}, task is {:?}, pretty is {:?}", project, task, pretty)
        },
        Commands::Archive { project } => {
            println!("'myapp archive' was used, project is: {:?}", project)
        },
        Commands::Unarchive { project } => {
            println!("'myapp unarchive' was used, project is: {:?}", project)
        },
        Commands::Amend { tochange, project, task } => {
            println!("'myapp amend' was used, tochange is: {:?}, project is: {:?}, task is {:?}", tochange, project, task)
        },
        Commands::Start { project, task } => {
            println!("'myapp start' was used, project is: {:?}, task is {:?}", project, task)
        },
        Commands::Stop { project } => {
            println!("'myapp stop' was used, project is: {:?}", project)
        },
        Commands::Pause {} => {
            println!("'myapp pause' was used")
        }
    }
}
