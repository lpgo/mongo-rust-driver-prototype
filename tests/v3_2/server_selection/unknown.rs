use super::framework::run_suite;

use std::fs;
use std::path::Path;

#[test]
fn server_selection_unknown_read() {
    let paths = fs::read_dir(&Path::new("tests/json/data/specs/source/server-selection/tests/server_selection/Unknown/read")).unwrap();

    for path in paths {
        let path2 = path.unwrap().path();
        let filename = path2.to_string_lossy();
        if filename.ends_with(".json") {
            println!("running suite {}", &filename);
            run_suite(&filename)
        }
    }
}

#[test]
fn server_selection_unknown_write() {
    let paths = fs::read_dir(&Path::new("tests/json/data/specs/source/server-selection/tests/server_selection/Unknown/write")).unwrap();

    for path in paths {
        let path2 = path.unwrap().path();
        let filename = path2.to_string_lossy();
        if filename.ends_with(".json") {
            println!("running suite {}", &filename);
            run_suite(&filename)
        }
    }
}
