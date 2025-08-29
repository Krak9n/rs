use walkdir::WalkDir;
use std::{ env, fs, process::Command, time::SystemTime, fs::read_dir, error::Error, io };
use colored::Colorize;
use sysinfo::System;
use terminal_size::{Width, Height, terminal_size};
use filetime::FileTime;

mod args;

enum smt{
}

fn main() {
  // takes current path as the
  // one to list
  args::asserting();
  // let input = Args::parse();
  let access = RS::new();
  RS::make_better(access.files);
}

fn colors() {
}

struct RS {
  files: String,
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

    let mut sys = System::new_all();
    sys.refresh_all();
    
    let metadata = fs::metadata("./").unwrap();
    
    Self {
      files: "./".to_string(),
      width: w,
      reverse: false,
      filesize: metadata.len().to_string(),
      date: metadata.created(),
      os: System::name(),
    }
  }

  pub fn make_better(path: String) {
    println!("--{}", 
      format!("{:?} {:?}", Self::printing(path.clone()), Self::creation_date(path.clone())));
  }
 
  fn printing(path: String) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(path)? {
      let path = entry?.path();
      println!("{}", path.display());
    }
    Ok(())
  }

  fn creation_date(path: String) -> std::io::Result<()> {
    let pop = RS::new();

    for entry in fs::read_dir(&path)? {
      let pathy = entry?.path();
      let path_str = pathy.to_str().unwrap();

      let mut value = Command::new("sh")
        .arg("-c")
        .arg("stat -c '%w' ")
        .arg(pathy)
        .output()
        .expect("failed to execute process");
      
      println!("");
    }
    Ok(())
  }
}
