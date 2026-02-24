// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use std::path::PathBuf;
use std::convert::identity;

use qrate::{ QBank, QBDB, SQLiteDB, Excel };
use rfd::FileDialog;
use iced::Task;

use crate::control_tower::Message;

/// Represents the result of an attempt to load a `QBank`.
///
/// This enum encapsulates either a successfully loaded `QBank` instance
/// or a specific error indicating why the loading failed.
#[derive(Debug, Clone)]
pub enum ResultLoadFile
{
    /// Indicates successful loading of a `QBank`.
    Success(QBank),
    
    /// The specified file was not found.
    FileNotFound,

    /// Failed to open the SQLite database file.
    FailedToOpenSQLite,

    /// Failed to read the QBank data from the SQLite database.
    FailedToReadSQLite,

    /// Failed to open the Excel file.
    FailedToOpenExcel,

    /// Failed to read the QBank data from the Excel file.
    FailedToReadExcel,

    /// The Excel file does not have the required .qb.xlsx extension.
    InvalidExcelExtension,

    /// The file extension is not supported.
    UnsupportedExtension,
}

/// Provides utility functions for file-related operations in the application,
/// specifically for picking and loading `QBank` files.
#[derive(Debug, Clone)]
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
    /// successful, or one of the error variants if it fails.
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
    ///         _ => {
    ///             println!("Error loading QBank.");
    ///         },
    ///     }
    ///
    ///     let result_invalid_ext = LoadFile::load_qbank_from_path(invalid_extension_path).await;
    ///     if matches!(result_invalid_ext, ResultLoadFile::UnsupportedExtension) {
    ///         println!("Expected unsupported extension error.");
    ///     }
    /// }
    /// ```
    pub async fn load_qbank_from_path(path: PathBuf) -> ResultLoadFile
    {
        if !path.exists()
            { return ResultLoadFile::FileNotFound; }

        let path_str = path.to_string_lossy().into_owned(); // Convert PathBuf to String for QBDB::open
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        match extension
        {
            "qbdb" => {
                match SQLiteDB::open(path_str) { // Use QBDB::open for SQLiteDB
                    Some(db) => {
                        match db.read_qbank() { // Then read_qbank
                            Some(qbank) => ResultLoadFile::Success(qbank),
                            None => ResultLoadFile::FailedToReadSQLite,
                        }
                    },
                    None => ResultLoadFile::FailedToOpenSQLite,
                }
            },
            "xlsx" => {
                if path_str.contains(".qb.xlsx") { // Still check for .qb.xlsx as per original logic
                    match Excel::open(path_str) { // Use QBDB::open for Excel
                        Some(excel) => {
                            match excel.read_qbank() { // Then read_qbank
                                Some(qbank) => ResultLoadFile::Success(qbank),
                                None => ResultLoadFile::FailedToReadExcel,
                            }
                        },
                        None => ResultLoadFile::FailedToOpenExcel,
                    }
                }
                else
                {
                    ResultLoadFile::InvalidExcelExtension
                }
            },
            _ => ResultLoadFile::UnsupportedExtension,
        }
    }

    // pub fn perform_pick_qbank_task() -> Task<Message>
    /// Creates a [Task] to perform the asynchronous operation of picking a question bank file.
    ///
    /// This function encapsulates the `Task::perform` call, which spawns an asynchronous
    /// operation to open a file dialog and then wraps the result in a `Message::FileSelected`.
    ///
    /// # Output
    /// A [Task] that, when run, will eventually produce a `Message::FileSelected`.
    ///
    /// # Examples
    /// ```no_run
    /// use iced::Task;
    /// use crate::load_file::LoadFile;
    /// use crate::control_tower::Message; // Assuming Message is public
    ///
    /// // In an `iced` update function:
    /// // let task: Task<Message> = LoadFile::perform_pick_qbank_task();
    /// // return task;
    /// ```
    #[inline]
    pub fn perform_pick_qbank_task() -> Task<Message>
    {
        Task::perform(async { Message::FileSelected(LoadFile::pick_question_bank().await.unwrap_or_default()) }, identity)
    }

    // pub fn perform_load_qbank_task(path: PathBuf) -> Task<Message>
    /// Creates a [Task] to perform the asynchronous operation of loading a `QBank` from a specified path.
    ///
    /// This function encapsulates the `Task::perform` call, which spawns an asynchronous
    /// operation to load the QBank and then wraps the result in a `Message::QBankLoaded`.
    ///
    /// # Arguments
    /// * `path` - The `PathBuf` of the file to load the QBank from.
    ///
    /// # Output
    /// A [Task] that, when run, will eventually produce a `Message::QBankLoaded`.
    ///
    /// # Examples
    /// ```no_run
    /// use iced::Task;
    /// use crate::load_file::LoadFile;
    /// use crate::control_tower::Message; // Assuming Message is public
    /// use std::path::PathBuf;
    ///
    /// // In an `iced` update function:
    /// // let path_to_qbank = PathBuf::from("path/to/my_qbank.qbdb");
    /// // let task: Task<Message> = LoadFile::perform_load_qbank_task(path_to_qbank);
    /// // return task;
    /// ```
    #[inline]
    pub fn perform_load_qbank_task(path: PathBuf) -> Task<Message>
    {
        Task::perform(LoadFile::load_qbank_from_path(path), Message::QBankLoaded)
    }
}

