
use std::fs;
use std::env;
use std::path::PathBuf;

fn main() {
  let mut build_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  build_dir.pop();
  build_dir.pop();
  build_dir.pop();

  let mut daemon_dir = build_dir.clone();
  daemon_dir.push("daemon");

  let mut resources_dir = build_dir;
  resources_dir.push("resources");

  let mut dispatch_toml = resources_dir.clone();
  dispatch_toml.push("dispatch.toml");

  if let Err(_) = fs::create_dir(daemon_dir) {}
  if let Err(_) = fs::create_dir(resources_dir) {}
  if let Err(_) = fs::File::open(&dispatch_toml) {
    fs::File::create(dispatch_toml).unwrap();
  }
}