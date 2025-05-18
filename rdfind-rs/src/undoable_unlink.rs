// Ported from: orig_src/UndoableUnlink.cc
// Ported on 2025-05-05
// Copyright 2018 Paul Dreik
// See LICENSE for further details.
//
// This file was ported from the C++ UndoableUnlink class.

use crate::easy_random::EasyRandom;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
enum State {
    Uninitialized,
    FailedMoveToTemporary,
    MovedToTemporary,
    FailedUndo,
    Undone,
    FailedUnlink,
    Unlinked,
}

pub struct UndoableUnlink {
    state: State,
    filename: PathBuf,
    tempfilename: PathBuf,
}

impl UndoableUnlink {
    /// Moves the file to a random temporary name in the same directory.
    pub fn new(filename: &str) -> Self {
        let filename = PathBuf::from(filename);
        let mut tempfilename;
        let easy_random = EasyRandom::new();
        let rand_name = easy_random.make_random_file_string(12);
        if let Some(parent) = filename.parent() {
            tempfilename = parent.join(rand_name);
        } else {
            tempfilename = PathBuf::from(rand_name);
        }
        let state = match fs::rename(&filename, &tempfilename) {
            Ok(_) => State::MovedToTemporary,
            Err(e) => {
                eprintln!("Failed moving {:?} to a temporary file: {}", filename, e);
                State::FailedMoveToTemporary
            }
        };
        UndoableUnlink {
            state,
            filename,
            tempfilename,
        }
    }

    /// Checks if file is moved and ready for undo or unlink
    pub fn file_is_moved(&self) -> bool {
        self.state == State::MovedToTemporary
    }

    /// Moves the file back from the random name into the original filename
    pub fn undo(&mut self) -> io::Result<()> {
        if self.state != State::MovedToTemporary {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "api misuse - calling undo() now is a programming error",
            ));
        }
        match fs::rename(&self.tempfilename, &self.filename) {
            Ok(_) => {
                self.state = State::Undone;
                Ok(())
            }
            Err(e) => {
                self.state = State::FailedUndo;
                eprintln!(
                    "Failed moving file from temporary back to {:?}: {}",
                    self.filename, e
                );
                Err(e)
            }
        }
    }

    /// Removes the moved file
    pub fn unlink(&mut self) -> io::Result<()> {
        if self.state != State::MovedToTemporary {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "api misuse - calling unlink() now is a programming error",
            ));
        }
        match fs::remove_file(&self.tempfilename) {
            Ok(_) => {
                self.state = State::Unlinked;
                Ok(())
            }
            Err(e) => {
                self.state = State::FailedUnlink;
                eprintln!(
                    "Failed unlinking temporary file made from {:?}: {}",
                    self.filename, e
                );
                Err(e)
            }
        }
    }
}

impl Drop for UndoableUnlink {
    fn drop(&mut self) {
        if self.state == State::MovedToTemporary {
            let _ = self.undo();
        }
    }
}
