use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

use censor::Censor;

pub(crate) fn sensor_profanity(text: &String) -> String {
    let file: BufReader<File> = BufReader::new(File::open("profanity_words/vi.txt").unwrap());
    let mut censor: Censor = Censor::Standard;

    for line in file.lines() {
        censor = censor + &line.unwrap();
    }

    return String::from(&censor.censor(text));
}

pub(crate) fn add_profanity(text: &str, language: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("profanity_words/{}.txt", language))
        .unwrap();

    if let Err(e) = writeln!(file, "{}", text) {
        eprintln!("Couldn't write to file: {}", e);
    }
}