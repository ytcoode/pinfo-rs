# pinfo
Display process information on Linux

## Usage

```
pinfo <pid>
```

## Example

```
$ sudo ./pinfo 1

pid        : 1
cmdline    : /sbin/init
exe        : /usr/lib/systemd/systemd
cwd        : /
mem (kb)   : 9916
threads    : 1
open files : 71

Limit                     Soft Limit           Hard Limit           Units
Max cpu time              unlimited            unlimited            seconds
Max file size             unlimited            unlimited            bytes
Max data size             unlimited            unlimited            bytes
Max stack size            8388608              unlimited            bytes
Max core file size        unlimited            unlimited            bytes
Max resident set          unlimited            unlimited            bytes
Max processes             63082                63082                processes
Max open files            1073741816           1073741816           files
Max locked memory         67108864             67108864             bytes
Max address space         unlimited            unlimited            bytes
Max file locks            unlimited            unlimited            locks
Max pending signals       63082                63082                signals
Max msgqueue size         819200               819200               bytes
Max nice priority         0                    0
Max realtime priority     0                    0
Max realtime timeout      unlimited            unlimited            us
```
