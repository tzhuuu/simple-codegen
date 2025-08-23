use std::path::PathBuf;

use iddqd::IdHashMap;
use thiserror::Error;

use crate::files::file::{File, FileCodegenError};
/// Errors that can occur during library code generation
#[derive(Error, Debug)]
pub enum LibraryCodegenError {
    /// The file already exists
    #[error("File already exists: {0}")]
    FileAlreadyExists(PathBuf),

    /// The file generation failed
    #[error("File generation failed: {0}")]
    FileGenerationFailed(FileCodegenError),
}

/// Defines a library.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Library {
    /// Library name
    name: String,

    /// The library path
    path: PathBuf,

    /// The library file
    lib: File,

    /// Library contents
    files: IdHashMap<File>,
}

impl Library {
    /// Creates a new library with the given name
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            lib: File::new("lib.rs"),
            files: IdHashMap::new(),
        }
    }

    /// Gets the library name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the library name
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Sets the library name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }

    /// Gets a mutable reference to the library name
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// Gets the library path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Sets the library path
    pub fn set_path(&mut self, path: impl Into<PathBuf>) -> &mut Self {
        self.path = path.into();
        self
    }

    /// Sets the library path
    pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.set_path(path);
        self
    }

    /// Gets a mutable reference to the library path
    pub fn path_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }

    /// Gets the lib.rs file
    pub fn lib(&self) -> &File {
        &self.lib
    }

    /// Sets the lib.rs file
    pub fn set_lib(&mut self, lib: impl Into<File>) -> &mut Self {
        self.lib = lib.into();
        self
    }

    /// Sets the lib.rs file
    pub fn with_lib(mut self, lib: impl Into<File>) -> Self {
        self.set_lib(lib);
        self
    }

    /// Gets a mutable reference to the lib.rs file
    pub fn lib_mut(&mut self) -> &mut File {
        &mut self.lib
    }

    /// Gets the files
    pub fn files(&self) -> &IdHashMap<File> {
        &self.files
    }

    /// Sets the files
    pub fn set_files(&mut self, files: impl Into<IdHashMap<File>>) -> &mut Self {
        self.files = files.into();
        self
    }

    /// Sets the files
    pub fn with_files(mut self, files: impl Into<IdHashMap<File>>) -> Self {
        self.set_files(files);
        self
    }

    /// Gets a mutable reference to the files
    pub fn files_mut(&mut self) -> &mut IdHashMap<File> {
        &mut self.files
    }

    /// Pushes a file to the lib
    pub fn push_file(&mut self, file: impl Into<File>) -> Result<(), LibraryCodegenError> {
        let file = file.into();
        let file_path = file.path().clone();

        if file_path.as_path().to_str() == Some("lib.rs") {
            return Err(LibraryCodegenError::FileAlreadyExists(file_path));
        }

        if let Err(_duplicate_item) = self.files.insert_unique(file) {
            return Err(LibraryCodegenError::FileAlreadyExists(file_path));
        }
        Ok(())
    }

    /// Writes the files
    pub fn generate(&self) -> Result<(), LibraryCodegenError> {
        for file in self.files.iter() {
            if let Err(e) = file.generate(self.path.as_path()) {
                return Err(LibraryCodegenError::FileGenerationFailed(e));
            }
        }
        Ok(())
    }
}
