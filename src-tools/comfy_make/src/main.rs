extern crate util_kauk_rs;

use util_kauk_rs::cmd::native;
use std::env;

fn main() {
  let version: u8 = 5;  // Version that gets printed into the PBO filename

  let worlds = vec![
    "VR",
    "utes",
    "Sara",
    "SaraLite",
    "Sara_dbe1",
    "Chernarus",
    "Chernarus_Summer",
    "Porto",
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

  let cwd: String = get_cwd();

  for map in worlds {
    compile(cwd.as_ref(), version, map);
  }
}

fn get_cwd() -> String {
  let pathbuf = env::current_dir().unwrap();
  let path: &str = pathbuf.to_str().unwrap();
  format!("{}\\", path.to_string())
}

fn compile(path: &str,version: u8, map: &str) {
  let dirpath: String = format!("{}FP_ComfyDowntime.VR", path);
  let newpath: String = format!("{}FP_ComfyCompetition_{}.{}.pbo", path, version, map);
  native::run("PBOConsole", &["-pack", dirpath.as_ref(), newpath.as_ref()]);
}
