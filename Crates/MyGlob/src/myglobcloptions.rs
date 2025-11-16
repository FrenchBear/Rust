// myglobcloptions.rs
// Standard MyGlob Command Line Options processing so it's consistent across various applications
//
// 2025-11-16   PV

#[derive(Debug, Default)]
pub struct MyGlobCLOptions {
    pub case_sensitive: bool,
    pub autorecurse: bool,
    pub link_mode: usize,
    pub max_depth: usize,
    pub no_glob_filtering: bool,
    pub filters: Vec<String>,
}

impl MyGlobCLOptions {

    /// Provide a new instance of MyGlobCLOptions with predefined options autorecurse:true, link_mode:1
    /// If this default is not pertinent for an app, create and initialize a nex instance directly in app
    pub fn new() -> MyGlobCLOptions {
        MyGlobCLOptions {
            autorecurse: true,
            link_mode: 1,
            ..Default::default()
        }
    }

    pub fn options() -> &'static str {
        "⌊Globbing options⌋:
⦃ci⦄       ¬Case-insensitive search (default)
⦃cs⦄       ¬Case-sensitive search (see following note for this option)
⦃a+⦄|⦃a-⦄    ¬Enable or disable glob autorecurse mode
⦃l 0|1|2⦄  ¬Links mode: 0=ignore links, 1=include links but don't follow them (default), 2=follow links
⦃md⦄ ⟨n⟩     ¬Limit the recursion depth of ** segments, 1=One directory only, ... Default=0 is unlimited depth
⦃ngf⦄      ¬No glob filtering: $RECYCLE.BIN, .git and System Volume Information are not filtered out
⦃f⦄ ⟨name⟩   ¬Add ⟨name⟩ to the list of excluded folders (simple folder name, no path, no *)
Multiple options can be separated by comma, use double quote around options if they contain spaces."
    }

    pub fn process_options(&mut self, opts: &str) -> Result<(), String> {
        let topt = opts.split(',');
        let mut it = topt.into_iter();

        while let Some(opt) = it.next() {
            let optlc = opt.to_lowercase();
            match optlc.as_str() {
                "cs" => self.case_sensitive = true,
                "ci" => self.case_sensitive = false,

                "a+" => self.autorecurse = true,
                "a-" => self.autorecurse = false,

                "l0" => self.link_mode = 0,
                "l1" => self.link_mode = 1,
                "l2" => self.link_mode = 2,
                "l" => {
                    if let Some(aopt) = it.next() {
                        if aopt == "0" {
                            self.link_mode = 0;
                        } else if aopt == "1" {
                            self.link_mode = 1;
                        } else if aopt == "2" {
                            self.link_mode = 2
                        } else {
                            return Err("glob option l expects arguemnt 0, 1 or 2".into());
                        }
                    } else {
                        return Err("glob option l expects arguemnt 0, 1 or 2".into());
                    }
                }

                "ngf" => self.no_glob_filtering = true,

                _ => {
                    if let Some(aarg) = optlc.strip_prefix("a") {
                        let aopt = aarg.trim();
                        if aopt == "+" {
                            self.autorecurse = true;
                        } else if aopt == "-" {
                            self.autorecurse = false;
                        } else {
                            return Err("glob option a expects arguemnt + or -".into());
                        }
                    } else if let Some(mdarg) = optlc.strip_prefix("md") {
                        let res = mdarg.trim().parse::<usize>();
                        match res {
                            Ok(n) => {
                                self.max_depth = n;
                                continue;
                            }
                            Err(_) => {
                                return Err("glob option md expects an integer ≥0 argument ".into());
                            }
                        }
                    } else if let Some(farg) = optlc.strip_prefix("f") {
                        // Guaranteed to have at least 1 char after f
                        self.filters.push(farg.trim().to_string());
                        continue;
                    }

                    return Err(format!("Unknown/unsupported glob option {opt}"));
                }
            }
        }

        Ok(())
    }
}
