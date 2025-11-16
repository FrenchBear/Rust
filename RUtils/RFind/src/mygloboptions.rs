// mygloboptions
// Standard MyGlob command line options management
//
// 2025-11-16   PV

#[derive(Debug, Default)]
pub struct GlobCLOptions {
    pub case_sensitive: bool,
    pub autorecurse: bool,
    pub link_mode: usize,
    pub max_depth: usize,
    pub no_glob_filtering: bool,
    pub filters: Vec<String>,
}

impl GlobCLOptions {
    pub fn new() -> GlobCLOptions {
        GlobCLOptions { ..Default::default() }
    }

    pub fn process_options(&mut self, opts: &str) -> Result<(), String> {
        let topt = opts.split(',');
        let mut it = topt.into_iter();

        while let Some(opt) = it.next() {
            let optlc = opt.to_lowercase();
            match (optlc.as_str()) {
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
                    if optlc.starts_with("a") {
                        let aopt = optlc[1..].trim();
                        if aopt == "+" {
                            self.autorecurse = true;
                        } else if aopt == "-" {
                            self.autorecurse = false;
                        } else {
                            return Err("glob option a expects arguemnt + or -".into());
                        }
                    } else if optlc.starts_with("md") {
                        let res = optlc[2..].trim().parse::<usize>();
                        match (res) {
                            Ok(n) => {
                                self.max_depth = n;
                                continue;
                            }
                            Err(e) => {
                                return Err("glob option md expects an integer ≥0 argument ".into());
                            }
                        }
                    } else if optlc.starts_with("f") {
                        // Guaranteed to have at least 1 char after f
                        self.filters.push(optlc[1..].trim().to_string());
                        continue;
                    }

                    return Err(format!("Unknown/unsupported glob option {opt}"));
                }
            }
        }

        Ok(())
    }
}
