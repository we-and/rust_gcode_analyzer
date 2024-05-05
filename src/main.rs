use async_std::fs::File;
use async_std::io::{BufReader, prelude::*};
use std::time::Instant;
use async_std::io::{self,};
use async_std::prelude::*;
use std::env;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
struct SlicerInfo {
    id: u32,
    name: String,
    gather_info_after: Vec<String>,
    version: Regex,
    info_finder: HashMap<String, String>,
}

struct GcodeAnalyzer {
    slicer_identifiers: HashMap<String, SlicerInfo>,
    active_slicer: Option<SlicerInfo>,
    current_line_number: usize,
    malicious_gcode_list: Vec<String>,
    gather_info_after: Vec<String>,
    do_gather_info: bool,
    flushing: bool,
    flushing_start_lines: Vec<String>,
    flushing_end_lines: Vec<String>,
    active_tool: u32,
    max_tool_number: u32,
    total_filament: HashMap<u32, (f64, f64)>, // filament usage per tool
    result: AnalysisResult,
}

struct AnalysisResult {
    estimate: Option<u64>, // estimation time in seconds
}

impl GcodeAnalyzer {
    fn new() -> Self {
        let mut slicer_identifiers = HashMap::new();
        let cura_regex = Regex::new(r"\b\d+\.\d+\.\d+\b").unwrap();
        slicer_identifiers.insert("Cura_SteamEngine".to_string(), SlicerInfo {
            id: 1,
            name: "Cura".to_string(),
            gather_info_after: vec![";LAYER:".to_string()],
            version: cura_regex,
            info_finder: HashMap::from([
                ("nozzleSize".to_string(), ";EXTRUDER_TRAIN.0.NOZZLE.DIAMETER:".to_string()),
                ("layerHeight".to_string(), ";Layer height: ".to_string()),
                // Additional fields omitted
            ]),
        });

        Self {
            current_line_number: 0,
            active_slicer: None,
            gather_info_after: vec![],
            do_gather_info: false,
            flushing: false,
            active_tool: 0,
            max_tool_number: 0,
            total_filament: HashMap::new(),
            result: AnalysisResult { estimate: None },
            flushing_start_lines: vec!["; FLUSH_START".to_string()],
            flushing_end_lines: vec!["; FLUSH_END".to_string()],
            slicer_identifiers,
            malicious_gcode_list: vec![
                "M999".to_string(), "M997".to_string(), // Others omitted
            ],
            // more initialization
        }
    }

 




    // more methods to do
}



impl GcodeAnalyzer {

    fn process_line(&mut self, line: &str, total_lines: Option<usize>) {
        self.current_line_number += 1;
        println!("Process line {}",line);
        let line = if line.starts_with(' ') { line.trim() } else { line };
        let first_space = line.find(' ').unwrap_or(line.len());
        let command = &line[0..first_space].split(';').next().unwrap().replace("\r", "").to_uppercase();

        match command.as_str() {
            "G0" | "G1" => {
                self.command_linear_movement_process(command, line);
            },
            _ => {
                if self.gather_info_after.iter().any(|info| line.contains(info)  && (line.len() - info.len() <= 30)) {
                    self.do_gather_info = true;
                }

                if self.flushing_start_lines.contains(&line.to_string()) {
                    self.flushing = true;
                } else if self.flushing_end_lines.contains(&line.to_string()) {
                    self.flushing = false;
                }

                if line.starts_with('T') {
                    if let Ok(t) = line[1..].split(';').next().unwrap().parse::<u32>() {
                        if t != self.active_tool && t <= 60 {
                            self.active_tool = t;
                            self.max_tool_number = std::cmp::max(self.max_tool_number, t);
                            self.total_filament.entry(t).or_insert((0.0, 0.0));
                        }
                    }
                }

                self.try_extract_print_estimate(line);
            }
        }
    }

    fn command_linear_movement_process(&mut self, command: &str, line: &str) {
        // Process G0 or G1 linear movement commands
    }

    fn try_extract_print_estimate(&mut self, line: &str) {
        // Extract print time estimates based on slicer-specific comments
    }
}


fn to_absolute_path<P: AsRef<Path>>(relative_path: P) -> PathBuf {
    match env::current_dir() {
        Ok(current_dir) => {
            let mut absolute_path = PathBuf::from(current_dir);
            absolute_path.push(relative_path);
            absolute_path
        },
        Err(_) => {
            // Fallback to just returning the relative path as is if unable to fetch current directory
            PathBuf::from(relative_path.as_ref()) 
        }
    }
}


enum FileType {
    Path,
    File,
    Array(Vec<String>),
}

struct FileAnalyzer {
    file_path: String,
    file: FileType,
    progress_callback: Box<dyn Fn(f64) + Send>,
    done_callback: Box<dyn Fn(String, u128) + Send>,
    error_callback: Box<dyn Fn(String) + Send>,
}

impl FileAnalyzer {
    async fn analyze(&self) {
        println!("Analyze start...");
        let start_time = Instant::now();

        match &self.file {
            FileType::Path => {
        
                let absolute_path = to_absolute_path(&self.file_path);
                println!("Analyze from path. {}",absolute_path.display());
                if let Ok(file) = File::open(&absolute_path).await {
                    let reader = BufReader::new(file);
                    let mut lines = reader.lines();

                    while let Some(line) = lines.next().await {
                        if let Ok(line) = line {
                            // Process line here
                            println!("Processing line: {}", line);
                        }
                    }
                }else{
                    println!("File not found");

                }
            },
            FileType::File=>{
                println!("Analyze from file.");

            },
            FileType::Array(strings) => {
                println!("Analyze from array.");

                for line in strings {
                    // Process line here
                    println!("Processing line: {}", line);
                }
            }
        }

        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time).as_millis();
        println!("Analyze done...");
        
        (self.done_callback)("Result from analysis".to_string(), duration);
    }
         
}

fn main() {

    let mut analyzer = GcodeAnalyzer::new();
    analyzer.process_line("G1 X0.1 Y0.2 E0.005", None);
    analyzer.process_line("; FLUSH_START", None); 

    // Setup the analyzer with callbacks and file type
    let analyzer = FileAnalyzer {
        file: FileType::Path,
        file_path:"gcode_example_files/square_layers.gcode".to_string(),
        progress_callback: Box::new(|progress| println!("Progress: {}%", progress)),
        done_callback: Box::new(|result, time| println!("Done. Result: {}, Time: {}ms", result, time)),
        error_callback: Box::new(|e| println!("Error: {}", e)),
    };

    // Run the analyzer (ensure to use Tokio runtime if using Tokio instead of async-std)
    async_std::task::block_on(analyzer.analyze());
}