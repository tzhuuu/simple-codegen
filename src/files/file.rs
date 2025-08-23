use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use iddqd::{IdHashItem, id_upcast};
use thiserror::Error;

use crate::scope::Scope;

/// Errors that can occur during file code generation
#[derive(Error, Debug)]
pub enum FileCodegenError {
    /// The file already exists
    #[error("File already exists: {0}")]
    FileAlreadyExists(PathBuf),

    /// The file generation failed
    #[error("File generation failed: {0}")]
    FileGenerationFailed(std::io::Error),
}

/// Defines a file.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct File {
    /// Relative path to the file
    path: PathBuf,

    /// Contents of the file
    scope: Scope,
}

impl IdHashItem for File {
    type Key<'a> = &'a Path;

    fn key(&self) -> Self::Key<'_> {
        &self.path
    }

    id_upcast!();
}

impl File {
    /// Creates a new file with the given path
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            scope: Scope::default(),
        }
    }

    /// Gets the file path.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Sets the file path.
    pub fn set_path(&mut self, path: impl Into<PathBuf>) -> &mut Self {
        self.path = path.into();
        self
    }

    /// Gets a mutable reference to the file path.
    pub fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }

    /// Gets the file scope.
    pub fn scope(&self) -> &Scope {
        &self.scope
    }

    /// Sets the file scope.
    pub fn set_scope(&mut self, scope: impl Into<Scope>) -> &mut Self {
        self.scope = scope.into();
        self
    }

    /// Sets the file scope.
    pub fn with_scope(mut self, scope: impl Into<Scope>) -> Self {
        self.set_scope(scope);
        self
    }

    /// Gets a mutable reference to the file scope.
    pub fn scope_mut(&mut self) -> &mut Scope {
        &mut self.scope
    }

    /// Writes to the file.
    pub fn generate<'a>(&self, out_dir: impl Into<&'a Path>) -> Result<(), FileCodegenError> {
        let out_dir = out_dir.into();
        let file_path = out_dir.join(self.path.as_path());

        if file_path.exists() {
            return Err(FileCodegenError::FileAlreadyExists(file_path));
        }

        if let Ok(mut file) = fs::File::create(file_path)
            && let Err(err) = file.write_all(self.scope.to_string().as_bytes()) {
                return Err(FileCodegenError::FileGenerationFailed(err));
            }

        Ok(())
    }
}
