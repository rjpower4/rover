// -------------------------------------------------------------------------- //
// -------------------------------------------------------------------------- //
//                         INTERNAL MODULE INCLUSION                          //
// -------------------------------------------------------------------------- //
// -------------------------------------------------------------------------- //
use crate::roverfile::Roverfile;

// -------------------------------------------------------------------------- //
// -------------------------------------------------------------------------- //
//                         DOWNLOADER IMPLEMENTATION                          //
// -------------------------------------------------------------------------- //
// -------------------------------------------------------------------------- //

/// Context information for dataset download
pub struct DatasetDownloader<'a> {
    rf: &'a Roverfile,
    datasets: &'a Vec<String>,
}

impl<'a> DatasetDownloader<'a> {
    pub fn new(rf: &'a Roverfile, datasets: &'a Vec<String>) -> Self {
        Self { rf, datasets }
    }

    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dss: Option<Vec<_>> = self
            .datasets
            .iter()
            .map(|x| self.rf.get_dataset(&x))
            .collect();

        // We will *NOT* download anything if any one dataset requested is not
        // found
        match dss {
            Some(datasets) => {
                for ds in datasets {
                    ds.fetch()?
                }
                Ok(())
            }
            None => {
                // Find the miscreants
                // We don't care about perf here as we're about to blow up
                let mut errstr = format!("Unknown dataset(s) encountered: ");
                self.datasets.into_iter().for_each(|dsn| {
                    if !self.rf.dataset_names().contains(&dsn.as_str()) {
                        errstr.push_str(dsn);
                        errstr.push_str(" ");
                    }
                });

                // Return the error identifying which inputs were bad
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    errstr,
                )))
            }
        }
    }
}
