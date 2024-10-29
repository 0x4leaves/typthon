use std::{env, error, fs, process, result};

use typthon::compile;

fn main() -> process::ExitCode {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() == 0 {
        eprintln!("Usage: ./pc <file0> <file1> ...");
        return process::ExitCode::FAILURE;
    }

    match drive(&args) {
        Ok(_) => process::ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            process::ExitCode::FAILURE
        }
    }
}

fn drive(paths: &[String]) -> result::Result<(), Box<dyn error::Error>> {
    let (oks, errs): (Vec<_>, Vec<_>) = paths
        .iter()
        .map(|p| (p, fs::read_to_string(p)))
        .partition(|(_, res)| res.is_ok());

    if !errs.is_empty() {
        let aggregated_errs: String = errs
            .into_iter()
            .map(|(p, res)| {
                res.map_err(|e| format!("IO Error: {e} (while handling '{p}')\n"))
                    .expect_err("This should contain an error")
            })
            .collect();
        return Err(aggregated_errs.trim_ascii_end().into());
    }

    for (p, res) in oks {
        let src = res.expect("This should not fail");
        let asm = compile(&src)?;
        fs::write(format!("{p}.S"), asm)?;
        // Need to decide what assembler to use
    }

    Ok(())
}
