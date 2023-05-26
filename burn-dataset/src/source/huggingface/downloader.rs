use std::fs;
use std::path::Path;
use std::process::Command;

use crate::dataset::base_dir;
use crate::SqliteDataset;

use sanitize_filename::sanitize;
use serde::de::DeserializeOwned;
use thiserror::Error;

const PYTHON: &str = "python3";
const PYTHON_SOURCE: &str = include_str!("importer.py");

#[derive(Error, Debug)]
pub enum ImporterError {
    #[error("unknown: `{0}`")]
    Unknown(String),
    #[error("fail to download python dependencies: `{0}`")]
    FailToDownloadPythonDependencies(String),
}

/// Load a dataset from [huggingface datasets](https://huggingface.co/datasets).
///
/// The dataset with all splits is stored in a single sqlite database (see [SqliteDataset](SqliteDataset)).
///
/// # Example
/// ```no_run
///  use burn_dataset::HuggingfaceDatasetLoader;
///  use burn_dataset::SqliteDataset;
///  use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, Debug, Clone)]
/// struct MNISTItemRaw {
///     pub image_bytes: Vec<u8>,
///     pub label: usize,
/// }
///
///  let train_ds:SqliteDataset<MNISTItemRaw> = HuggingfaceDatasetLoader::new("mnist")
///       .dataset("train")
///       .unwrap();
pub struct HuggingfaceDatasetLoader {
    name: String,
    subset: Option<String>,
    base_dir: Option<String>,
    huggingface_token: Option<String>,
    huggingface_cache_dir: Option<String>,
}

impl HuggingfaceDatasetLoader {
    /// Create a huggingface dataset loader.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            subset: None,
            base_dir: None,
            huggingface_token: None,
            huggingface_cache_dir: None,
        }
    }

    /// Create a huggingface dataset loader for a subset of the dataset.
    ///
    /// The subset name must be one of the subsets listed in the dataset page.
    ///
    /// If no subset names are listed, then do not use this method.
    pub fn with_subset(mut self, subset: &str) -> Self {
        self.subset = Some(subset.to_string());
        self
    }

    /// Specify a base directory to store the dataset.
    ///
    /// If not specified, the dataset will be stored in `~/.cache/burn-dataset`.
    pub fn with_base_dir(mut self, base_dir: &str) -> Self {
        self.base_dir = Some(base_dir.to_string());
        self
    }

    /// Specify a huggingface token to download datasets behind authentication.
    ///
    /// You can get a token from https://huggingface.co/settings/tokens
    pub fn with_huggingface_token(mut self, huggingface_token: &str) -> Self {
        self.huggingface_token = Some(huggingface_token.to_string());
        self
    }

    /// Specify a huggingface cache directory to store the downloaded datasets.
    ///
    /// If not specified, the dataset will be stored in `~/.cache/huggingface/datasets`.
    pub fn with_huggingface_cache_dir(mut self, huggingface_cache_dir: &str) -> Self {
        self.huggingface_cache_dir = Some(huggingface_cache_dir.to_string());
        self
    }

    /// Load the dataset.
    pub fn dataset<I: DeserializeOwned + Clone>(
        self,
        split: &str,
    ) -> Result<SqliteDataset<I>, ImporterError> {
        let db_file = self.db_file()?;
        let dataset = SqliteDataset::from_db_file(db_file.as_str(), split, false);
        Ok(dataset)
    }

    /// Get the path to the sqlite database file.
    ///
    /// If the database file does not exist, it will be downloaded and imported.
    pub fn db_file(self) -> Result<String, ImporterError> {
        // determine (and create if needed) the base directory
        let base_dir = base_dir(self.base_dir);

        //sanitize the name and subset
        let name = sanitize(self.name.as_str());

        // create the db file path
        let db_file = if let Some(subset) = self.subset.clone() {
            format!("{}/{}-{}.db", base_dir, name, sanitize(subset.as_str()))
        } else {
            format!("{}/{}.db", base_dir, name)
        };

        // import the dataset if needed
        if !Path::new(&db_file).exists() {
            import(
                self.name,
                self.subset,
                db_file.clone(),
                base_dir,
                self.huggingface_token,
                self.huggingface_cache_dir,
            )?;
        }

        Ok(db_file)
    }
}

/// Import a dataset from huggingface. The transformed dataset is stored as sqlite database.
fn import(
    name: String,
    subset: Option<String>,
    base_file: String,
    base_dir: String,
    huggingface_token: Option<String>,
    huggingface_cache_dir: Option<String>,
) -> Result<(), ImporterError> {
    install_python_deps()?;

    let mut command = Command::new(PYTHON);

    command.arg(importer_script_path(base_dir));

    command.arg("--name");
    command.arg(name);

    command.arg("--file");
    command.arg(base_file);

    if let Some(subset) = subset {
        command.arg("--subset");
        command.arg(subset);
    }

    if let Some(huggingface_token) = huggingface_token {
        command.arg("--token");
        command.arg(huggingface_token);
    }

    if let Some(huggingface_cache_dir) = huggingface_cache_dir {
        command.arg("--cache_dir");
        command.arg(huggingface_cache_dir);
    }

    let mut handle = command.spawn().unwrap();
    handle
        .wait()
        .map_err(|err| ImporterError::Unknown(format!("{err:?}")))?;

    Ok(())
}

fn importer_script_path(base_dir: String) -> String {
    let path_file = format!("{base_dir}/importer.py");
    fs::write(path_file.as_str(), PYTHON_SOURCE).expect("Write python dataset downloader");
    path_file
}

fn install_python_deps() -> Result<(), ImporterError> {
    let mut command = Command::new(PYTHON);
    command.args([
        "-m",
        "pip",
        "--quiet",
        "install",
        "pyarrow",
        "sqlalchemy",
        "Pillow",
        "soundfile",
        "datasets",
    ]);

    // Spawn the process and wait for it to complete.
    let mut handle = command.spawn().unwrap();
    handle.wait().map_err(|err| {
        ImporterError::FailToDownloadPythonDependencies(format!(" error: {}", err))
    })?;

    Ok(())
}
