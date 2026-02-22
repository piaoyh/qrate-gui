use qrate::{QBank, QBDB, SQLiteDB, Excel};
use rfd::FileDialog;
use std::path::PathBuf;

/// Represents the result of an attempt to load a `QBank`.
///
/// This enum encapsulates either a successfully loaded `QBank` instance
/// or an error message indicating why the loading failed.
#[derive(Clone)]
pub enum ResultLoadFile
{
    /// Indicates successful loading of a `QBank`.
    Success(QBank),
    /// Indicates that `QBank` loading failed, containing an error message.
    Error(String),
}

/// Provides utility functions for file-related operations in the application,
/// specifically for picking and loading `QBank` files.
pub struct LoadFile;

impl LoadFile
{
    // pub async fn pick_question_bank() -> Option<PathBuf>
    /// Asynchronously opens a file dialog for the user to pick a question bank file.
    ///
    /// This function is designed to be called within an `iced::Task`. It presents
    /// a native file dialog filtered for question bank file types (`.qbdb`, `.xlsx`).
    ///
    /// # Output
    /// An `Option<PathBuf>` representing the path to the selected file,
    /// or `None` if no file was selected.
    ///
    /// # Examples
    /// ```no_run
    /// // This is an async function that opens a GUI file dialog.
    /// // It cannot be directly tested with assert_eq! without mocking the GUI,
    /// // but here's how you would typically call it in an Iced application:
    /// async fn example_usage() {
    ///     use std::path::PathBuf;
    ///     use crate::load_file::LoadFile;
    ///
    ///     let selected_path: Option<PathBuf> = LoadFile::pick_question_bank().await;
    ///     match selected_path {
    ///         Some(path) => println!("File selected: {:?}", path),
    ///         None => println!("No file selected."),
    ///     }
    /// }
    /// ```
    pub async fn pick_question_bank() -> Option<PathBuf>
    {
        FileDialog::new()
            .add_filter("Question Bank", &["qbdb", "xlsx"])
            .set_directory(".")
            .pick_file()
    }

    // pub async fn load_qbank_from_path(path: PathBuf) -> ResultLoadFile
    /// Asynchronously loads a `QBank` from the given file path.
    ///
    /// This function reads the file at the specified path and attempts to parse
    /// it into a `QBank` object based on its extension.
    ///
    /// # Arguments
    /// * `path` - The `PathBuf` of the file to load.
    ///
    /// # Output
    /// A `ResultLoadFile` enum, which is `Success(QBank)` if loading is
    /// successful, or `Error(String)` if it fails.
    ///
    /// # Examples
    /// ```no_run
    /// // This is an async function that interacts with the file system.
    /// // It cannot be directly tested with assert_eq! without mocking the file system,
    /// // but here's how you would typically call it in an Iced application:
    /// async fn example_usage() {
    ///     use std::path::PathBuf;
    ///     use crate::load_file::{LoadFile, ResultLoadFile};
    ///     // For a real test, you would create dummy files.
    ///     // Assume "dummy.qbdb" is a valid qbdb file in the current directory
    ///     // and "invalid.txt" is an unsupported file.
    ///     let valid_qbdb_path = PathBuf::from("dummy.qbdb");
    ///     let invalid_extension_path = PathBuf::from("invalid.txt");
    ///
    ///     let result_valid_qbdb = LoadFile::load_qbank_from_path(valid_qbdb_path).await;
    ///     match result_valid_qbdb {
    ///         ResultLoadFile::Success(qbank) => {
    ///             println!("Successfully loaded QBank with {} questions.",
    ///                      qbank.get_questions().len());
    ///         },
    ///         ResultLoadFile::Error(e) => {
    ///             println!("Error loading QBank: {}", e);
    ///         },
    ///     }
    ///
    ///     let result_invalid_ext = LoadFile::load_qbank_from_path(invalid_extension_path).await;
    ///     if let ResultLoadFile::Error(e) = result_invalid_ext {
    ///         println!("Expected error for invalid file: {}", e);
    ///     }
    /// }
    /// ```
    pub async fn load_qbank_from_path(path: PathBuf) -> ResultLoadFile
    {
        if !path.exists()
            { return ResultLoadFile::Error("File does not exist.".to_string()); }

        let path_str = path.to_string_lossy().into_owned(); // Convert PathBuf to String for QBDB::open
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        match extension
        {
            "qbdb" => {
                match SQLiteDB::open(path_str) { // Use QBDB::open for SQLiteDB
                    Some(db) => {
                        match db.read_qbank() { // Then read_qbank
                            Some(qbank) => ResultLoadFile::Success(qbank),
                            None => ResultLoadFile::Error("Failed to read QBank from QBDB.".to_string()),
                        }
                    },
                    None => ResultLoadFile::Error("Failed to open QBDB file.".to_string()),
                }
            },
            "xlsx" => {
                if path_str.contains(".qb.xlsx") { // Still check for .qb.xlsx as per original logic
                    match Excel::open(path_str) { // Use QBDB::open for Excel
                        Some(excel) => {
                            match excel.read_qbank() { // Then read_qbank
                                Some(qbank) => ResultLoadFile::Success(qbank),
                                None => ResultLoadFile::Error("Failed to read QBank from Excel.".to_string()),
                            }
                        },
                        None => ResultLoadFile::Error("Failed to open Excel file.".to_string()),
                    }
                }
                else
                {
                    ResultLoadFile::Error("Not a valid *.qb.xlsx file. Expecting .qb.xlsx extension for Excel QBank.".to_string())
                }
            },
            _ => ResultLoadFile::Error(format!("Unsupported file extension: {}", extension)),
        }
    }
}
