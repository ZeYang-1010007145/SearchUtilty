use std::error::Error;
use std::fs;
use std::path::PathBuf;
use glob::glob;
use walkdir::WalkDir;
use colored::*;
/**
 * 
 * 
Usage: grep [OPTIONS] <pattern> <files...>
Options:
-i                Case-insensitive search
-n                Print line numbers
-v                Invert match (exclude lines that match the pattern)
-r                Recursive directory search
-f                Print filenames
-c                Enable colored output
-h, --help        Show help information
 * 
 * 
*/

// refer to the io project in the Rust book
pub struct Config {
    pub query: String,
    pub file_paths: Vec<PathBuf>,
    pub case_insensitive: bool,
    pub line_number: bool,
    pub invert_match: bool,
    pub recursive_search:bool,
    pub print_filenames: bool,
    pub colored_output :bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Skip the program name

        let query = match args.next() {
            Some(arg) if arg == "-h" || arg == "--help" => {
                return Err(
                    "Usage: grep [OPTIONS] <pattern> <files...>\nOptions:\n-i\tCase-insensitive search\n-n\tPrint line numbers\n-v\tInvert match (exclude lines that match the pattern)\n-r\tRecursive directory search\n-f\tPrint filenames\n-c\tEnable colored output\n-h, --help\tShow help information",
                );
            }
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        // Set default values for options
        let mut case_insensitive = false;
        let mut line_number = false;
        let mut invert_match = false;
        let mut recursive_search = false;
        let mut print_filenames = false;
        let mut colored_output = false;

        // A vector to hold all the file paths
        let mut file_paths: Vec<PathBuf> = Vec::new();

        // Parse the remaining arguments
        while let Some(arg) = args.next() {
            if arg.starts_with('-') {
                // Handle options
                match arg.as_str() {
                    "-i" => case_insensitive = true,
                    "-n" => line_number = true,
                    "-v" => invert_match = true,
                    "-r" => recursive_search = true,
                    "-f" => print_filenames = true,
                    "-c" => colored_output = true,
                    "-h" | "--help" => {
                        return Err(
                            "Usage: grep [OPTIONS] <pattern> <files...>\nOptions:\n-i\tCase-insensitive search\n-n\tPrint line numbers\n-v\tInvert match (exclude lines that match the pattern)\n-r\tRecursive directory search\n-f\tPrint filenames\n-c\tEnable colored output\n-h, --help\tShow help information",
                        );
                    }
                    _ => return Err("Unknown option encountered"),
                }
            } else {
                // Handle file paths and wildcards
                if arg.contains('*') {
                    // Handle wildcard expansion using glob for patterns like *.md
                    match glob(&arg) {
                        Ok(paths) => {
                            for path in paths {
                                match path {
                                    Ok(path_buf) => file_paths.push(path_buf),
                                    Err(e) => eprintln!("Error reading path: {:?}", e),
                                }
                            }
                        }
                        Err(e) => eprintln!("Failed to read glob pattern {}: {}", arg, e),
                    }
                } else {
                    // Preserve relative paths (like "../") 
                    file_paths.push(PathBuf::from(arg));
                }
            }
        }

        // Ensure at least one file is provided
        if file_paths.is_empty() {
            return Err("Didn't get any file paths");
        }

        // Return the constructed Config object
        Ok(Config {
            query,
            file_paths,
            case_insensitive,
            line_number,
            invert_match,
            recursive_search,
            print_filenames,
            colored_output,
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for file_path in &config.file_paths {
        if config.recursive_search {
            search_recursive(&config, file_path.to_str().unwrap())?;
        } else {
            let contents = fs::read_to_string(file_path)?;
            search_and_print(&config, file_path, &contents)?;
        }
    }
    Ok(())
}

fn search_and_print(config: &Config, file_path: &PathBuf, contents: &str) -> Result<(), Box<dyn Error>> {
    for (line_number, line) in contents.lines().enumerate() {
        let matches = if config.case_insensitive {
            line.to_lowercase().contains(&config.query.to_lowercase())
        } else {
            line.contains(&config.query)
        };

        let should_print = if config.invert_match { !matches } else { matches };

        if should_print {
            print_result(config, file_path, line, line_number + 1);
        }
    }
    Ok(())
}

fn search_recursive(config: &Config, folder: &str) -> Result<(), Box<dyn Error>> {
    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            if let Ok(contents) = fs::read_to_string(file_path) {
                search_and_print(config, &file_path.to_path_buf(), &contents)?;
            }
        }
    }
    Ok(())
}

fn highlight_query(line: &str, query: &str, case_insensitive: bool) -> String {
    if case_insensitive {
        let mut result = String::new();
        let mut last_index = 0;
        for (start, part) in line.to_lowercase().match_indices(&query.to_lowercase()) {
            result.push_str(&line[last_index..start]);
            result.push_str(&line[start..start + part.len()].red().bold().to_string());
            last_index = start + part.len();
        }
        result.push_str(&line[last_index..]);
        result
    } else {
        line.replace(query, &query.red().bold().to_string())
    }
}
fn print_result(config: &Config, file_path: &PathBuf, line: &str, line_number: usize) {
    let mut output = String::new();

    if config.print_filenames {
        output.push_str(&format!("{}: ", file_path.display()));
    }

    if config.line_number {
        output.push_str(&format!("{}: ", line_number));
    }

    if config.colored_output {
        output.push_str(&highlight_query(line, &config.query, config.case_insensitive));
    } else {
        output.push_str(line);
    }

    println!("{}", output);
}
//  /**
//  * print filenames + color output + recursive directory search
//  * recursive directory search + print filenames
//  * recursive directory search
//  * invert match
//  * line number
//  * multiple files
//  * 
// */
// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let mut results = Vec::new();

//     for file_path in &config.file_paths {
//         if config.recursive_search {
//             results.extend(search_recursive(&config.query, file_path.to_str().unwrap()));
//         } else {
//             let contents = fs::read_to_string(file_path)?;
//             let file_results = if config.case_insensitive {
//                 search_case_insensitive(&config.query, &contents)
//             } else if config.invert_match {
//                 invert_search(&config.query, &contents)
//             } else if config.line_number {
//                 print_linenumbers(&config, &contents)
//             } else {
//                 search(&config.query, &contents)
//             };

//             for line in file_results {
//                 results.push(format!("{}: {}", file_path.display(), line));
//             }
//         }
//     }

//     // Print the results using the print_result function
//     for result in results {
//         let (file_path, line) = result.split_once(": ").unwrap();
//         print_result(&PathBuf::from(file_path), line, &config);
//     }

//     Ok(())
// }
// // Perform Basic Search: refer to the io project in the Rust book
// pub fn search(query: &str, contents: &str) -> Vec<String> {
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line.to_string()); // Convert &str to String
//         }
//     }
//     results
// }
// // Perform case insensitive search: refer to the io project in the Rust book
// pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<String> {
//     let query = query.to_lowercase();
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line.to_string()); // Convert &str to String
//         }
//     }
//     results
// }
// // Perform unmatching search
// pub fn invert_search(query: &str,dir: & str) -> Vec<String>{
//     let query = query;
//     let mut results = Vec::new();
//     for line in dir.lines() {
//         if !line.contains(&query) {
//             results.push(line.to_string());
//         }
//     }

