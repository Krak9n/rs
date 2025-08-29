use std::{env, fs};
use walkdir::WalkDir;
use std::fs::ReadDir;
use std::time::SystemTime;
use std::io;
use std::error::Error;
use clap::Parser;
use colored::Colorize;
use sysinfo::System;
use terminal_size::{Width, Height, terminal_size};

#[derive(Parser)]
struct Args {
  r: String,
  i: String,
  // only if directory is given
  d: String,
}

enum files { 
  
}

fn main() {
  // takes current path as the
  // one to list
  
  let access = RS::new();
  RS::printing(access.files);
  RS::creation_date();  
}

fn colors() {
}

struct RS {
  files: ReadDir,
  width: u16,
  reverse: bool,
  filesize: String,
  date: io::Result<SystemTime>,
  os: Option<String>,
}

impl RS {
  pub fn new() -> Self {
    let size = terminal_size();
    let Some((Width(w), Height(h))) = size else { todo!()};

    let metadata = fs::metadata("./").unwrap();
    
    Self {
      files: fs::read_dir("./").unwrap(),
      width: w,
      reverse: false,
      filesize: metadata.len().to_string(),
      date: metadata.created(),
      os: System::name(),
    }
  }

  pub fn printing(path: ReadDir) {
    for path in path {
      println!("{}", 
        format!("{} ", path.unwrap().path().display()));
    }
  }

  pub fn creation_date() -> Result<(), Box<dyn std::error::Error>> {
    let pop = RS::new();
   
    let path: String = pop.files;
    for entry in fs::read_dir(path)? {
      if let Ok(time) = pop.date {
        println!("{time:?}");
      } else {
        println!("not supported on this platform or filesystem");
      }
    }
    Ok(())
  }
}
