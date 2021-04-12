use structopt::StructOpt;

// -------------------------------------------------------------------------- //
// -------------------------------------------------------------------------- //
//                         INTERNAL MODULE INCLUSION                          //
// -------------------------------------------------------------------------- //
// -------------------------------------------------------------------------- //
mod downloader;
mod roverfile;
use crate::downloader::DatasetDownloader;
use crate::roverfile::Roverfile;

// -------------------------------------------------------------------------- //
// -------------------------------------------------------------------------- //
//                          COMMAND LINE ARG PARSING                          //
// -------------------------------------------------------------------------- //
// -------------------------------------------------------------------------- //
/// Structure defining the subcommands available in `rover`.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "rover",
    author = "Rolfe Power",
    about = "Simple utility for fetching files from the internet."
)]
enum Command {
    /// List available datasets and their descriptions.
    List,

    /// Retrieve datasets to the current directory.
    ///
    /// Fetch desired datasets to the current directory using the URLs and
    /// filenames defined in the `roverfile`.
    Fetch {
        /// Datasets to download to the current directory.
        #[structopt()]
        datasets: Vec<String>,
    },

    /// Show detailed information for specified datasets
    Info {
        /// Datasets to show information for
        datasets: Vec<String>,
    },
}

/// Main `StructOpt` structure to define the CLI.
#[derive(Debug, StructOpt)]
struct Rover {
    #[structopt(subcommand)]
    command: Command,
}

// -------------------------------------------------------------------------- //
// -------------------------------------------------------------------------- //
//                             MAIN ENTRY POINT                               //
// -------------------------------------------------------------------------- //
// -------------------------------------------------------------------------- //
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Rover::from_args();
    let rf = Roverfile::new(None)?;

    // Match on subcommand and perform desired operation
    match opt.command {
        Command::List => rf.list_to_console(),
        Command::Fetch { datasets } => {
            let dd = DatasetDownloader::new(&rf, &datasets);
            dd.execute()?;
        }
        Command::Info { datasets } => {
            for dsn in datasets {
                println!("{}", dsn);
                let d = rf.get_dataset(&dsn).unwrap();
                d.show_info();
                println!("\n");
            }
        }
    };

    Ok(())
}
