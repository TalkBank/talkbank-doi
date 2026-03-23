# homebank (now talkbank.org) Disk Expansion — 2017

**Status:** Historical
**Last updated:** 2026-03-23 14:35 EDT

## Context

This is a command log from **2017-08-14** documenting the disk expansion on the
CMU Campus Cloud VM then known as **homebank**, which was later renamed/reprovisioned
as **talkbank.org**.

The expansion went from **40 GB to 120 GB** using the fdisk + LVM method. The machine
is still at 120 GB today (2026-03-23) and needs a further expansion to 250+ GB for the
GitLab-to-GitHub data repo migration. See `docs/talkbank-org-server.md` section
"Disk Resize" for the current (simpler, `parted`-based) procedure.

**Differences from current procedure:**
- Used interactive `fdisk` (delete/recreate partition) instead of `parted resizepart`
- Required a reboot to pick up the new partition table
- Volume group was named `git-vg` (now `ubuntu-vg` after OS reprovisioning)

## Original Command Log

```
#command history for expanding diskspace on HOMEBANK

#checking updated mem/cpu
root@homebank:~# lscpu
Architecture:          x86_64
CPU op-mode(s):        32-bit, 64-bit
Byte Order:            Little Endian
CPU(s):                4
On-line CPU(s) list:   0-3
Thread(s) per core:    1
Core(s) per socket:    1
Socket(s):             4
NUMA node(s):          1
Vendor ID:             GenuineIntel
CPU family:            6
Model:                 45
Model name:            Intel(R) Xeon(R) CPU E5-2640 0 @ 2.50GHz
Stepping:              2
CPU MHz:               2500.000
BogoMIPS:              5000.00
Hypervisor vendor:     VMware
Virtualization type:   full
L1d cache:             32K
L1i cache:             32K
L2 cache:              256K
L3 cache:              15360K
NUMA node0 CPU(s):     0-3
Flags:                 fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts mmx fxsr sse sse2 ss syscall nx rdtscp lm constant_tsc arch_perfmon pebs bts nopl xtopology tsc_reliable nonstop_tsc aperfmperf pni pclmulqdq ssse3 cx16 pcid sse4_1 sse4_2 x2apic popcnt tsc_deadline_timer aes xsave avx hypervisor lahf_lm epb tsc_adjust dtherm ida arat pln pts
root@homebank:~# free -m
              total        used        free      shared  buff/cache   available
Mem:           7966         858        6649          29         458        6802
Swap:          2043           0        2043


#show current state of system storage and logical volumes. Making sure sda now shows 120GB
root@homebank:~# lsblk
NAME               MAJ:MIN RM  SIZE RO TYPE MOUNTPOINT
sda                  8:0    0  120G  0 disk
|-sda1               8:1    0  487M  0 part /boot
|-sda2               8:2    0    1K  0 part
`-sda5               8:5    0 39.5G  0 part
  |-git--vg-root   252:0    0 37.5G  0 lvm  /
  `-git--vg-swap_1 252:1    0    2G  0 lvm  [SWAP]
sr0                 11:0    1 1024M  0 rom

root@homebank:~# pvs
  PV         VG     Fmt  Attr PSize  PFree
  /dev/sda5  git-vg lvm2 a--  39.52g 36.00m

root@homebank:~# vgs
  VG     #PV #LV #SN Attr   VSize  VFree
  git-vg   1   2   0 wz--n- 39.52g 36.00m

root@homebank:~# lvs
  LV     VG     Attr       LSize  Pool Origin Data%  Meta%  Move Log Cpy%Sync Convert
  root   git-vg -wi-ao---- 37.49g
  swap_1 git-vg -wi-ao----  2.00g


#resize partition to reflect size of now larger underlying disk
root@homebank:~# fdisk /dev/sda

Welcome to fdisk (util-linux 2.27.1).
Changes will remain in memory only, until you decide to write them.
Be careful before using the write command.


Command (m for help): p
Disk /dev/sda: 120 GiB, 128849018880 bytes, 251658240 sectors
Units: sectors of 1 * 512 = 512 bytes
Sector size (logical/physical): 512 bytes / 512 bytes
I/O size (minimum/optimal): 512 bytes / 512 bytes
Disklabel type: dos
Disk identifier: 0xc385326e

Device     Boot   Start      End  Sectors  Size Id Type
/dev/sda1  *       2048   999423   997376  487M 83 Linux
/dev/sda2       1001470 83884031 82882562 39.5G  5 Extended
/dev/sda5       1001472 83884031 82882560 39.5G 8e Linux LVM

Command (m for help): d
Partition number (1,2,5, default 5): 2

Partition 2 has been deleted.

Command (m for help): n
Partition type
   p   primary (1 primary, 0 extended, 3 free)
   e   extended (container for logical partitions)
Select (default p): e
Partition number (2-4, default 2):
First sector (999424-251658239, default 999424): 1001470
Last sector, +sectors or +size{K,M,G,T,P} (1001470-251658239, default 251658239):

Created a new partition 2 of type 'Extended' and of size 119.5 GiB.

Command (m for help): n
All space for primary partitions is in use.
Adding logical partition 5
First sector (1003518-251658239, default 1003520):
Last sector, +sectors or +size{K,M,G,T,P} (1003520-251658239, default 251658239):

Created a new partition 5 of type 'Linux' and of size 119.5 GiB.

Command (m for help): p
Disk /dev/sda: 120 GiB, 128849018880 bytes, 251658240 sectors
Units: sectors of 1 * 512 = 512 bytes
Sector size (logical/physical): 512 bytes / 512 bytes
I/O size (minimum/optimal): 512 bytes / 512 bytes
Disklabel type: dos
Disk identifier: 0xc385326e

Device     Boot   Start       End   Sectors   Size Id Type
/dev/sda1  *       2048    999423    997376   487M 83 Linux
/dev/sda2       1001470 251658239 250656770 119.5G  5 Extended
/dev/sda5       1003520 251658239 250654720 119.5G 83 Linux

Command (m for help): t
Partition number (1,2,5, default 5):
Partition type (type L to list all types): 8e

Changed type of partition 'Linux' to 'Linux LVM'.

Command (m for help): p
Disk /dev/sda: 120 GiB, 128849018880 bytes, 251658240 sectors
Units: sectors of 1 * 512 = 512 bytes
Sector size (logical/physical): 512 bytes / 512 bytes
I/O size (minimum/optimal): 512 bytes / 512 bytes
Disklabel type: dos
Disk identifier: 0xc385326e

Device     Boot   Start       End   Sectors   Size Id Type
/dev/sda1  *       2048    999423    997376   487M 83 Linux
/dev/sda2       1001470 251658239 250656770 119.5G  5 Extended
/dev/sda5       1003520 251658239 250654720 119.5G 8e Linux LVM

Command (m for help): x

Expert command (m for help): b
Partition number (1,2,5, default 5):
New beginning of data (1001471-251658239, default 1003520): 1001472

Expert command (m for help): p

Disk /dev/sda: 120 GiB, 128849018880 bytes, 251658240 sectors
Units: sectors of 1 * 512 = 512 bytes
Sector size (logical/physical): 512 bytes / 512 bytes
I/O size (minimum/optimal): 512 bytes / 512 bytes
Disklabel type: dos
Disk identifier: 0xc385326e

Device     Boot   Start       End   Sectors Id Type      Start-C/H/S End-C/H/S Attrs
/dev/sda1  *       2048    999423    997376 83 Linux         0/33/32  62/55/53    80
/dev/sda2       1001470 251658239 250656770  5 Extended     62/23/86  305/15/0
/dev/sda5       1001472 251658239 250656768 8e Linux LVM   62/57/118  305/15/0

Expert command (m for help): r

Command (m for help): w
The partition table has been altered.
Calling ioctl() to re-read partition table.
Re-reading the partition table failed.: Device or resource busy

The kernel still uses the old table. The new table will be used at the next reboot or after you run partprobe(8) or kpartx(8).

#reboot to pick up new partition table
root@homebank:~# reboot

#look at current state of storage and logical volumes
root@homebank:~# lsblk
NAME               MAJ:MIN RM   SIZE RO TYPE MOUNTPOINT
sda                  8:0    0   120G  0 disk
|-sda1               8:1    0   487M  0 part /boot
|-sda2               8:2    0     1K  0 part
`-sda5               8:5    0 119.5G  0 part
  |-git--vg-root   252:0    0  37.5G  0 lvm  /
  `-git--vg-swap_1 252:1    0     2G  0 lvm  [SWAP]
sr0                 11:0    1  1024M  0 rom
root@homebank:~# pvs
  PV         VG     Fmt  Attr PSize  PFree
  /dev/sda5  git-vg lvm2 a--  39.52g 36.00m

#resize logical volumes
root@homebank:~# pvresize /dev/sda5
  Physical volume "/dev/sda5" changed
  1 physical volume(s) resized / 0 physical volume(s) not resized
root@homebank:~# pvs
  PV         VG     Fmt  Attr PSize   PFree
  /dev/sda5  git-vg lvm2 a--  119.52g 80.04g

#show current size of filesystem
root@homebank:~# df -h /
Filesystem                Size  Used Avail Use% Mounted on
/dev/mapper/git--vg-root   37G   22G   14G  63% /

#show current logical volume size
root@homebank:~# lvs
  LV     VG     Attr       LSize  Pool Origin Data%  Meta%  Move Log Cpy%Sync Convert
  root   git-vg -wi-ao---- 37.49g
  swap_1 git-vg -wi-ao----  2.00g

#extend logical volume
root@homebank:~# lvextend --extents +100%FREE /dev/git-vg/root
  Size of logical volume git-vg/root changed from 37.49 GiB (9597 extents) to 117.52 GiB (30086 extents).
  Logical volume root successfully resized.
root@homebank:~# lvs
  LV     VG     Attr       LSize   Pool Origin Data%  Meta%  Move Log Cpy%Sync Convert
  root   git-vg -wi-ao---- 117.52g
  swap_1 git-vg -wi-ao----   2.00g

#expand filesystem
root@homebank:~# resize2fs /dev/git-vg/root
resize2fs 1.42.13 (17-May-2015)
Filesystem at /dev/git-vg/root is mounted on /; on-line resizing required
old_desc_blocks = 3, new_desc_blocks = 8
The filesystem on /dev/git-vg/root is now 30808064 (4k) blocks long.

#verify updates to cpu/mem/disk
root@homebank:~# date;lscpu;free -m;df -h /
Mon Aug 14 12:51:38 EDT 2017
Architecture:          x86_64
CPU op-mode(s):        32-bit, 64-bit
Byte Order:            Little Endian
CPU(s):                4
On-line CPU(s) list:   0-3
Thread(s) per core:    1
Core(s) per socket:    1
Socket(s):             4
NUMA node(s):          1
Vendor ID:             GenuineIntel
CPU family:            6
Model:                 45
Model name:            Intel(R) Xeon(R) CPU E5-2640 0 @ 2.50GHz
Stepping:              2
CPU MHz:               2500.000
BogoMIPS:              5000.00
Hypervisor vendor:     VMware
Virtualization type:   full
L1d cache:             32K
L1i cache:             32K
L2 cache:              256K
L3 cache:              15360K
NUMA node0 CPU(s):     0-3
Flags:                 fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts mmx fxsr sse sse2 ss syscall nx rdtscp lm constant_tsc arch_perfmon pebs bts nopl xtopology tsc_reliable nonstop_tsc aperfmperf pni pclmulqdq ssse3 cx16 pcid sse4_1 sse4_2 x2apic popcnt tsc_deadline_timer aes xsave avx hypervisor lahf_lm epb tsc_adjust dtherm ida arat pln pts
              total        used        free      shared  buff/cache   available
Mem:           7966         909        5741          29        1315        6703
Swap:          2043           0        2043
Filesystem                Size  Used Avail Use% Mounted on
/dev/mapper/git--vg-root  116G   22G   89G  20% /
```
