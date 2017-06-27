extern crate clap;
extern crate polydoc;
extern crate polydoc_core;
extern crate polydoc_js;
extern crate serde_json;
extern crate serde_yaml;


use polydoc_core::DocumentedItem;


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

    let doc_parse_func = polydoc_core::docparsing::extract_docs;
    let source_parse_func = polydoc_js::extract_declarations;
    let merge_func = polydoc_core::merge::merge_docs_with_decls;
    let serialize_func = match args.value_of("format").unwrap()
    {
        "json" => serialize_to_json,
        "yaml" => serialize_to_yaml,
        _ => unreachable!()
    };

    match inputs
    {
        Some(inputs) =>
        {
            for input in inputs
            {
                let mut filestring = String::new();
                let mut file = File::open(input).expect("Open failed");
                file.read_to_string(&mut filestring).expect("Read failed");

                let serialized = polydoc::polydoc(&filestring, &doc_parse_func, &source_parse_func, &merge_func, &serialize_func).expect("polydoc: error in inputs");
                println!("{}", serialized);
            }
        },
        None =>
        {
            let mut stdin = String::new();
            std::io::stdin().read_to_string(&mut stdin).expect("polydoc: Failed to read from stdin.");

            let serialized = polydoc::polydoc(&stdin, &doc_parse_func, &source_parse_func, &merge_func, &serialize_func).expect("polydoc: error in inputs");
            println!("{}", serialized);
        }
    };
}


fn serialize_to_json(items: &[DocumentedItem]) -> Option<String>
{
    serde_json::to_string(items).ok()
}


fn serialize_to_yaml(items: &[DocumentedItem]) -> Option<String>
{
    serde_yaml::to_string(items).ok()
}