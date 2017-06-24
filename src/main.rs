extern crate clap;
extern crate polydoc_js;


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
            .after_help(
r#"Parse one or more source files to extract documentation information, and output the results as JSON.

If no source files are provided, source code is read from standard input."#)
    };

    let args = cli.get_matches();
    let inputs = args.values_of("inputs");
    let output = match inputs
    {
        Some(inputs) =>
        {
            let mut docs = Vec::new();
            for input in inputs
            {
                let mut filestring = String::new();
                let mut file = File::open(input).expect("open");
                file.read_to_string(&mut filestring).expect("read");

                // TODO: Allow any combination of language parsing functions, doc parsing function, and merging functions
                let mut filedocs = polydoc_js::generate(&filestring);
                docs.append(&mut filedocs);
            }
            docs
        },
        None =>
        {
            let mut stdin = String::new();
            std::io::stdin().read_to_string(&mut stdin).expect("polydoc: Failed to read from stdin.");

            // TODO: Allow any combination of language parsing functions, doc parsing function, and merging functions
            let docs = polydoc_js::generate(&stdin);
            docs
        }
    };
    println!("{:?}", output);
}