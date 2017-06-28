extern crate clap;
extern crate polydoc;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;


use serde::Serialize;


fn main()
{
    use std::fs::File;
    use std::io::Read;
    use clap::{App, Arg};

    let cli = {
        App::new("polydoc")
            .author("Pirh, <***redacted.email@redacted.nope***>")
            .version("0.1.0")
            .about("Parse code and comments and generate JSON-formatted documentation.")
            .arg(
                Arg::with_name("inputs")
                    .help("Source files to parse.")
                    .takes_value(true)
                    .multiple(true)
            )
            .arg(
                Arg::with_name("format")
                    .short("-f")
                    .long("--format")
                    .help("Output format to use.")
                    .possible_values(&["json", "yaml"])
                    .default_value("json")
            )
            .after_help(
r#"Parse one or more source files to extract documentation information, and output the results as JSON.

If no source files are provided, source code is read from standard input."#)
    };

    let args = cli.get_matches();
    let inputs = args.values_of("inputs");

    let format = args.value_of("format").unwrap();

    match inputs
    {
        Some(inputs) =>
        {
            use std::collections::HashMap;

            let mut file_docs = HashMap::new();

            for filename in inputs
            {
                let mut filestring = String::new();
                let mut file = File::open(filename).expect("Open failed");
                file.read_to_string(&mut filestring).expect("Read failed");

                let docs = polydoc::parse_from_source(&filestring);
                file_docs.insert(filename, docs);
            }

            let serialized = serialize(&file_docs, format).expect("Failed to serialize");
            println!("{}", serialized);
        },
        None =>
        {
            let mut stdin = String::new();
            std::io::stdin().read_to_string(&mut stdin).expect("polydoc: Failed to read from stdin.");

            let docs = polydoc::parse_from_source(&stdin);
            let serialized = serialize(&docs, format).expect("Failed to serialize");
            println!("{}", serialized);
        }
    };
}


fn serialize<T>(items: &T, format: &str) -> Option<String>
where
    T: Serialize
{
    match format
    {
        "json" => serde_json::to_string(items).ok(),
        "yaml" => serde_yaml::to_string(items).ok(),
        _ => unreachable!()
    }
}