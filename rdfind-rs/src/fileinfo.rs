// Ported from orig_src/Fileinfo.cc
// Copyright 2006-2017 Paul Dreik (earlier Paul Sundvall)
// See LICENSE for further details.

use std::env;
use std::fs::{self, hard_link, File};
use std::io::{self, Read, Seek, SeekFrom};

use std::os::unix::fs::{MetadataExt, symlink};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadToBufferMode {
    NotDefined = -1,
    ReadFirstBytes = 0,
    ReadLastBytes = 1,
    CreateMd5Checksum = 2,
    CreateSha1Checksum,
    CreateSha256Checksum,
    CreateSha512Checksum,
    CreateXxh128Checksum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DupType {
    Unknown,
    FirstOccurrence,
    WithinSameTree,
    OutsideTree,
}

pub struct FileInfo {
    pub filename: PathBuf,
    pub delete_flag: bool,
    pub duptype: DupType,
    pub cmdline_index: i32,
    pub depth: i32,
    pub identity: i64,
    pub somebytes: [u8; 64],
    pub stat_size: i64,
    pub stat_ino: u64,
    pub stat_dev: u64,
    pub is_file: bool,
    pub is_directory: bool,
}

impl FileInfo {
    pub fn new(filename: PathBuf, cmdline_index: i32, depth: i32) -> Self {
        FileInfo {
            filename,
            delete_flag: false,
            duptype: DupType::Unknown,
            cmdline_index,
            depth,
            identity: 0,
            somebytes: [0; 64],
            stat_size: 99999,
            stat_ino: 99999,
            stat_dev: 99999,
            is_file: false,
            is_directory: false,
        }
    }

    pub fn read_file_info(&mut self) -> bool {
        match fs::metadata(&self.filename) {
            Ok(meta) => {
                self.stat_size = meta.size() as i64;
                self.stat_ino = meta.ino();
                self.stat_dev = meta.dev();
                self.is_file = meta.is_file();
                self.is_directory = meta.is_dir();
                true
            }
            Err(e) => {
                self.stat_size = 0;
                self.stat_ino = 0;
                self.stat_dev = 0;
                self.is_file = false;
                self.is_directory = false;
                eprintln!("read_file_info: Error reading {:?}: {}", self.filename, e);
                false
            }
        }
    }

    pub fn fill_with_bytes(
        &mut self,
        filltype: ReadToBufferMode,
        lasttype: ReadToBufferMode,
        buffer: &mut [u8],
    ) -> io::Result<()> {
        // If file is short, first bytes might be ALL bytes!
        if lasttype != ReadToBufferMode::NotDefined {
            if self.stat_size <= self.somebytes.len() as i64 {
                // pointless to read - all bytes in the file are in the field
                // somebytes, or checksum is calculated!
                return Ok(());
            }
        }
        self.somebytes.fill(0);
        let mut file = File::open(&self.filename)?;
        match filltype {
            ReadToBufferMode::ReadFirstBytes => {
                file.read_exact(&mut self.somebytes)?;
            }
            ReadToBufferMode::ReadLastBytes => {
                let len = self.somebytes.len() as u64;
                let filesize = file.metadata()?.len();
                if filesize >= len {
                    file.seek(SeekFrom::End(-(len as i64)))?;
                } else {
                    file.seek(SeekFrom::Start(0))?;
                }
                file.read_exact(&mut self.somebytes)?;
            }
            // For checksum modes, you would call out to a checksum module here.
            // Placeholder: just fill with zeros.
            _ => {
                // TODO: integrate with checksum.rs
            }
        }
        Ok(())
    }

    pub fn delete_file(&self) -> io::Result<()> {
        match fs::remove_file(&self.filename) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed deleting file {:?}: {}", self.filename, e);
                Err(e)
            }
        }
    }

    pub fn make_symlink(&self, target: &FileInfo) -> io::Result<()> {
        let mut target_path = target.filename.clone();
        make_absolute(&mut target_path)?;
        simplify_path(&mut target_path);
        match symlink(&target_path, &self.filename) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!(
                    "Failed to make symlink {:?} to {:?}: {}",
                    self.filename, target.filename, e
                );
                Err(e)
            }
        }
    }

    pub fn make_hardlink(&self, target: &FileInfo) -> io::Result<()> {
        match hard_link(&target.filename, &self.filename) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!(
                    "Failed to make hardlink {:?} to {:?}: {}",
                    self.filename, target.filename, e
                );
                Err(e)
            }
        }
    }

    pub fn get_duptype_string(&self) -> &'static str {
        match self.duptype {
            DupType::Unknown => "DUPTYPE_UNKNOWN",
            DupType::FirstOccurrence => "DUPTYPE_FIRST_OCCURRENCE",
            DupType::WithinSameTree => "DUPTYPE_WITHIN_SAME_TREE",
            DupType::OutsideTree => "DUPTYPE_OUTSIDE_TREE",
        }
    }
}

fn simplify_path(path: &mut PathBuf) {
    let mut s = path.to_string_lossy().to_string();
    while let Some(pos) = s.find("/./") {
        s.replace_range(pos..pos + 3, "/");
    }
    while let Some(pos) = s.find("//") {
        s.replace_range(pos..pos + 2, "/");
    }
    *path = PathBuf::from(s);
}

fn make_absolute(path: &mut PathBuf) -> io::Result<()> {
    if path.is_absolute() {
        return Ok(());
    }
    let cwd = env::current_dir()?;
    *path = cwd.join(&*path);
    Ok(())
}
