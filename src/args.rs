use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

use clap::{ArgGroup, Parser};

#[derive(Debug, Parser)]
#[command(name = "EsotericReaction")]
#[command(author = "bigyhsuan")]
#[command(version = "0.0.0")]
#[clap(group(ArgGroup::new("source").required(false).multiple(false).args(&["file", "code"])))]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
    #[arg(short, long, value_name = "CODE")]
    code: Option<String>,
}

pub fn parse_args() -> String {
    let args = Args::parse();
    let source = match (args.file, args.code) {
        (None, Some(code)) => code,
        (Some(file), None) => fs::read_to_string(file)
            .unwrap_or_else(|err| panic!("could not read from stdin: {}", err)),
        (Some(_), Some(_)) => panic!("only 1 of `--file` or `--code` is allowed"),
        (None, None) => {
            let mut s = String::new();
            io::stdin()
                .read_to_string(&mut s)
                .unwrap_or_else(|err| panic!("could not read from stdin: {}", err));
            s
        }
    };
    let source = source + "\n";
    source
}
