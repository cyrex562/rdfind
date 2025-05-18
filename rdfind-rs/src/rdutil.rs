// Ported from orig_src/Rdutil.cc on 2025-05-05
// Copyright 2006-2017 Paul Dreik (earlier Paul Sundvall)
// See LICENSE for further details.

use crate::fileinfo::{DupType, FileInfo, ReadToBufferMode};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

pub struct Rdutil<'a> {
    pub list: &'a mut Vec<FileInfo>,
}

impl<'a> Rdutil<'a> {
    pub fn new(list: &'a mut Vec<FileInfo>) -> Self {
        Rdutil { list }
    }

    pub fn print_to_file(&self, filename: &str) -> io::Result<()> {
        let mut f = File::create(filename)?;
        writeln!(f, "# Automatically generated")?;
        writeln!(f, "# duptype id depth size device inode priority name")?;
        for fi in self.list.iter() {
            writeln!(
                f,
                "{} {} {} {} {} {} {} {}",
                fi.get_duptype_string(),
                fi.identity,
                fi.depth,
                fi.stat_size,
                fi.stat_dev,
                fi.stat_ino,
                fi.cmdline_index,
                fi.filename.display()
            )?;
        }
        writeln!(f, "# end of file")?;
        Ok(())
    }

    pub fn mark_items(&mut self) {
        let mut fileno = 1;
        for file in self.list.iter_mut() {
            file.identity = fileno;
            fileno += 1;
        }
    }

    pub fn sort_on_device_and_inode(&mut self) {
        self.list
            .sort_by(|a, b| (a.stat_dev, a.stat_ino).cmp(&(b.stat_dev, b.stat_ino)));
    }

    pub fn sort_on_depth_and_name(&mut self, index_of_first: usize) {
        let len = self.list.len();
        if index_of_first < len {
            self.list[index_of_first..]
                .sort_by(|a, b| (a.depth, &a.filename).cmp(&(b.depth, &b.filename)));
        }
    }

    pub fn remove_identical_inodes(&mut self) -> usize {
        self.list
            .sort_by(|a, b| (a.stat_dev, a.stat_ino).cmp(&(b.stat_dev, b.stat_ino)));
        let mut removed = 0;
        let mut i = 0;
        while i < self.list.len() {
            let (dev, ino) = (self.list[i].stat_dev, self.list[i].stat_ino);
            let mut j = i + 1;
            while j < self.list.len()
                && self.list[j].stat_dev == dev
                && self.list[j].stat_ino == ino
            {
                j += 1;
            }
            if j - i > 1 {
                // Keep the one with the lowest cmdline_index, depth, identity
                let best = (i..j)
                    .min_by_key(|&k| {
                        (
                            self.list[k].cmdline_index,
                            self.list[k].depth,
                            self.list[k].identity,
                        )
                    })
                    .unwrap();
                for k in i..j {
                    self.list[k].delete_flag = k != best;
                }
            }
            i = j;
        }
        removed += self.cleanup();
        removed
    }

    pub fn remove_unique_sizes(&mut self) -> usize {
        self.list.sort_by(|a, b| a.stat_size.cmp(&b.stat_size));
        let mut removed = 0;
        let mut i = 0;
        while i < self.list.len() {
            let size = self.list[i].stat_size;
            let mut j = i + 1;
            while j < self.list.len() && self.list[j].stat_size == size {
                j += 1;
            }
            if j - i == 1 {
                self.list[i].delete_flag = true;
            } else {
                for k in i..j {
                    self.list[k].delete_flag = false;
                }
            }
            i = j;
        }
        removed += self.cleanup();
        removed
    }

    pub fn cleanup(&mut self) -> usize {
        let before = self.list.len();
        self.list.retain(|f| !f.delete_flag);
        before - self.list.len()
    }

    pub fn fill_with_bytes(
        &mut self,
        type_: ReadToBufferMode,
        lasttype: ReadToBufferMode,
        nsecsleep: u64,
        buffersize: usize,
    ) {
        self.sort_on_device_and_inode();
        let duration = Duration::from_nanos(nsecsleep);
        let mut buffer = vec![0u8; buffersize];
        for elem in self.list.iter_mut() {
            let _ = elem.fill_with_bytes(type_, lasttype, &mut buffer);
            if nsecsleep > 0 {
                thread::sleep(duration);
            }
        }
    }
}
