use config;
use simulation::models::location::Location;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use utils;

pub fn debug(string: &str) {
    if config::DEBUG {
        println!("{}", string);
    }
}

pub fn write_to_file(line: String, append: bool) {
    let file_name = utils::file_name();
    let path = Path::new(&file_name);

    let mut file = OpenOptions::new()
        .write(true)
        .append(append)
        .open(path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn create_file() {
    let file_name = utils::file_name();
    let path = Path::new(&file_name);
    let display = path.display();

    match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
}

pub fn file_name() -> String {
    let mut data_type = "test";

    if !config::TEST_DATA {
        data_type = "random";
    }

    let selection;
    unsafe {
        match config::SELECTION_ALGORITHM_X {
            config::SelectionAlgorithm::Tournament => {
                selection = "tour";
            }
            config::SelectionAlgorithm::Roulette => {
                selection = "roul";
            }
            config::SelectionAlgorithm::Random => {
                selection = "rand";
            }
        }
    }

    return format!(
        "target/{}_{}_loc{}_pop{}.csv",
        data_type,
        selection,
        config::LOCATION_SIZE,
        config::POP_SIZE
    );
}

pub fn test_data() -> Vec<Location> {
    let mut locations: Vec<Location> = Vec::new();
    locations.push(Location { 
        x: 567.0, 
        y: 471.0
    });
    locations.push(Location { 
        x: 286.0, 
        y: 571.0
    });
    locations.push(Location {
        x: 1269.0,
        y: 491.0,
    });
    locations.push(Location { 
        x: 501.0, 
        y: 410.0
    });
    locations.push(Location { 
        x: 880.0, 
        y: 120.0
    });
    locations.push(Location {
        x: 1221.0,
        y: 158.0,
    });
    locations.push(Location { 
        x: 474.0, 
        y: 46.0
    });
    locations.push(Location {
        x: 1008.0,
        y: 561.0,
    });
    locations.push(Location { 
        x: 728.0, 
        y: 632.0
    });
    locations.push(Location { 
        x: 952.0, 
        y: 658.0
    });
    locations.push(Location {
        x: 1022.0,
        y: 202.0,
    });
    locations.push(Location { 
        x: 341.0, 
        y: 637.0
    });
    locations.push(Location {
        x: 1160.0,
        y: 367.0,
    });
    locations.push(Location { 
        x: 753.0, 
        y: 138.0
    });
    locations.push(Location { 
        x: 17.0, 
        y: 505.0
    });
    locations.push(Location { 
        x: 723.0, 
        y: 334.0
    });
    locations.push(Location { 
        x: 302.0, 
        y: 320.0
    });
    locations.push(Location {
        x: 1252.0,
        y: 181.0,
    });
    locations.push(Location {
        x: 1237.0,
        y: 195.0,
    });
    locations.push(Location { 
        x: 562.0, 
        y: 719.0
    });
    locations.push(Location { 
        x: 313.0, 
        y: 144.0
    });
    locations.push(Location { 
        x: 149.0, 
        y: 715.0
    });
    locations.push(Location { 
        x: 601.0, 
        y: 23.0
    });
    locations.push(Location {
        x: 1033.0,
        y: 159.0,
    });
    locations.push(Location { 
        x: 420.0, 
        y: 43.0
    });
    locations.push(Location { 
        x: 64.0, 
        y: 263.0
    });
    locations.push(Location { 
        x: 661.0, 
        y: 108.0
    });
    locations.push(Location { 
        x: 706.0, 
        y: 157.0
    });
    locations.push(Location { 
        x: 719.0, 
        y: 452.0
    });
    locations.push(Location { 
        x: 704.0, 
        y: 467.0
    });
    locations.push(Location { 
        x: 639.0, 
        y: 146.0
    });
    locations.push(Location { 
        x: 1243.0, 
        y: 9.0
    });
    locations.push(Location { 
        x: 417.0, 
        y: 39.0
    });
    locations.push(Location {
        x: 1114.0,
        y: 476.0,
    });
    locations.push(Location {
        x: 1039.0,
        y: 220.0,
    });
    locations.push(Location { 
        x: 1146.0, 
        y: 7.0
    });
    locations.push(Location { 
        x: 9.0, 
        y: 397.0
    });
    locations.push(Location { 
        x: 649.0, 
        y: 465.0
    });
    locations.push(Location {
        x: 1164.0,
        y: 142.0,
    });
    locations.push(Location { 
        x: 803.0, 
        y: 520.0
    });
    locations.push(Location { 
        x: 575.0, 
        y: 311.0
    });
    locations.push(Location { 
        x: 423.0, 
        y: 553.0
    });
    locations.push(Location { 
        x: 362.0, 
        y: 611.0
    });
    locations.push(Location { 
        x: 177.0, 
        y: 357.0
    });
    locations.push(Location { 
        x: 975.0, 
        y: 614.0
    });
    locations.push(Location { 
        x: 765.0, 
        y: 2.0
    });
    locations.push(Location {
        x: 1078.0,
        y: 646.0,
    });
    locations.push(Location { 
        x: 17.0, 
        y: 571.0
    });
    locations.push(Location { 
        x: 852.0, 
        y: 478.0
    });
    locations.push(Location { 
        x: 377.0, 
        y: 556.0
    });
    locations.push(Location { 
        x: 326.0, 
        y: 462.0
    });
    locations.push(Location { 
        x: 839.0, 
        y: 451.0
    });
    locations.push(Location {
        x: 1165.0,
        y: 541.0,
    });
    locations.push(Location { 
        x: 916.0, 
        y: 584.0
    });
    locations.push(Location { 
        x: 0.0, 
        y: 450.0
    });
    locations.push(Location {
        x: 1218.0,
        y: 336.0,
    });
    locations.push(Location { 
        x: 153.0, 
        y: 644.0
    });
    locations.push(Location {
        x: 1072.0,
        y: 324.0,
    });
    locations.push(Location { 
        x: 293.0, 
        y: 182.0
    });
    locations.push(Location { 
        x: 739.0, 
        y: 547.0
    });
    locations.push(Location { 
        x: 777.0, 
        y: 284.0
    });
    locations.push(Location { 
        x: 841.0, 
        y: 553.0
    });
    locations.push(Location { 
        x: 163.0, 
        y: 349.0
    });
    locations.push(Location { 
        x: 249.0, 
        y: 538.0
    });
    locations.push(Location { 
        x: 635.0, 
        y: 545.0
    });
    locations.push(Location {
        x: 1123.0,
        y: 546.0,
    });
    locations.push(Location { 
        x: 15.0, 
        y: 682.0
    });
    locations.push(Location {
        x: 1277.0,
        y: 436.0,
    });
    locations.push(Location { 
        x: 530.0, 
        y: 381.0
    });
    locations.push(Location { 
        x: 1256.0, 
        y: 10.0
    });
    locations.push(Location { 
        x: 437.0, 
        y: 713.0
    });
    locations.push(Location {
        x: 1236.0,
        y: 213.0,
    });
    locations.push(Location { 
        x: 418.0, 
        y: 651.0
    });
    locations.push(Location { 
        x: 304.0, 
        y: 48.0
    });
    locations.push(Location {
        x: 1241.0,
        y: 257.0,
    });
    locations.push(Location { 
        x: 1268.0, 
        y: 63.0
    });
    locations.push(Location { 
        x: 457.0, 
        y: 381.0
    });
    locations.push(Location { 
        x: 578.0, 
        y: 165.0
    });
    locations.push(Location {
        x: 1242.0,
        y: 241.0,
    });
    locations.push(Location { 
        x: 818.0, 
        y: 140.0
    });
    locations.push(Location { 
        x: 833.0, 
        y: 664.0
    });
    locations.push(Location {
        x: 1115.0,
        y: 583.0,
    });
    locations.push(Location { 
        x: 1031.0, 
        y: 89.0
    });
    locations.push(Location { 
        x: 567.0, 
        y: 308.0
    });
    locations.push(Location { 
        x: 395.0, 
        y: 159.0
    });
    locations.push(Location { 
        x: 355.0, 
        y: 529.0
    });
    locations.push(Location {
        x: 1220.0,
        y: 532.0,
    });
    locations.push(Location { 
        x: 152.0, 
        y: 501.0
    });
    locations.push(Location {
        x: 1044.0,
        y: 143.0,
    });
    locations.push(Location { 
        x: 826.0, 
        y: 257.0
    });
    locations.push(Location { 
        x: 623.0, 
        y: 445.0
    });
    locations.push(Location {
        x: 1060.0,
        y: 246.0,
    });
    locations.push(Location { 
        x: 466.0, 
        y: 28.0
    });
    locations.push(Location { 
        x: 592.0, 
        y: 278.0
    });
    locations.push(Location {
        x: 1255.0,
        y: 288.0,
    });
    locations.push(Location { 
        x: 559.0, 
        y: 672.0
    });
    locations.push(Location {
        x: 1145.0,
        y: 611.0,
    });
    locations.push(Location { 
        x: 360.0, 
        y: 290.0
    });
    locations.push(Location { 
        x: 601.0, 
        y: 584.0
    });
    locations.push(Location { 
        x: 364.0, 
        y: 7.0
    });

    assert_eq!(locations.len(), config::LOCATION_SIZE);

    locations
}
