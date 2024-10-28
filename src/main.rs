use std::{env, fs, process};

fn main() -> process::ExitCode {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() == 0 {
        eprintln!("Usage: ./pc <file0> <file1> ...");
        return process::ExitCode::FAILURE;
    }

    let (oks, errs): (Vec<_>, Vec<_>) = args
        .iter()
        .map(|p| (p, fs::read_to_string(p)))
        .partition(|(_, res)| res.is_ok());

    if !errs.is_empty() {
        for (p, res) in errs {
            let _ = res.inspect_err(|e| eprintln!("[ERROR] '{p}' does not exist ({e})"));
        }
        return process::ExitCode::FAILURE;
    }

    for (_, res) in oks {
        let src = res.expect("This should not fail");
        // match compile(&src) {}
    }

    process::ExitCode::SUCCESS
}
