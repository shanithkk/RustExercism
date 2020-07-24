use failure::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Flags {
    line_number: bool,
    only_file_name: bool,
    case_insensitive: bool,
    invert: bool,
    match_entire_lines: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {
            line_number: flags.contains(&"-n"),
            only_file_name: flags.contains(&"-l"),
            case_insensitive: flags.contains(&"-i"),
            invert: flags.contains(&"-v"),
            match_entire_lines: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut matches = vec![];

    let pattern = if flags.case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    for file_name in files {
        match File::open(file_name) {
            Err(e) => return Err(failure::err_msg(e.to_string())),
            Ok(mut file) => {
                let mut contents = String::new();
                if let Err(e) = file.read_to_string(&mut contents) {
                    return Err(failure::err_msg(e.to_string()));
                }
                for (n, line) in contents.lines().enumerate() {
                    let normalised_line = if flags.case_insensitive {
                        line.to_lowercase()
                    } else {
                        line.to_string()
                    };

                    if flags.invert
                        ^ ((flags.match_entire_lines && normalised_line == pattern)
                        || (!flags.match_entire_lines && normalised_line.contains(&pattern))) {
                        if flags.only_file_name {
                            matches.push(file_name.to_string());
                            break;
                        }

                        let mut formatted = String::new();

                        if files.len() > 1 {
                            formatted.push_str(file_name);
                            formatted.push(':');
                        }

                        if flags.line_number {
                            formatted.push_str(&(n + 1).to_string());
                            formatted.push(':');
                        }

                        formatted.push_str(&line);

                        matches.push(formatted);
                    }
                }
            }
        }
    }

    Ok(matches)
}