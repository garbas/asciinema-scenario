use asciicast::{Entry, EventType, Header};
use failure::Error;
use serde::Deserialize;
use serde_json::{from_str, to_string};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::fs::write;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;
use structopt::StructOpt;
use structopt_flags::{LogLevel, Verbose};

const SVG_HEADER: &'static str = "<svg
   xmlns:dc=\"http://purl.org/dc/elements/1.1/\"
   xmlns:cc=\"http://creativecommons.org/ns#\"
   xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"
   xmlns:svg=\"http://www.w3.org/2000/svg\"
   xmlns=\"http://www.w3.org/2000/svg\"
   version=\"1.1\"
   width=\"100%\"
   viewBox=\"0 0 824 623\"
   preserveAspectRatio=\"xMidYMid meet\">
  <mask id=\"bigterminal-mask\">
    <rect x=\"0\" y=\"0\" width=\"824\" height=\"623\" fill=\"#fff\" />
  </mask>
  <rect class=\"background\" y=\"0\" x=\"0\" width=\"824\" height=\"623\" />
  <text mask=\"url(#bigterminal-mask)\" transform=\"translate(0 0)\" y=\"0\" x=\"0\" xml:space=\"preserve\">";

const SVG_FOOTER: &'static str = "</text>
</svg>";

#[derive(Deserialize, Debug)]
struct ScenarioHeader {
    #[serde(default = "default_step")]
    step: f64,

    #[serde(default = "default_width")]
    width: u32,

    #[serde(default = "default_height")]
    height: u32,
}

fn default_step() -> f64 {
    0.10
}

fn default_width() -> u32 {
    77
}

fn default_height() -> u32 {
    20
}

fn print_entry(entry: Entry) -> Result<(), Error> {
    println!("{}", to_string(&entry)?);
    Ok(())
}

fn clear_terminal(time: &mut f64, step: &f64) -> Result<(), Error> {
    *time += 18.0 * step;
    print_entry(Entry {
        time: *time,
        event_type: EventType::Output,
        event_data: "\r\x1b[2J\r\x1b[H".to_string(),
    })?;
    *time += 3.0 * step;
    Ok(())
}

fn echo_typing(time: &mut f64, step: &f64, line_raw: &str) -> Result<String, Error> {
    let mut bright_applied = false;
    for char in line_raw.to_string().chars() {
        *time += step;
        if char == '#' {
            print_entry(Entry {
                time: *time,
                event_type: EventType::Output,
                event_data: "\x1b[1m".to_string(),
            })?;
            bright_applied = true;
        }
        print_entry(Entry {
            time: *time,
            event_type: EventType::Output,
            event_data: char.to_string(),
        })?;
    }
    // clear
    if bright_applied {
        print_entry(Entry {
            time: *time,
            event_type: EventType::Output,
            event_data: "\x1b[0m".to_string(),
        })?;
    }

    *time += 3.0 * step;
    print_entry(Entry {
        time: *time,
        event_type: EventType::Output,
        event_data: "\r\n".to_string(),
    })?;

    let parts: Vec<&str> = line_raw.splitn(2, "#").collect();
    Ok(if parts.len() == 1 {
        parts[0].to_string()
    } else {
        format!(
            "{}<tspan class=\"fg-15\">#{}</tspan>",
            parts[0].to_string(),
            parts.clone().split_off(1).join(""),
        )
    })
}

fn echo_console_line(
    time: &mut f64,
    step: &f64,
    prompt: &str,
    line: &str,
) -> Result<Vec<String>, Error> {
    *time += step;

    let prompt_line: String;
    let mut preview_lines: Vec<String> = vec![];

    if prompt != "" {
        prompt_line = format!("\x1b[32m{}\x1b[0m$ ", prompt);
        preview_lines.push(format!("<tspan class=\"fg-2\">{}</tspan>$ ", prompt));
    } else {
        prompt_line = "$ ".to_string();
        preview_lines.push("$ ".to_string());
    };

    print_entry(Entry {
        time: *time,
        event_type: EventType::Output,
        event_data: prompt_line,
    })?;

    *time += 3.0 * step;

    preview_lines.push(echo_typing(time, step, line)?);

    Ok(preview_lines)
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "asciinema-scenario",
    about = "Create asciinema videos from a text file."
)]
struct Cli {
    #[structopt(flatten)]
    verbose: Verbose,

