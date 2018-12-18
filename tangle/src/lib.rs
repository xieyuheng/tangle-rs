use std::path::Path;
use std::path::PathBuf;

use std::str::Lines;

use std::fs;
use std::io;
use std::env;

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

#[derive(Debug)]
struct File {
    path: PathBuf,
    content: String,
}

impl File {
    fn read (path: &Path) -> io::Result <File> {
        Ok (File {
            path: PathBuf::from (path),
            content: fs::read_to_string (path)?,
        })
    }
}

impl File {
    fn tangle (&self) -> Option <File> {
        match self.extension () .as_str () {
            "org" => self.org_tangle (),
            "md"  => self.md_tangle (),
            _ => None
        }
    }
}

impl File {
    fn extension (&self) -> String {
        if let Some (os_string) = self.path.extension () {
            if let Some (str) = os_string.to_str () {
                String::from (str)
            } else {
                String::new ()
            }
        } else {
            String::new ()
        }
    }
}

impl File {
    fn org_tangle (&self) -> Option <File> {
        if let Some (
            destination
        ) = org_find_destination (&self.content) {
            let mut path = PathBuf::new ();
            path.push (&self.path);
            path.pop ();
            path.push (destination);
            println! ("- org_tangle : {:?} => {:?}",
                      self.path, path);
            let content = org_tangle_content (
                &self.content) .unwrap ();
            Some (File { path, content })
        } else {
            None
        }
    }
}

impl File {
    fn md_tangle (&self) -> Option <File> {
        if let Some (
            destination
        ) = md_find_destination (&self.content) {
            let mut path = PathBuf::new ();
            path.push (&self.path);
            path.pop ();
            path.push (destination);
            println! ("- md_tangle : {:?} => {:?}",
                      self.path, path);
            let content = md_tangle_content (
                &self.content) .unwrap ();
            Some (File { path, content })
        } else {
            None
        }
    }
}

impl File {
    fn write (&self) -> io::Result <()> {
        fs::write (&self.path, &self.content)
    }
}

fn org_property_line_p (line: &str) -> bool {
    line .trim_start () .starts_with ("#+property:")
}

