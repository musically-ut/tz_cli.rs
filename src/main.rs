use std::env;
use std::path::PathBuf;
use std::io;
use std::cmp;
use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

extern crate chrono;
extern crate chrono_tz;

use chrono::Local;
use chrono_tz::Tz;

/// The function takes arguments passed to the program and an optional path to $HOME
/// and returns the location of the config file to read the timezones from.
pub fn get_tz_file<'a>(args: &'a Vec<String>, opt_home: &'a Option<PathBuf>) -> Result<PathBuf, String> {
    if args.len() > 1 {
        return Ok(PathBuf::from(&args[1]));
    } else {
        return match opt_home {
            &Some(ref home) => {
                let mut abs_path = home.clone();
                abs_path.push(PathBuf::from(".tz.rc"));
                Ok(abs_path)
            },
            &None => Err("No home directory!".to_string())
        }
    }
}

/// Reads the config file and returns a vector of parsed timezones or an io::error.
pub fn read_file(conf_file: &PathBuf) -> Result<Vec<(String, Tz)>, io::Error> {
    let mut tzs : Vec<(String, Tz)> = Vec::new();
    let f = try!(File::open(conf_file.to_str().unwrap()));
    let file_buffer = BufReader::new(&f);
    for tz_name_line in file_buffer.lines() {
        let tz_name = tz_name_line.unwrap();
        match tz_name.parse() {
            Ok(tz) => tzs.push((tz_name, tz)),
            Err(why) =>
                return Err(
                    Error::new(ErrorKind::InvalidData,
                               format!("Parsing error with: {}, {}",
                                       tz_name, why))
                ),
        }
    }
    Ok(tzs)
}

/// Calculate the maximum length of a vector of strings.
pub fn max_len(strs: &Vec<String>) -> usize {
    strs.iter().map(|s| s.len()).max().unwrap_or(0)
}

/// Pad a string from on the right
pub fn pad_to_size(s: &String, desired_len: usize) -> String {
    let mut s2 = s.clone();
    while s2.len() < desired_len {
        s2.push(' ');
    }
    s2
}

fn main() {
    let opt_home = env::home_dir();
    let args = env::args().collect::<Vec<String>>();
    let time_fmt = "%Y-%m-%d %H:%M %Z";

    match get_tz_file(&args, &opt_home) {
        Ok(conf_file) => {
            match read_file(&conf_file) {
                Ok(tzs) => {

                    // Calculate local time
                    let local_time = Local::now();

                    // The string to identify local timezone.
                    let local_time_string = "Local time".to_string();

                    // Get names of all time-zones
                    let tzs_names = tzs.iter().map(|s| s.0.clone()).collect();

                    // Find the maximum length of all time-zones
                    let mx_len = cmp::max::<usize>(local_time_string.len(),
                                                   max_len(&tzs_names));

                    // Print the local time first
                    println!("{}\t= {}",
                             pad_to_size(&local_time_string, mx_len),
                             local_time.format(time_fmt));

                    // Loop over the other time-zones, print their names
                    // and the current time transformed to that zone.
                    for tz in tzs {
                        println!("{}\t= {}",
                                 pad_to_size(&tz.0, mx_len),
                                 local_time.with_timezone(&tz.1)
                                           .format(time_fmt))
                    }
                },
                Err(why) => println!("Failed to read file {}: {}",
                                     conf_file.to_str().unwrap(),
                                     why)
            }
        },
        Err(why) => println!("Unable to retrieve name of config file:\n{}.", why)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use chrono_tz::Asia::Kolkata;

    #[test]
    fn test_get_tz_file() {
        let conf = "/tmp/test.conf".to_string();
        let prog = "prog_name".to_string();
        let args = vec![prog, conf];
        assert_eq!(
            get_tz_file(&args, &None).unwrap().to_str().unwrap(),
            args[1]
        )
    }

    #[test]
    #[should_panic]
    fn test_read_file_not_found() {
        read_file(&PathBuf::from("./test-data/non-existent.conf")).unwrap();
    }

    #[test]
    fn test_read_file() {
        assert_eq!(
            read_file(&PathBuf::from("./test-data/test.conf")).unwrap()[0].0,
            "Asia/Kolkata"
        );
        assert_eq!(
            read_file(&PathBuf::from("./test-data/test.conf")).unwrap()[0].1,
            Kolkata
        )
    }

    #[test]
    fn test_max_len() {
        assert_eq!(max_len(&vec!["abc".to_string(), "xy".to_string()]), 3)
    }

    #[test]
    fn test_max_len_empty() {
        assert_eq!(max_len(&vec![]), 0)
    }
}
