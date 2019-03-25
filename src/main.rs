mod pid;
mod util;

use crate::pid::{PidInfo, PidStat};

fn main() {
    let mut pinfo = PidInfo::new();

    let path = pinfo.path("cmdline");
    pinfo.cmdline = util::read_file_null_separated(path);

    let path = pinfo.path("cwd");
    pinfo.cwd = util::read_link(path);

    let path = pinfo.path("exe");
    pinfo.exe = util::read_link(path);

    let path = pinfo.path("limits");
    pinfo.limits = util::read_file(path);

    let path = pinfo.path("fd");
    pinfo.fds = util::dir_entries(path);

    let path = pinfo.path("task");
    pinfo.tasks = util::dir_entries(path);

    let path = pinfo.path("stat");
    let stat = util::read_file(path);
    pinfo.stat = PidStat::parse(stat);

    println!();
    println!("pid        : {}", pinfo.pid);
    println!("cmdline    : {}", pinfo.cmdline);
    println!("exe        : {}", pinfo.exe);
    println!("cwd        : {}", pinfo.cwd);
    println!("mem (kb)   : {}", pinfo.stat.rss);
    println!("threads    : {}", pinfo.tasks);
    println!("open files : {}", pinfo.fds);

    println!();
    println!("{}", pinfo.limits);
}
