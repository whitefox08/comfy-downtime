#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_attributes)]

extern crate regex;
extern crate time;
extern crate rustc_serialize;

mod util;

use util::command;
use util::fs;
use std::env;
use std::path::Path;

fn main() {
  let worlds = vec![
    "VR",
    "utes",
    "Sara",
    "SaraLite",
    "Sara_dbe1",
    "Chernarus",
    "Chernarus_Summer",
    "Porto",
    "Intro",
    "IsolaDiCapraia",
    "Mountains_ACR",
    "Takistan",
    "Zargabad",
    "Woodland_ACR",
    "Bootcamp_ACR",
    "Desert_E",
    "ProvingGrounds_PMC",
    "Shapur_BAF",
    "fata",
    "Stratis",
    "anim_helvantis_v2",
    "Altis",
    "pja310"
  ];

  let cwd: String = format!("{}\\", get_cwd());

  prepare_comfyDowntime(cwd.as_ref());

  // Mission Generation for all Maps
  for map in worlds {
    println!("Generating missions for Map: {}", map);
    compile_comfyDowntime(cwd.as_ref(), map);
    compile_comfyCompetition(cwd.as_ref(), map);
  }
}

/*
  This function prepares the ComfyDowntime mission to be packed.
  It copies the main ComfyDowntime folder into 3 seperate ones. 1 for each major faction in the Armaverse.
  BLUFOR, OPFOR and INDFOR.
  
  Along with this, it edits the mission.sqm and description.ext files and Replaces the units, sides and mission name to fit the currently used faction.
*/
fn prepare_comfyDowntime(path: &str) {
  let mission_downtime = "FP_ComfyDowntime";
  let dirpath: String = format!("{}{}.VR", path, mission_downtime);
  let mission_str: &str = "COMFY_USE_B = false; COMFY_USE_O = false; COMFY_USE_I = false;";
  
  // Create Folders for ComfyDowntime Faction Variations
  command::run("robocopy", &[dirpath.as_ref(), format!("{}{}_Blufor", path, mission_downtime).as_ref(), "/E"]);
  command::run("robocopy", &[dirpath.as_ref(), format!("{}{}_Opfor", path, mission_downtime).as_ref(), "/E"]);
  command::run("robocopy", &[dirpath.as_ref(), format!("{}{}_Indfor", path, mission_downtime).as_ref(), "/E"]);
  
  /***********************
  # Mission.sqm Editing
  ************************/
  let mission_path: String = format!("{}.VR\\mission.sqm", mission_downtime);
  let mission_path_slice: &str = mission_path.as_ref();
  let mission_path_struct = Path::new(mission_path_slice);
  let mission_file: String = fs::read(&mission_path_struct);
  
  let mut mission_blufor: String = mission_file.replace(mission_str, "COMFY_USE_B = true; COMFY_USE_O = false; COMFY_USE_I = false;");
  let mut mission_opfor: String = mission_file.replace(mission_str, "COMFY_USE_B = false; COMFY_USE_O = true; COMFY_USE_I = false;");
  let mut mission_indfor: String = mission_file.replace(mission_str, "COMFY_USE_B = false; COMFY_USE_O = false; COMFY_USE_I = true;");
  
  // Now replace the Units in the mission to fit the Faction. We don't require to replace blufor units, since blufor classes are already default
  mission_opfor = mission_opfor.replace("b_soldier_unarmed_f", "o_soldier_unarmed_f");
  mission_indfor = mission_indfor.replace("b_soldier_unarmed_f", "i_soldier_unarmed_f");
  
  // Adjust the Actual Side of the unit.
  mission_opfor = mission_opfor.replace("side=\"WEST\";", "side=\"EAST\";");
  mission_indfor = mission_indfor.replace("side=\"WEST\";", "side=\"GUER\";");
  
  mission_blufor = mission_blufor.replace("<!mission_name!>", "FP Comfy Downtime 3.0 (BLUFOR)");
  mission_opfor = mission_opfor.replace("<!mission_name!>", "FP Comfy Downtime 3.0 (OPFOR)");
  mission_indfor = mission_indfor.replace("<!mission_name!>", "FP Comfy Downtime 3.0 (INDFOR)");
  
  fs::write(format!("{}_Blufor.\\mission.sqm", mission_downtime).as_ref(), mission_blufor.as_ref());
  fs::write(format!("{}_Opfor.\\mission.sqm", mission_downtime).as_ref(), mission_opfor.as_ref());
  fs::write(format!("{}_Indfor.\\mission.sqm", mission_downtime).as_ref(), mission_indfor.as_ref());
  
  /***********************
  # Description.ext Editing
  ************************/
  let description_path: String = format!("{}.VR\\description.ext", mission_downtime);
  let description_path_slice: &str = description_path.as_ref();
  let description_path_struct = Path::new(description_path_slice);
  let description_file: String = fs::read(&description_path_struct);
  
  let description_blufor: String = description_file.replace("<!mission_name!>", "FP Comfy Downtime 3.0 (BLUFOR)");
  let description_opfor: String = description_file.replace("<!mission_name!>", "FP Comfy Downtime 3.0 (OPFOR)");
  let description_indfor: String = description_file.replace("<!mission_name!>", "FP Comfy Downtime 3.0 (INDFOR)");
  
  fs::write(format!("{}_Blufor.\\description.ext", mission_downtime).as_ref(), description_blufor.as_ref());
  fs::write(format!("{}_Opfor.\\description.ext", mission_downtime).as_ref(), description_opfor.as_ref());
  fs::write(format!("{}_Indfor.\\description.ext", mission_downtime).as_ref(), description_indfor.as_ref());
}

