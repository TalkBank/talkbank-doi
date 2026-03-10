# Resize Linux virtual filesystem

```
macw@git:~$ echo 1 | sudo tee /sys/class/block/sda/device/rescan
macw@git:~$ lsblk
NAME                     MAJ:MIN RM   SIZE RO TYPE MOUNTPOINTS
loop0                      7:0    0   104M  1 loop /snap/core/16928
loop1                      7:1    0 104.2M  1 loop /snap/core/17200
loop2                      7:2    0  55.4M  1 loop /snap/core18/2846
loop3                      7:3    0  55.4M  1 loop /snap/core18/2855
loop4                      7:4    0  63.7M  1 loop /snap/core20/2496
loop5                      7:5    0  63.8M  1 loop /snap/core20/2501
loop6                      7:6    0  73.9M  1 loop /snap/core22/1802
loop7                      7:7    0  66.2M  1 loop /snap/core24/739
loop8                      7:8    0  73.9M  1 loop /snap/core22/1908
loop9                      7:9    0  66.8M  1 loop /snap/core24/888
loop10                     7:10   0 104.1M  1 loop /snap/lxd/29943
loop11                     7:11   0 104.1M  1 loop /snap/lxd/30130
loop12                     7:12   0  44.4M  1 loop /snap/snapd/23771
loop13                     7:13   0  44.4M  1 loop /snap/snapd/23545
sda                        8:0    0   500G  0 disk
├─sda1                     8:1    0   487M  0 part /boot
├─sda2                     8:2    0     1K  0 part
└─sda5                     8:5    0 199.5G  0 part
  ├─talkbank2--vg-root   252:0    0 197.5G  0 lvm  /
  └─talkbank2--vg-swap_1 252:1    0     2G  0 lvm  [SWAP]
sr0                       11:0    1  1024M  0 rom
macw@git:~$ sudo parted /dev/sda
GNU Parted 3.6
Using /dev/sda
Welcome to GNU Parted! Type 'help' to view a list of commands.
(parted) print
Model: VMware Virtual disk (scsi)
Disk /dev/sda: 537GB
Sector size (logical/physical): 512B/512B
Partition Table: msdos
Disk Flags:

Number  Start   End    Size   Type      File system  Flags
 1      1049kB  512MB  511MB  primary   ext2         boot
 2      512MB   215GB  214GB  extended
 5      513MB   215GB  214GB  logical                lvm

(parted) resizepart 2 100%
(parted) resizepart 5 100%
(parted) print
Model: VMware Virtual disk (scsi)
Disk /dev/sda: 537GB
Sector size (logical/physical): 512B/512B
Partition Table: msdos
Disk Flags:

Number  Start   End    Size   Type      File system  Flags
 1      1049kB  512MB  511MB  primary   ext2         boot
 2      512MB   537GB  536GB  extended
 5      513MB   537GB  536GB  logical                lvm

(parted) quit
Information: You may need to update /etc/fstab.
macw@git:~$ sudo pvresize /dev/sda5
macw@git:~$ sudo resize2fs /dev/talkbank2-vg/root
resize2fs 1.47.2 (1-Jan-2025)
Filesystem at /dev/talkbank2-vg/root is mounted on /; on-line resizing required
old_desc_blocks = 13, new_desc_blocks = 32
The filesystem on /dev/talkbank2-vg/root is now 130422784 (4k) blocks long.

macw@git:~$ df -h ~
Filesystem                      Size  Used Avail Use% Mounted on
/dev/mapper/talkbank2--vg-root  490G  178G  292G  38% /
macw@git:~$ lsblk
NAME                     MAJ:MIN RM   SIZE RO TYPE MOUNTPOINTS
loop0                      7:0    0   104M  1 loop /snap/core/16928
loop1                      7:1    0 104.2M  1 loop /snap/core/17200
loop2                      7:2    0  55.4M  1 loop /snap/core18/2846
loop3                      7:3    0  55.4M  1 loop /snap/core18/2855
loop4                      7:4    0  63.7M  1 loop /snap/core20/2496
loop5                      7:5    0  63.8M  1 loop /snap/core20/2501
loop6                      7:6    0  73.9M  1 loop /snap/core22/1802
loop7                      7:7    0  66.2M  1 loop /snap/core24/739
loop8                      7:8    0  73.9M  1 loop /snap/core22/1908
loop9                      7:9    0  66.8M  1 loop /snap/core24/888
loop10                     7:10   0 104.1M  1 loop /snap/lxd/29943
loop11                     7:11   0 104.1M  1 loop /snap/lxd/30130
loop12                     7:12   0  44.4M  1 loop /snap/snapd/23771
loop13                     7:13   0  44.4M  1 loop /snap/snapd/23545
sda                        8:0    0   500G  0 disk
├─sda1                     8:1    0   487M  0 part /boot
├─sda2                     8:2    0   512B  0 part
└─sda5                     8:5    0 499.5G  0 part
  ├─talkbank2--vg-root   252:0    0 497.5G  0 lvm  /
  └─talkbank2--vg-swap_1 252:1    0     2G  0 lvm  [SWAP]
sr0                       11:0    1  1024M  0 rom
```
