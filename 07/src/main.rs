use std::{collections::HashMap, fs};

struct File {
    name: String,
    size: usize,
}
struct Directory {
    name: String,
    files: Vec<File>,
    subdirectories: Vec<String>,
}

impl Directory {
    fn new(name: &str) -> Self {
        return Directory {
            name: name.to_string(),
            files: Vec::new(),
            subdirectories: Vec::new(),
        };
    }
}
impl File {
    fn new(name: &str, size: usize) -> Self {
        return File {
            name: name.to_string(),
            size: size,
        };
    }
}

fn to_path(parts: &Vec<String>) -> String {
    return format!("/{}", parts.join("/"));
}

fn to_path_from_part(parts: &Vec<String>, part: &str) -> String {
    if parts.len() == 0 {
        return format!("/{}", part);
    }
    return format!("/{}/{}", parts.join("/"), part);
}

fn main() {
    assert!(play("sample.txt") == 95437);
    println!("{}", play("data.txt"));
}

fn play(input: &str) -> usize {
    let data = fs::read_to_string(input).expect("unable to read files");

    let entries = data.split("$");

    let mut flat_dirs: HashMap<String, Directory> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();

    for entry in entries {
        println!("state dump");
        println!("cwd '{}'", to_path(&stack));
        for (key, value) in &flat_dirs {
            println!("{}", key);
        }
        println!("end state dump");

        if entry.trim().len() == 0 {
            continue;
        }
        println!("Entry: {}", entry);
        let (command_string, rest_r) = entry
            .trim_start()
            .split_once("\r\n")
            .expect(&format!("Could not extract commandString from '{}'", entry));

        println!("Processing command {}", command_string);

        let (command, args) = command_string
            .trim_start()
            .split_once(" ")
            .or(Some((command_string, "")))
            .expect(&format!(
                "Could not extract command from '{}'",
                command_string
            ));

        println!("Command '{}' with args '{}'", command, args);

        match command {
            "cd" => match args.trim() {
                ".." => _ = stack.pop(),
                "/" => {
                    if !flat_dirs.contains_key("/") {
                        let new_dir = Directory::new("/");
                        flat_dirs.insert("/".to_string(), new_dir);
                    }
                }
                _ => {
                    stack.push(args.trim().to_string());

                    let cwd = to_path(&stack);

                    if !flat_dirs.contains_key(&cwd) {
                        let new_dir = Directory::new(args.trim());
                        flat_dirs.insert(cwd, new_dir);
                    }
                }
            },
            "ls" => {
                for line in rest_r.split("\r\n") {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    let (data, name) = line
                        .trim()
                        .split_once(" ")
                        .expect(&format!("Couldn't split LS line '{}'", line.trim()));

                    if data.trim() == "dir" {
                        let parent = to_path(&stack);
                        let cwd = to_path_from_part(&stack, name.trim());

                        println!("Discovered '{}'", cwd);

                        if !flat_dirs.contains_key(&cwd) {
                            flat_dirs
                                .get_mut(&parent)
                                .expect("parent?")
                                .subdirectories
                                .push(cwd.to_string());

                            let new_dir = Directory::new(name.trim());
                            flat_dirs.insert(cwd, new_dir);
                        }
                    } else {
                        let size: usize = data.trim().parse().expect("nan?");

                        let cwd = to_path(&stack);

                        let mut directory = flat_dirs.get_mut(&cwd).expect("no dir?");

                        directory.files.push(File::new(name.trim(), size));
                    }
                }
            }
            _ => assert!(false, "{} not recognized", command),
        }
    }

    let mut sizes: Vec<usize> = Vec::new();

    for (key, value) in &flat_dirs {
        let size = compute_recursive_size(&value, &flat_dirs);

        println!("{}: {}", key, size);

        if size < 100000 {
            sizes.push(size);
        }
    }

    return sizes.iter().sum();
}

fn compute_recursive_size(directory: &Directory, flat_map: &HashMap<String, Directory>) -> usize {
    return directory.files.iter().map(|f| f.size).sum::<usize>()
        + directory
            .subdirectories
            .iter()
            .map(|subdir| {
                return compute_recursive_size(&flat_map[subdir], flat_map);
            })
            .sum::<usize>();
}
