// command_to_run.rs
// Definition of a CommandToRun structure to make it convenient
//
// 2025-10-30   PV      Move structure to this separate file, including exec/exec1/make_chunks so it can be shared
// 2025-11-04   PV      quoted_string doesn'r re-quote an already quoted string

use std::path::Path;
use std::process::Command;

// Represents a command to be executed where args contain {} placeholder(s)
#[derive(Debug, Default, Clone)]
pub struct CommandToRun {
    pub command: String,
    pub args: Vec<String>,
}

impl CommandToRun {
    pub fn exec1(&self, path: &Path, noaction: bool) -> Result<String, String> {
        let mut ctr_final = self.clone();

        // Special case
        if ctr_final.command.contains("{}") {
            ctr_final.command = ctr_final.command.replace("{}", path.display().to_string().as_str());
        }

        for refarg in ctr_final.args.iter_mut() {
            if refarg.contains("{}") {
                *refarg = refarg.replace("{}", path.display().to_string().as_str());
            }
        }

        ctr_final.exec(noaction)
    }

    pub fn exec(&self, noaction: bool) -> Result<String, String> {
        let res = format!("exec {} {}", quoted_string(&self.command), self.args.join(" "));
        if !noaction {
            let status = Command::new(self.command.as_str()).args(&self.args).spawn();
            if let Err(e) = status {
                return Err(format!("*** Error running command {}: {}", self.command, e));
            }
        }
        Ok(res)
    }

    // Helpers for grouped execution, breaking a parameters replacement into one or more RunCommands to ensure that an individual
    // command will not exceed len16_max UTF-16 characters
    // This code is not trivial...
    pub fn make_chunks(&self, paths: &[String], len16_max: usize) -> Vec<CommandToRun> {
        let mut res = Vec::<CommandToRun>::new();
        let mut braces_args = Vec::<Vec<String>>::new();

        fn transform_path(arg: &str, path: &str) -> String {
            // For now, we just support {}, but in the future, we may support transformations between {}
            arg.replace("{}", path)
        }

        // First pass, prepare transformations, calculate sizes of fixed args (without {})
        let mut len16_fixed: usize = 0;
        for arg in self.args.iter() {
            if arg.contains('{') {
                let ba = paths.iter().map(|pa| transform_path(arg, pa)).collect::<Vec<String>>();
                braces_args.push(ba);
            } else {
                len16_fixed += 1 + arg.encode_utf16().count();
            }
        }

        // Second pass, we build chunks
        let mut startixpath = 0; // We will add paths starting from this index
        let mut l16 = len16_fixed; // Cumulated length of command until previous ixpath

        fn add_chunk(res: &mut Vec<CommandToRun>, ctr: &CommandToRun, startixpath: usize, ixpath: usize, braces_args: &[Vec<String>]) {
            // Otherwise it's time to generate a new chunk
            let mut ctrchunk = CommandToRun {
                command: ctr.command.clone(),
                args: Vec::<String>::new(),
            };

            let mut ixba = 0; // Follows progression in braces_args, firs arg with {} is at index 0
            // We process args in sequence
            for arg in ctr.args.iter() {
                if arg.contains('{') {
                    let ba = &braces_args[ixba];
                    ixba += 1;
                    #[allow(clippy::needless_range_loop)]
                    // Clippy suggestion is actually less efficient:  for <item> in ba.iter().take(ixpath).skip(startixpath) {
                    for j in startixpath..ixpath {
                        ctrchunk.args.push(ba[j].clone());
                    }
                } else {
                    ctrchunk.args.push(arg.clone());
                }
            }

            res.push(ctrchunk);
        }

        // We iterate over all paths since we need to add them all
        for ixpath in 0..paths.len() {
            // First calculate in l16next the size of processed arguments
            let mut l16cur: usize = 0; // Length of processed paths for current ixpath
            for ba in braces_args.iter() {
                l16cur += 1 + ba[ixpath].encode_utf16().count();
            }

            // if it still fits, we continue (but we force the first one anyway)
            if l16 + l16cur < len16_max || startixpath == ixpath {
                l16 += l16cur;
                continue;
            }

            add_chunk(&mut res, self, startixpath, ixpath, &braces_args);

            startixpath = ixpath;
            l16 = len16_fixed + l16cur;
        }

        // There is always a last chunk
        add_chunk(&mut res, self, startixpath, paths.len(), &braces_args);

        res
    }
}


#[allow(unused)]
pub fn quoted_path(path: &Path) -> String {
    quoted_string(&path.display().to_string())
}

pub fn quoted_string(string: &str) -> String {
    if string.contains(' ') && !(string.starts_with('"') && string.ends_with('"')) {
        format!("\"{}\"", string)
    } else {
        string.into()
    }
}
