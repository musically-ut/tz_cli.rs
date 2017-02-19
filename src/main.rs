use std::env;
use std::path::PathBuf;

fn get_tz_file<'a>(args: &'a Vec<String>, opt_home: &'a Option<PathBuf>) -> Result<PathBuf, String> {
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

fn main() {
    let opt_home = env::home_dir();
    let args = env::args().collect::<Vec<String>>();
    match get_tz_file(&args, &opt_home) {
        Ok(conf_file) => println!("{} {}", "tz-file:", conf_file.to_str().unwrap()),
        Err(err_str) => println!("Unable to retrieve name of config file:\n{}.", err_str)
    }
}