//     results
// }
// // Perform recursive searching for all satisfied files
// pub fn search_recursive(query: &str, folder: &str) -> Vec<String> {
//     let mut all_results = Vec::new();

//     // WalkDir performs a depth-first search by default
//     for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
//         if entry.file_type().is_file() {
//             let file_path = entry.path();
//             if let Ok(contents) = fs::read_to_string(file_path) {
//                 let results = search(query, &contents);
//                 for line in results {
//                     all_results.push(format!("{}: {}", file_path.display(), line));
//                 }
//             }
//         }
//     }

//     // Reverse the results to get deepest matches first
//     all_results.reverse();
//     all_results
// }
// //Print result with line number
// pub fn print_linenumbers(config: &Config,dir: & str)->Vec<String> {
//     let mut results = Vec::new();
//     // Iterate over the lines with their line numbers (starting from 1)
//     for (line_number, line) in dir.lines().enumerate() {
//         if !line.contains(&config.query) && config.invert_match {
//             // Format the line as "line_number: line"
//             results.push(format!("{}: {}", line_number + 1, line));
//         }else if line.contains(&config.query) {
//             results.push(format!("{}: {}", line_number + 1, line));
//         }
//     }
//     results
// }
// fn highlight_query(line: &str, query: &str) -> String {
//     let colored_query = query.red().bold(); // Color and style the query
//     line.replace(query, &colored_query.to_string()) // Replace query with colored version
// }
// // Define the print_result function to handle printing logic
// fn print_result(file_path: &PathBuf, line: &str,  config: &Config) {
//     if config.colored_output && config.print_filenames {
//         println!("{}: {}", file_path.display(), highlight_query(line,  &config.query)); // Use `file_path`
//     } else if config.colored_output {
//         println!("{}", highlight_query(line, &config.query));
//     } else if config.print_filenames {
//         println!("{}: {}", file_path.display(), line); // Use `file_path`
//     } else {
//         println!("{}", line);
//     }
// }

