use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let mut args = std::env::args();
    let name = args.next().expect("argv[0] not set");
    for candidate in std::env::current_dir()
        .expect("could not get cwd")
        .ancestors()
        .map(|dir| dir.join(&name))
    {
        if let Ok(f) = File::open(&candidate) {
            let first_line = BufReader::new(f).lines().next();
            if matches!(first_line, Some(Ok(s)) if s == "#!/usr/bin/env dotslash") {
                let err = Command::new(&candidate).args(args).exec();
                panic!("failed to exec {candidate:?}: {err:#?}");
            }
        }
    }
    eprintln!("could not find {name} anywhere in ancestor tree");
    std::process::exit(254);
}
