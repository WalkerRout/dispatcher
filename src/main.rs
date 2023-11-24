
// no stdout/stderr, need to redirect eventually
#![windows_subsystem = "windows"]

use serde::Deserialize;

use execute::{
  command,
  Execute,
};

use async_executors::ThreadPool;

use livesplit_hotkey::{
  Hook,
  Hotkey,
  KeyCode,
  Modifiers,
};

use std::fs;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::sync::atomic::{
  Ordering,
  AtomicBool,
};
use std::error::Error;
use std::thread;
use std::process::Stdio;
use std::collections::HashMap;

#[cfg(target_family = "unix")]
use daemonize::Daemonize;

static TERMINATE: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
struct KeyScriptMap(HashMap<Hotkey, String>);

impl KeyScriptMap {
  fn from_config(config: Config) -> Result<KeyScriptMap, Box<dyn Error>> {
    let keys = config.commands
      .iter()
      .map(|c| c.into_hotkey())
      .collect::<Result<Vec<_>, _>>()?;

    let scripts = config.commands
      .into_iter()
      .map(|c| c.script)
      .collect::<Vec<_>>();

    let map = keys
      .into_iter()
      .zip(scripts.into_iter())
      .collect();

    Ok(KeyScriptMap(map))
  }
}

#[derive(Debug, Deserialize)]
struct Config {
  commands: Vec<Command>,
}

#[derive(Debug, Deserialize)]
struct Command {
  alt: Option<bool>, // option
  meta: Option<bool>, // win, super, cmd
  shift: Option<bool>,
  control: Option<bool>,
  hotkey: String,
  script: String,
}

impl Command {
  fn into_hotkey(&self) -> Result<Hotkey, Box<dyn Error>> {
    let key = Hotkey {
      key_code: self.key_code()?,
      modifiers: self.modifiers()
    };

    Ok(key)
  }

  fn modifiers(&self) -> Modifiers {
    let def = Modifiers::empty();

    let alt     = self.alt.map(|_| Modifiers::ALT).unwrap_or(def);
    let meta    = self.meta.map(|_| Modifiers::META).unwrap_or(def);
    let shift   = self.shift.map(|_| Modifiers::SHIFT).unwrap_or(def);
    let control = self.control.map(|_| Modifiers::CONTROL).unwrap_or(def);
  
    alt | meta | shift | control
  }

  fn key_code(&self) -> Result<KeyCode, Box<dyn Error>> {
    Ok(KeyCode::from_str(&self.hotkey).unwrap())
  }
}

fn exit_key() -> Hotkey {
  let key_code = KeyCode::KeyE;
  let modifiers = Modifiers::SHIFT | Modifiers::CONTROL | Modifiers::ALT;

  Hotkey {
    key_code,
    modifiers,
  }
}

fn toml_config<S: AsRef<str>>(dispatch_toml_path: S) -> Result<Config, Box<dyn Error>> {
  let dispatch_toml = fs::read_to_string(dispatch_toml_path.as_ref())?;
  let config: Config = toml::from_str(&dispatch_toml)?;
  Ok(config)
}

fn register_hotkeys(config: Config, pool: &Arc<ThreadPool>) -> Result<Hook, Box<dyn Error>> {
  let hook = Hook::new()?;

  if let Ok(map) = KeyScriptMap::from_config(config) {
    // register all user keys
    for (key, script) in map.0 {
      let pool = Arc::clone(pool);

      hook.register(key, move || {
        let script = script.clone();
        let fut = async move {
          println!("running: `{}`", &script);
          let mut command = command(script);
          command.stdout(Stdio::piped());
          if let Err(e) = command.execute_output() {
            eprintln!("failed with: {e}");
          }
        };

        pool.spawn_ok(fut);
      })?;
    }
  }

  // did user accidentally register exit keycode?
  if let Err(_) = hook.unregister(exit_key()) {}
  hook.register(exit_key(), || {
    TERMINATE.swap(true, Ordering::Relaxed);
  })?;

  Ok(hook)
}

fn construct_hook(pool: &Arc<ThreadPool>) -> Result<Hook, Box<dyn Error>> {
  let dispatch_toml_path = {
    let mut exe = env::current_exe()?;
    exe.pop();
    exe.push("resources");
    exe.push("dispatch.toml");

    exe
      .into_os_string()
      .into_string()
      .unwrap()
  };

  let config = toml_config(dispatch_toml_path)?;
  register_hotkeys(config, pool)
}

fn main_loop() {
  while !TERMINATE.load(Ordering::Relaxed) {
    thread::sleep(Duration::from_millis(2000));
  }
}

// hold _hook for drop check
fn event_loop() {
  let pool = Arc::new(ThreadPool::new().expect("failed to create ThreadPool"));
  let mut _hook = construct_hook(&pool).expect("failed to create Hook");

  while !TERMINATE.load(Ordering::Relaxed) { 
    thread::sleep(Duration::from_millis(5000)); // update every 5 seconds
    if let Ok(h) = construct_hook(&pool) {
      _hook = h;
    }
  }
}

fn loops() -> Result<(), Box<dyn Error>> {
  let mh = thread::spawn(move || main_loop());
  let eh = thread::spawn(move || event_loop());

  eh.join().expect("failed to join event thread");
  mh.join().expect("failed to join main thread");

  Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(target_family = "unix")]
  {
    use fs::File;

    let mut daemon_dir = env::current_exe()?;
    daemon_dir.pop();
    daemon_dir.push("daemon");

    let stdout = File::create(format!("{}/daemon.out", daemon_dir.display()))?;
    let stderr = File::create(format!("{}/daemon.err", daemon_dir.display()))?;

    let daemon = Daemonize::new()
      .working_directory(daemon_dir)
      .pid_file("daemon.pid")
      .stdout(stdout)
      .stderr(stderr)
      .privileged_action(|| "Executed before drop privileges\n");

    match daemon.start() {
      Ok(v) => {
        println!("{v}: Daemon successfully started, beginning loops...");
        loops()?;
      },
      Err(e) => println!("{e}"),
    }

    Ok(())
  }

  #[cfg(target_family = "windows")]
  {
    loops()
  }
}
