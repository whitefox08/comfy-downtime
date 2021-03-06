#![allow(unused_must_use)]

extern crate util_kauk_rs;
extern crate walkdir;

use walkdir::WalkDir;
use std::fs;
use util_kauk_rs::cmd::native;
use std::env;
use std::path::Path;

fn main() {
  let version: u8 = 7;  // Version that gets printed into the PBO filename

  let worlds = vec![
    "VR",
    "Sara",
    "SaraLite",
    "Sara_dbe1",
    "Stratis",
    "Chernarus",
    "Chernarus_Summer",
    "Takistan",
    "utes",
    "Zargabad",
    "reshmaan",
    "MCN_Aliabad",
    "IsolaDiCapraia",
    "fata",
    "Caribou",
    "Altis",
    "anim_helvantis_v2",
    "Woodland_ACR",
    "ProvingGrounds_PMC"
  ];

  let cwd: String = get_cwd();
  prepare_adv_medcial();

  for map in worlds {
    compile(cwd.as_ref(), version, map);
    compile_adv_medical(cwd.as_ref(), version, map);
  }
}

/*
  Get Current working directory, where the binary was executed.
*/
fn get_cwd() -> String {
  let pathbuf = env::current_dir().unwrap();
  let path: &str = pathbuf.to_str().unwrap();
  format!("{}\\", path.to_string())
}

/*
  Function that Generates the new Filename with version info and map postfix
  and call to PBOConsole for the Basic Medical Mission.
*/
fn compile(path: &str,version: u8, map: &str) {
  let dirpath: String = format!("{}FP_ComfyDowntime.VR", path);
  let newpath: String = format!("{}FP_ComfyDowntime_{}.{}.pbo", path, version, map);
  native::run("PBOConsole", &["-pack", dirpath.as_ref(), newpath.as_ref()]);
}

/*
  Function that Generates the new Filename with version info and map postfix
  and call to PBOConsole for the Advanced Medical Mission.
*/
fn compile_adv_medical(path: &str, version: u8, map: &str) {
  let dirpath: String = format!("{}FP_ComfyDowntime_ADV.VR", path);
  let newpath: String = format!("{}FP_ComfyDowntime_ADV_{}.{}.pbo", path, version, map);
  native::run("PBOConsole", &["-pack", dirpath.as_ref(), newpath.as_ref()]);
}

/*
  Function that Prepares the ADV medical Folder to be built.
  This may be obsolete once ADV Medical gets incorporated into the main mission File.
*/
fn prepare_adv_medcial() {
  
  for entry in WalkDir::new("FP_ComfyDowntime.VR") {
    let entry = entry.unwrap();
    let entry_path: &Path = entry.path();
    let pathbuf = entry_path.to_path_buf();
    let newpath_string: String = pathbuf.to_str().unwrap().to_owned();
    let newpath_string_replaced: String = newpath_string.replace("FP_ComfyDowntime", "FP_ComfyDowntime_ADV");
    let newpath_string_replaced_slice: &str = newpath_string_replaced.as_ref();
    let newpath = Path::new(newpath_string_replaced_slice);
    if entry_path.is_dir() {fs::create_dir(&newpath);} else {fs::copy(&entry_path, &newpath);}
  }

  // Copy over the Advanced medical mission file
  fs::copy("mission_adv_medical.sqm", "FP_ComfyDowntime_ADV.VR/mission.sqm");
}