use clap::{Subcommand, Parser};

#[derive(Subcommand)]
pub enum Commands {
    Create {
        #[clap(value_parser)]
        project: String,

        #[clap(short)]
        task: Option<String>, 
    },
    
    Delete {

        #[clap(short, takes_value = false)]
        force: bool,
        
        #[clap(value_parser)]
        project: String,

        #[clap(value_parser)]
        task: Option<String>,
    },
    
    Show {
        #[clap(short, takes_value = false)]
        pretty: bool,

        #[clap(value_parser)]
        project: String,

        #[clap(value_parser)]
        task: Option<String>, // TODO make it able to add several tasks
    },

    Archive {
        #[clap(value_parser)]
        project: String,
    },
    
    Unarchive {
        #[clap(value_parser)]
        project: String,
    },

    Amend {
        #[clap()]
        tochange: String,

        #[clap(value_parser)]
        project: String,
        
        #[clap(value_parser)]
        task: Option<String>,
    }, 

    //record
    Start {
        #[clap(value_parser)]
        project: String,

        #[clap(value_parser)]
        task: String,
    },
    Pause {},

    Stop {
        #[clap(value_parser)]
        project: String,
    },
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct CLIArgs {
    #[clap(subcommand)]
    pub command: Commands,
}
