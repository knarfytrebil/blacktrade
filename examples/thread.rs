use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Command, Stdio};

fn main() {
    let mut child =
        Command::new("/bin/bash")
        .arg("/Users/knarfytrebil/Programs/rust/bash_cmds/spot.sh")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Couldn't run program");

    let mut output = Vec::new();

    // Should be moved to a function that accepts something implementing `Write`
    {
        let stdout = child.stdout.as_mut().expect("Wasn't stdout");
        let stderr = child.stderr.as_mut().expect("Wasn't stderr");

        let mut stdout = BufReader::new(stdout);
        let mut stderr = BufReader::new(stderr);

        loop {
            let (stdout_bytes, stderr_bytes) = match (stdout.fill_buf(), stderr.fill_buf()) {
                (Ok(stdout), Ok(stderr)) => {
                    output.write_all(stdout).expect("Couldn't write");
                    output.write_all(stderr).expect("Couldn't write");

                    (stdout.len(), stderr.len())
                }
                other => panic!("Some better error handling here... {:?}", other)
            };

            if stdout_bytes == 0 && stderr_bytes == 0 {
                // Seems less-than-ideal; should be some way of
                // telling if the child has actually exited vs just
                // not outputting anything.
                break;
            }

            stdout.consume(stdout_bytes);
            stderr.consume(stderr_bytes);
        }
    }

    let status = child.wait().expect("Waiting for child failed");
    println!("Finished with status {:?}", status);
    println!("Combined output: {:?}", std::str::from_utf8(&output))
}
