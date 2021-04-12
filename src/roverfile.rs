use directories::BaseDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Error, ErrorKind};
use std::path::PathBuf;
use toml;

const ROVERFILE_DEFAULT_NAME: &str = "roverfile";

/// Using the default file name and search paths, find the roverfile.
fn find_roverfile() -> Result<PathBuf, Error> {
    let rf_path = BaseDirs::new()
        .ok_or(Error::new(
            ErrorKind::NotFound,
            "could not find config dirs",
        ))?
        .config_dir()
        .join("rover")
        .join(ROVERFILE_DEFAULT_NAME);

    Ok(rf_path)
}

/// Dataset that can be downloaded.
#[derive(Debug, Deserialize)]
pub struct Dataset {
    pub filename: String,
    pub description: String,
    pub url: String,
}

impl Dataset {
    /// Download the dataset to the current directory.
    ///
    /// This writes to the file specified by the `filename` field.
    pub fn fetch(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut bw = BufWriter::new(File::create(&self.filename)?);
        let response = reqwest::blocking::get(&self.url)?;
        let data = response.bytes()?;
        bw.write_all(&data)?;
        bw.flush()?;
        Ok(())
    }

    pub fn show_info(&self) {
        println!("Filename    : {}", self.filename);
        println!("Description : {}", self.description);
        println!("Source      : {}", self.url);
    }
}

/// Roverfile that contains collection of datasets as well as metainformation.
#[derive(Debug, Deserialize)]
pub struct Roverfile {
    #[serde(default)]
    path: PathBuf,

    author: Option<String>,

    datasets: HashMap<String, Dataset>,
}

/// Read the contents of a file into a string.
fn slurp(path: &PathBuf) -> Result<String, Error> {
    let f = File::open(path)?;
    let mut breader = BufReader::new(f);
    let mut contents = String::new();
    breader.read_to_string(&mut contents)?;
    Ok(contents)
}

impl Roverfile {
    /// Create a new  Roverfile initializing from the on disk file at path `p`.
    pub fn new(p: Option<PathBuf>) -> Result<Self, Error> {
        let path = match p {
            Some(pt) => pt,
            None => find_roverfile()?,
        };
        let contents = slurp(&path)?;
        let mut out: Self = toml::from_str(&contents)?;
        out.path = path;
        Ok(out)
    }

    /// Return a list of dataset names available in the Roverfile.
    pub fn dataset_names(&self) -> Vec<&str> {
        let mut ret = Vec::new();

        for k in self.datasets.keys() {
            ret.push(k.as_str());
        }

        ret
    }

    /// Return the dataset with name `nm` defined in the Roverfile.
    pub fn get_dataset(&self, nm: &str) -> Option<&Dataset> {
        self.datasets.get(nm)
    }

    pub fn list_to_console(&self) {
        let mut ns = self.dataset_names();
        ns.sort();

        let maxlen = ns.iter().fold(0, |acc, &x| {
            let l = x.len();
            if l > acc {
                l
            } else {
                acc
            }
        });

        for n in ns {
            let d = &self.get_dataset(n).unwrap().description;
            println!("{:width$}\t{}", n, d, width = maxlen);
        }
    }
}
