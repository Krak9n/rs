use walkdir::WalkDir;
use std::{ 
  env, fs, io, 
  process::Command, time::SystemTime, fs::read_dir, 
  error::Error, path::PathBuf };
use colored::Colorize;
use sysinfo::System;
use terminal_size::{Width, Height, terminal_size};
use filetime::FileTime;
use clap::{ Arg, Parser, parser::ValueSource };

#[derive(Parser)]
struct Args {
  #[clap(short, default_value_t = true)]
  r: bool,
  #[clap(short, default_value_t = true)]
  i: bool,
  #[clap(short, default_value_t = true)]
  d: bool,
}

enum smt{
}

fn main() {
  let access = RS::new();

  let k: bool = false;
  let a: bool = true;

  if access.width <= 39 { 
    RS::make_better(access.files, k, access.filesize);
  } else {
    RS::make_it_suck(access.files, a, access.filesize);
  }
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
   
    let args1 = Args::parse();
    // doesnt work as intended
    let metadata = fs::metadata("./").unwrap();
    let args: Vec<String> = env::args()
      .collect();

    Self {
      files: {
        if args1.d == false {
          let path = args[2].to_string();
          path
        } else {
          let path = "./".to_string();
          path
        }
      },
      width: w,
      reverse: false,
      filesize: "".to_string(),
      date: metadata.created(),
      os: System::name(),
    }
  }

  // make the colors function apper
  // during foor loop, and/or if it equals 
  // to dir make the color fucking red 
  // and file to be idk green
  fn filesize(path: String, size: String) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(path)? {
      let path2 = entry?.path();
      let path_str = path2.to_str().unwrap();
 
      let metada = fs::metadata(path_str).unwrap();
      print!("{} ", metada.len());
    }
    Ok(())
  }
  
  // large width
  pub fn make_it_suck(path: String, fuck: bool, s: String) {
    Self::printing(path.clone(), fuck);
    /*print!("{}", 
      format!("{:?} {:?}", Self::filesize(path.clone(), s), Self::printing(path.clone(), fuck))
      );*/ 
    //Self::creation_date(path.clone());
    println!();
  }

  fn coloring(color: &str, text: &str, ind: &str, k: bool) {
    if k == true { 
      print!("{} {}", text.color(color), ind);
    } else {
      println!("{} {}", text.color(color), ind);
    } 
  }

  // small terminal width
  pub fn make_better(path: String, fuukc: bool, s: String) {
    Self::printing(path.clone(), fuukc);
    /*print!("{}", 
      format!("{:?} {:?}", Self::filesize(path.clone(), s), Self::printing(path.clone(), fuukc))
      );*/
    //Self::creation_date(path.clone());
    //println!();
  }
 
  fn printing(path: String, of: bool) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(path)? {
      let path2 = entry?.path();
      let path_str = path2.to_str().unwrap();

      let pb = PathBuf::from(path2.clone());
      let is_file = path2.is_file();
     
      // finish alphabetical stuff later
      /*
      let mut vec = Vec::new();
      vec.push(path_str);
      vec.sort_by_key(|name| name.to_lowercase());
      println!("{vec:?}");
      */

      if is_file == true { 
        Self::coloring("red", &path2.display().to_string(), ""/*"--is file"*/, of);
      } 

      if is_file == false {
        Self::coloring("yellow", &path2.display().to_string(), ""/*"--is dir"*/, of);
      }
    }
    Ok(())
  }

  fn show_from_input() {

  }

  fn creation_date(path: String) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
      let path2 = entry?.path();
      let path_str = path2.to_str().unwrap();
      
      let metadata = fs::metadata(&path_str)?;
      let created = metadata.created()?;
      //println!("{} was created on {:?}", path_str, created);
    }

    Ok(())
  }
}
