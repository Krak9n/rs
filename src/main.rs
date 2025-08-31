extern crate chrono;

use chrono::prelude::*;
use std::{ 
  fs, path::PathBuf, os::linux::fs::MetadataExt
};
use colored::Colorize;
use eventheader::*;

enum smt{
}

fn main() {
  let access = RS::new();

  RS::make_better(access.files);
}

struct RS {
  files: String,
  reverse: bool,
  filesize: String,
  date: String,
}

impl RS {
  pub fn new() -> Self {
    // doesnt work as intended
    let metadata = fs::metadata("./").unwrap();

    Self {
      // figure out tomorrow
      // how to make it understand
      // if -d was passed
      files: "./".to_string(),
      reverse: false,
      filesize: "".to_string(),
      date: "".to_string(),
    }
  }

  // make the colors function apper
  // during foor loop, and/or if it equals 
  // to dir make the color fucking red 
  // and file to be idk green
  fn filesize(path: &str) -> Result<u64, Box <dyn std::error::Error>> {
    let x = fs::metadata(path)?.st_size();   
    //println!("{}", x);
    Ok(x)
  }
  
  fn coloring(color: &str, text: &str, ind: &str) {
    print!("{} {}", text.color(color), ind);
  }

  // small terminal width
  pub fn make_better(path: String) {
    let _ = Self::printing(path.clone());
    /*print!("{}", 
      format!("{:?} {:?}", Self::filesize(path.clone(), s), Self::printing(path.clone(), fuukc))
      );*/
    //println!();
  }
 
  fn printing(path: String) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(&path)? {
      let path2 = entry?.path();
      let path_str = path2.to_str().unwrap();

      let pb = PathBuf::from(path2.clone());
      let is_file = path2.is_file();
     
      if is_file == true { 
        let _ = Self::coloring("red", &path2.display().to_string(), " --> "/*"--is file"*/);
      } 

      if is_file == false {
        let _ = Self::coloring("yellow", &path2.display().to_string(), " --> "/*"--is dir"*/);
      }
      match Self::filesize(&path_str) {
        Ok(x) => print!("{} bytes", x),
        Err(e) => print!("Error: {}", e),
      }

      let _ = Self::creation_date(path_str);
      // here passing date
 
    }
    Ok(())
  }

  fn creation_date(path: &str) -> std::io::Result<()> {
    let metadata = fs::metadata(&path)?;
    let created = metadata.created()?;

    let time = time_from_systemtime!(created);
    let naive = NaiveDateTime::from_timestamp(time, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

    println!(" --> {}", newdate);
    Ok(())
  }
}
