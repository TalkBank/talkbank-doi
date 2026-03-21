# Firewall Exception Request: net → media server

**Status:** In progress (waiting on CMU IT)
**Last updated:** 2026-03-20

## CMU Help Ticket

**Subject:** Firewall rule request — allow SSH from net (172.24.73.5) to talkbank-02 (128.237.92.37)

---

**Requesting team:** TalkBank, Department of Psychology (PI: Brian MacWhinney)

### What we need

A single firewall rule allowing one CMU server to reach another CMU server
via SSH, without requiring Cisco AnyConnect VPN:

| Field | Value |
|-------|-------|
| Source | `net.lan.local.cmu.edu` — 172.24.73.5 (Baker/Porter, private subnet) |
| Destination | `talkbank-02.andrew.cmu.edu` — 128.237.92.37 (Campus Cloud Plus) |
| Port | TCP 22 (SSH/SFTP only) |
| Direction | Outbound from source to destination |
| Authentication | SSH key, service account `psych-tb-svc` (already configured and working) |

This is a single source IP to a single destination IP on a single port.
No public or external access is being requested — both machines are
CMU-operated and CMU-networked.

The requested rule is identical in form to two existing rules on
talkbank-02's firewall:

```
permit tcp object GIT.TALKBANK.ORG object TALKBANK-02.ANDREW.CMU.EDU eq ssh
permit tcp object SLA2.TALKBANK.ORG object TALKBANK-02.ANDREW.CMU.EDU eq ssh
```

We are requesting:

```
permit tcp object NET.LAN.LOCAL.CMU.EDU object TALKBANK-02.ANDREW.CMU.EDU eq ssh
```

### Why we need this

We periodically sync ~12 TB of research media files (audio, video) from
net to talkbank-02 (which serves `media.talkbank.org`) using `rclone`
over SFTP, across 18 corpus banks. Today, every sync requires a team
member to manually log into Cisco AnyConnect on net with CMU credentials
and Duo 2FA, wait for the VPN to connect, run the sync, and then
disconnect. This cannot be scripted or automated.

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

---

## Ticket History

### 2026-03-17 — Initial submission

Submitted the firewall rule request above.

### 2026-03-18 — First reply (forwarded to Campus Cloud Plus group)

CMU IT forwarded to the Campus Cloud Plus group. Noted that VPN access is
the "intended way" of accessing Campus Cloud Plus VMs. Shared the current
inbound firewall rules for talkbank-02:

```
permit tcp object VPN-SII-CCP object TALKBANK-02.ANDREW.CMU.EDU eq ssh
permit tcp any4 object TALKBANK-02.ANDREW.CMU.EDU object-group HTTPS_HTTP
permit tcp any4 object TALKBANK-02.ANDREW.CMU.EDU eq 3000
permit tcp object GIT.TALKBANK.ORG object TALKBANK-02.ANDREW.CMU.EDU eq ssh
permit tcp object SLA2.TALKBANK.ORG object TALKBANK-02.ANDREW.CMU.EDU eq ssh
permit tcp any4 object TALKBANK-02.ANDREW.CMU.EDU eq 7000
permit tcp any4 object TALKBANK-02.ANDREW.CMU.EDU eq 9000
```

Noted that `git.talkbank.org` and `sla2.talkbank.org` are on the
non-computing services colo in the datacenter, and that
`net.lan.local.cmu.edu` is on the Baker/Porter building networks,
private subnet.

### 2026-03-18 — Second reply (three options proposed)

A different responder proposed three approaches:

1. **Relay through git.talkbank.org** — sync files to git, trigger a
   runner to push to talkbank-02. Connection already permitted.
2. **Relay through sla2.talkbank.org** — sync to sla2, schedule rclone
   to talkbank-02. Connection already permitted.
3. **Allow CMU subnets (not public)** to SSH to talkbank-02.

Options 1 and 2 are impractical: double-copying ~12 TB of media files
through an intermediary machine. We rejected these.

### 2026-03-19 — Our reply

Explained that relay options make no sense for 12 TB of locally-stored
media. Noted that git.talkbank.org is being decommissioned. Clarified
the scope: ~12 TB across internal and external drives on net.

### 2026-03-19 — Third reply (Dan Fassinger)

Dan confirmed option 3 is the path forward. Asked for source subnet(s)
and IP ranges. Said to expect a reply when the change is ready.

### Next step

Reply to Dan with the specific source IP (172.24.73.5), emphasizing this
is a single-IP rule, not a subnet range, and reference the existing
git/sla2 rules as the exact pattern to follow.
