# Firewall Exception Request: net → media server

**Status:** Draft
**Last updated:** 2026-03-17

## CMU Help Ticket

**Subject:** Firewall rule request — allow SSH from net (172.24.73.5) to talkbank-02 (128.237.92.37)

---

**Requesting team:** TalkBank, Department of Psychology (PI: Brian MacWhinney)

### What we need

A firewall rule allowing one CMU server to reach another CMU server via
SSH, without requiring Cisco AnyConnect VPN:

| Field | Value |
|-------|-------|
| Source | `net.lan.local.cmu.edu` — 172.24.73.5 |
| Destination | `talkbank-02.andrew.cmu.edu` — 128.237.92.37 (Campus Cloud Plus) |
| Port | TCP 22 (SSH/SFTP only) |
| Direction | Outbound from source to destination |
| Authentication | SSH key, service account `psych-tb-svc` (already configured and working) |

This is a single source IP to a single destination IP on a single port.
No public or external access is being requested — both machines are
CMU-operated and CMU-networked.

### Why we need this

We periodically sync research media files from net to talkbank-02 (which
serves `media.talkbank.org`) using `rclone` over SFTP. Today, every sync
requires a team member to manually log into Cisco AnyConnect on net with
CMU credentials and Duo 2FA, wait for the VPN to connect, run the sync,
and then disconnect. This is a significant operational burden — syncs
happen frequently across 19 corpus banks, and each session requires
interactive authentication that cannot be scripted or automated.

A direct firewall rule would eliminate this manual overhead entirely.

### What this firewall rule would replace

The persistent AnyConnect VPN session. With this rule, net could reach
talkbank-02 directly for SFTP file sync, and we would no longer need to
run VPN on this server at all. Everything else about our setup (SSH keys,
the `psych-tb-svc` account, the rclone-based sync tool) is already in
place and working — the VPN tunnel is the only reason the connection
exists.

### Contact

[name/email/phone]
