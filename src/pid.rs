use std::env;
use std::fs;
use std::process;

#[derive(Debug, Default)]
pub struct PidInfo {
    pub pid: String,
    pub cmdline: String,
    pub exe: String,
    pub cwd: String,
    pub stat: PidStat,
    pub tasks: usize,
    pub fds: usize,
    pub limits: String,
}

impl PidInfo {
    pub fn new() -> PidInfo {
        let mut args = env::args();

        let pn = args.nth(0).unwrap();
        let pid = args.nth(0).unwrap_or_else(|| {
            println!("Usage: {} <pid>", pn);
            process::exit(-1);
        });

        let pid = pid.trim().to_string();
        check_pid(&pid);

        PidInfo {
            pid,
            ..Default::default()
        }
    }

    pub fn path(&self, s: &str) -> String {
        format!("/proc/{}/{}", self.pid, s)
    }
}

#[derive(Debug, Default)]
pub struct PidStat {
    pub rss: u32,
}

impl PidStat {
    pub fn parse(s: String) -> PidStat {
        let mut pstat = PidStat::default();
        let stats: Vec<&str> = s.split_whitespace().collect();

        pstat.rss = stats[23].parse().unwrap_or_default();
        pstat.rss *= 4; // page size: 4k

        pstat
    }
}

fn check_pid(pid: &str) {
    let base = format!("/proc/{}/", pid);

    match fs::symlink_metadata(base) {
        Err(_) => {
            println!("illegal pid! {}", pid);
            process::exit(-2);
        }

        Ok(md) => {
            if md.is_dir() {
            } else {
                println!("illegal pid! {}", pid);
                process::exit(-2);
            }
        }
    }
}
