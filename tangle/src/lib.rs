use std::path::Path;
use std::path::PathBuf;

use std::str::Lines;

use std::fs;
use std::io;
use std::env;

#[derive(Debug)]
struct TangleError {
    report: String,
}

impl TangleError {
    fn new (report: &str) -> Self {
        TangleError {
            report: report.to_string (),
        }
    }
}

fn property_line_p (line: &str) -> bool {
    line .trim_start () .starts_with ("#+property:")
}

fn find_destination_in_property_line (
    line: &str,
) -> Option <String> {
    let mut words = line.split_whitespace ();
    while let Some (word) = words.next () {
        if word == "tangle" || word == ":tangle" {
            if let Some (destination) = words.next () {
                return Some (destination.to_string ())
            }
        }
    }
    None
}

fn find_destination (string: &str) -> Option <String> {
    for line in string.lines () {
        if property_line_p (line) {
            let destination =
                find_destination_in_property_line (line);
            if destination. is_some () {
                return destination;
            }
        }
    }
    None
}

#[test]
fn test_find_destination () {
    let example = "#+property: tangle lib.rs";
    let destination = find_destination (example) .unwrap ();
    assert_eq! (destination, "lib.rs");

    let example = "#+property: header-args :tangle lib.rs";
    let destination = find_destination (example) .unwrap ();
    assert_eq! (destination, "lib.rs");
}

fn block_begin_line_p (line: &str) -> bool {
    line .trim_start () .starts_with ("#+begin_src")
}

fn block_end_line_p (line: &str) -> bool {
    line .trim_start () .starts_with ("#+end_src")
}

fn block_indentation (line: &str) -> usize {
    let mut indentation = 0;
    for ch in line.chars () {
        if ch == ' ' {
            indentation += 1;
        } else {
            return indentation;
        }
    }
    0
}

    fn line_trim_indentation <'a> (
        mut line: &'a str,
        indentation: usize,
    ) -> &'a str {
        let mut counter = 0;
        while counter < indentation {
            if line.starts_with (' ') {
                counter += 1;
                line = &line[1..];
            } else {
                return line;
            }
        }
        line
    }

fn tangle_collect (
    result: &mut String,
    lines: &mut Lines,
    indentation: usize,
) -> Result <(), TangleError> {
    for line in lines {
        if block_end_line_p (line) {
            result.push ('\n');
            return Ok (());
        } else {
            let line = line_trim_indentation (
                line, indentation);
            result.push_str (line);
            result.push ('\n');
        }
    }
    let error = TangleError::new ("block_end mismatch");
    Err (error)
}

fn tangle (string: &str) -> Result <String, TangleError> {
    let mut result = String::new ();
    let mut lines = string.lines ();
    while let Some (line) = lines.next () {
        if block_begin_line_p (line) {
            tangle_collect (
                &mut result,
                &mut lines,
                block_indentation (line))?;
        }
    }
    result.pop ();
    Ok (result)
}

#[test]
fn test_tangle () {
    let example = format! (
        "{}\n{}\n{}\n{}\n",
        "#+begin_src rust",
        "hi",
        "hi",
        "#+end_src",
    );
    let expect = format! (
        "{}\n{}\n",
        "hi",
        "hi",
    );
    let result = tangle (&example) .unwrap ();
    assert_eq! (expect, result);

    let example = format! (
        "{}\n{}\n{}\n{}\n",
        "    #+begin_src rust",
        "    hi",
        "    hi",
        "    #+end_src",
    );
    let expect = format! (
        "{}\n{}\n",
        "hi",
        "hi",
    );
    let result = tangle (&example) .unwrap ();
    assert_eq! (expect, result);

    let example = format! (
        "{}\n{}\n{}\n{}\n",
        "#+begin_src rust",
        "    hi",
        "    hi",
        "#+end_src",
    );
    let expect = format! (
        "{}\n{}\n",
        "    hi",
        "    hi",
    );
    let result = tangle (&example) .unwrap ();
    assert_eq! (expect, result);
}

fn good_path_p (path: &Path) -> bool {
    for component in path.iter () {
        if let Some (string) = component.to_str () {
            if string.starts_with ('.') {
                if ! string .chars () .all (|x| x == '.') {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    true
}

pub fn org_file_p (file: &Path) -> bool {
    if let Some (os_string) = file.extension () {
        if let Some (string) = os_string.to_str () {
            string == "org"
        } else {
            false
        }
    } else {
        false
    }
}

pub fn file_tangle (file: &Path) -> io::Result <()> {
    if ! org_file_p (file) {
        return Ok (());
    }
    let string = fs::read_to_string (file)?;
    if let Some (destination) = find_destination (&string) {
        let result = tangle (&string) .unwrap ();
        let mut destination_path = PathBuf::new ();
        destination_path.push (file);
        destination_path.pop ();
        destination_path.push (destination);
        fs::write (&destination_path, result)?;
        println! (
            "- tangle : {:?} => {:?}",
            file.canonicalize ()?,
            destination_path.canonicalize ()?);
        Ok (())
    } else {
        Ok (())
    }
}

pub fn dir_tangle (dir: &Path) -> io::Result <()> {
    for entry in dir.read_dir ()? {
        if let Ok (entry) = entry {
            if good_path_p (&entry.path ()) {
                if entry.file_type ()? .is_file () {
                    file_tangle (&entry.path ())?
                }
            }
        }
    }
    Ok (())
}

pub fn dir_tangle_rec (dir: &Path) -> io::Result <()> {
    for entry in dir.read_dir ()? {
        if let Ok (entry) = entry {
            if good_path_p (&entry.path ()) {
                if entry.file_type ()? .is_file () {
                    file_tangle (&entry.path ())?
                } else if entry.file_type ()? .is_dir () {
                    dir_tangle_rec (&entry.path ())?
                }
            }
        }
    }
    Ok (())
}

pub fn absolute_lize (path: &Path) -> PathBuf {
    if path.is_relative () {
        let mut absolute_path = env::current_dir () .unwrap ();
        absolute_path.push (path);
        absolute_path
    } else {
        path.to_path_buf ()
    }
}

pub fn tangle_all_before_build () -> io::Result <()> {
    let path = Path::new (".");
    let current_dir = env::current_dir () .unwrap ();
    println! ("- org_tangle_engine");
    println! ("  tangle_all_before_build");
    println! ("  current_dir : {:?}", current_dir);
    let path = absolute_lize (&path);
    dir_tangle_rec (&path)
}