    scenario_file: String,

    #[structopt(name = "preview-file", long, short)]
    svg_preview_file: Option<String>,
}

fn main() -> Result<(), Error> {
    let cli = Cli::from_args();

    // Initialize logging
    let log_level = cli.verbose.get_level_filter();

    // stdout/stderr based logger
    TermLogger::init(
        log_level,            // set log level via "-vvv" flags
        Config::default(),    // how to format logs
        TerminalMode::Stderr, // log to stderr
        ColorChoice::Auto,    // color preference of an end user
    )?;

    // check if does not scenario_file exists
    if !Path::new(&cli.scenario_file).exists() {
        println!(
            "\x1b[31mERROR:\x1b[0m scenario file `{}` does not exist!",
            cli.scenario_file
        );
        exit(1);
    }

    // check if svg_preview_file exists
    if cli.svg_preview_file.is_some() && Path::new(cli.svg_preview_file.as_ref().unwrap()).exists()
    {
        println!(
            "\x1b[31mERROR:\x1b[0m svg preview file `{}` already exist!",
            cli.svg_preview_file.unwrap()
        );
        exit(1);
    }

    // Read lines from scenario_file
    let first_f = File::open(&cli.scenario_file)?;
    let mut first_reader = BufReader::new(first_f);

    // Header
    let mut first_line = String::new();
    first_reader.read_line(&mut first_line)?;

    let header: ScenarioHeader = if first_line.starts_with("#! ") {
        from_str(&first_line[3..])?
    } else {
        from_str(&"{}")?
    };
    let asciicast_header = Header {
        version: 2,
        width: header.width,
        height: header.height,
        timestamp: None,
        duration: None,
        idle_time_limit: None,
        command: None,
        title: None,
        env: None,
    };
    println!("{}", to_string(&asciicast_header)?);

    // The rest of the file
    // Read lines from scenario_file
    let mut preview_lines: Vec<Vec<String>> = vec![];
    let f = File::open(cli.scenario_file)?;
    let reader = BufReader::new(f);
    let mut time = 3.0 * header.step;
    for (index, maybe_line) in reader.lines().enumerate() {
        let line = maybe_line?;
        // skip when first line starts with "#! " since we already processed it above
        if index == 0 && line.starts_with("#! ") {
            continue;

        // lines starting with "#timeout: " will create defined timeout
        } else if line.starts_with("#timeout:") {
            {
                let timeout: f64 = line[9..].trim().parse()?;
                time += timeout;
            }

        // skip lines starting with "#"
        } else if line.starts_with("#") {
            continue;

        // lines starting with "$ " display as console lines
        } else if line.starts_with("$ ") {
            preview_lines.push(echo_console_line(&mut time, &header.step, "", &line[2..])?);

        // lines starting with "(nix-shell) $ " display as console lines
        } else if line.starts_with("(nix-shell) $ ") {
            preview_lines.push(echo_console_line(
                &mut time,
                &header.step,
                "(nix-shell) ",
                &line[14..],
            )?);

        // lines starting with "--" will clear display
        } else if line.starts_with("--") {
            clear_terminal(&mut time, &header.step)?;

        // timeout
        } else if line.trim() == "" {
            time += 3.0 * header.step;

        // everything else print immediately
        } else {
            print_entry(Entry {
                time: time,
                event_type: EventType::Output,
                event_data: format!("{}\r\n", line.clone()),
            })?;
            preview_lines.push(vec![line.to_string()]);
        }
    }

    match cli.svg_preview_file {
        Some(filename) => {
            write(
                filename,
                format!(
                    "{}{}{}",
                    SVG_HEADER,
                    preview_lines
                        .into_iter()
                        .map(|line_items| format!(
                            "<tspan x=\"0\" dy=\"1.2em\">{}</tspan>",
                            line_items.join("")
                        ))
                        .collect::<Vec<String>>()
                        .join(""),
                    SVG_FOOTER,
                ),
            )?;
            Ok(())
        }
        None => Ok(()),
    }
}