/*
  This function Executes the PBOConsole Command and builds the Actual PBO Files for the
  ComfyDowntime Mission.
  
  It creates 3 PBO Files. One for Each Major Faction in Arma. BLUFOR; OPFOR and INFDOR/GUER
*/
fn compile_comfyDowntime(path: &str, map: &str) {
  let dirpath_blufor: String = format!("{}FP_ComfyDowntime_Blufor", path);
  let newpath_blufor: String = format!("{}FP_ComfyDowntime_Blufor.{}.pbo", path, map);
  
  let dirpath_opfor: String = format!("{}FP_ComfyDowntime_Opfor", path);
  let newpath_opfor: String = format!("{}FP_ComfyDowntime_Opfor.{}.pbo", path, map);
  
  let dirpath_indfor: String = format!("{}FP_ComfyDowntime_Indfor", path);
  let newpath_indfor: String = format!("{}FP_ComfyDowntime_Indfor.{}.pbo", path, map);

  command::run("PBOConsole", &["-pack", dirpath_blufor.as_ref(), newpath_blufor.as_ref()]);
  command::run("PBOConsole", &["-pack", dirpath_opfor.as_ref(), newpath_opfor.as_ref()]);
  command::run("PBOConsole", &["-pack", dirpath_indfor.as_ref(), newpath_indfor.as_ref()]);
}

/*
  This function Executes the PBOConsole Command and builds the Actual BPO Files for the ComfyCompetition Mission.
*/
fn compile_comfyCompetition(path: &str, map: &str) {
  let dirpath: String = format!("{}FP_ComfyCompetition.VR", path);
  let newpath: String = format!("{}FP_ComfyCompetition.{}.pbo", path, map);
  command::run("PBOConsole", &["-pack", dirpath.as_ref(), newpath.as_ref()]);
}


/*
  This function returns the Current working directory as a String.
  If combined with another path, an \\ or / might have to be added to
  the End of this String.
  
  for this, use the following format:
  format!("{}\\", get_cwd())
*/
fn get_cwd() -> String {
  let pathbuf = env::current_dir().unwrap();
  let path: &str = pathbuf.to_str().unwrap();
  path.to_string()
}