fn org_find_destination_in_property_line (
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

fn org_find_destination (string: &str) -> Option <String> {
    for line in string.lines () {
        if org_property_line_p (line) {
            let destination =
                org_find_destination_in_property_line (line);
            if destination. is_some () {
                return destination;
            }
        }
    }
    None
}

#[test]
fn test_org_find_destination () {
    let example = "#+property: tangle lib.rs";
    let destination = org_find_destination (example) .unwrap ();
    assert_eq! (destination, "lib.rs");

    let example = "#+property: header-args :tangle lib.rs";
    let destination = org_find_destination (example) .unwrap ();
    assert_eq! (destination, "lib.rs");
}

fn org_block_begin_line_p (line: &str) -> bool {
    line .trim_start () .starts_with ("#+begin_src")
}

fn org_block_end_line_p (line: &str) -> bool {
    line .trim_start () .starts_with ("#+end_src")
}

fn org_block_indentation (line: &str) -> usize {
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

fn org_collect_content (
    result: &mut String,
    lines: &mut Lines,
    indentation: usize,
) -> Result <(), TangleError> {
    for line in lines {
        if org_block_end_line_p (line) {
            result.push ('\n');
            return Ok (());
        } else {
            let line = line_trim_indentation (
                line, indentation);
            result.push_str (line);
            result.push ('\n');
        }
    }
    let error = TangleError::new ("org_block_end mismatch");
    Err (error)
}

fn org_tangle_content (string: &str) -> Result <String, TangleError> {
    let mut result = String::new ();
    let mut lines = string.lines ();
    while let Some (line) = lines.next () {
        if org_block_begin_line_p (line) {
            org_collect_content (
                &mut result,
                &mut lines,
                org_block_indentation (line))?;
        }
    }
    result.pop ();
    Ok (result)
}

#[test]
fn test_org_tangle_content () {
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
    let result = org_tangle_content (&example) .unwrap ();
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
    let result = org_tangle_content (&example) .unwrap ();
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
    let result = org_tangle_content (&example) .unwrap ();
    assert_eq! (expect, result);
}

fn md_meta_block_line_p (line: &str) -> bool {
    if ! line .trim_start () .starts_with ("---") {
        false
    } else {
        let string = line.trim ();
        string .chars () .all (|x| x == '-')
    }
}

fn md_meta_block_collect (
    lines: &mut Lines,
) -> Option <String> {
    let mut result = String::new ();
    for line in lines {
        if md_meta_block_line_p (line) {
            return Some (result);
        } else {
            result.push_str (line);
            result.push ('\n');
        }
    }
    None
}

fn md_find_meta_block (string: &str) -> Option <String> {
    let mut lines = string.lines ();
    if let Some (first_line) = lines.next () {
        if md_meta_block_line_p (first_line) {
            md_meta_block_collect (&mut lines)
        } else {
            None
        }
    } else {
        None
    }
}

const MD_DESTINATION_PREFIX: &'static str = "tangle:";

fn md_destination_line_p (line: &str) -> bool {
    line .trim_start () .starts_with (MD_DESTINATION_PREFIX)
}

fn md_find_destination (string: &str) -> Option <String> {
    let meta_block = md_find_meta_block (string);
    if meta_block.is_none () {
        return None;
    }
    let string = meta_block .unwrap ();
    for line in string.lines () {
        if md_destination_line_p (line) {
            let destination = &line [MD_DESTINATION_PREFIX.len () ..];
            let destination = destination.trim ();
            return Some (destination.to_string ());
        }
    }
    None
}

#[test]
fn test_md_find_destination () {
    let example = format! (
        "{}\n{}\n{}\n",
        "---",
        "tangle: core.rs",
        "---",
    );
    let destination = md_find_destination (&example) .unwrap ();
    assert_eq! (destination, "core.rs");
}

fn md_block_begin_line_p (line: &str) -> bool {
    line .trim_start () .starts_with ("```")
}

fn md_block_end_line_p (line: &str) -> bool {
    line .trim_start () .starts_with ("```")
}

fn md_collect_content (
    result: &mut String,
    lines: &mut Lines,
) -> Result <(), TangleError> {
    for line in lines {
        if md_block_end_line_p (line) {
            result.push ('\n');
            return Ok (());
        } else {
            result.push_str (line);
            result.push ('\n');
        }
    }
    let error = TangleError::new ("md_block_end mismatch");
    Err (error)
}

fn md_tangle_content (string: &str) -> Result <String, TangleError> {
    let mut result = String::new ();
    let mut lines = string.lines ();
    while let Some (line) = lines.next () {
        if md_block_begin_line_p (line) {
            md_collect_content (&mut result, &mut lines)?;
        }
    }
    result.pop ();
    Ok (result)
}

#[test]
fn test_md_tangle_content () {
    let example = format! (
        "{}\n{}\n{}\n{}\n",
        "``` rust",
        "hi",
        "hi",
        "```",
    );
    let expect = format! (
        "{}\n{}\n",
        "hi",
        "hi",
    );
    let result = md_tangle_content (&example) .unwrap ();
    assert_eq! (expect, result);
    let example = format! (
        "{}\n{}\n{}\n{}\n",
        "    ``` rust",
        "    hi",
        "    hi",
        "    ```",
    );
    let expect = format! (
        "{}\n{}\n",
        "    hi",
        "    hi",
    );
    let result = md_tangle_content (&example) .unwrap ();
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

pub fn file_tangle (path: &Path) -> io::Result <()> {
    if let Some (file) = File::read (path)? .tangle () {
        file.write ()
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
    println! ("- tangle_all_before_build");
    println! ("  current_dir : {:?}", current_dir);
    let path = absolute_lize (&path);
    dir_tangle_rec (&path)
}
