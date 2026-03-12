# Postmortem: net.talkbank.org Interrupted Ubuntu Upgrade (2026-03-12)

**Date of incident:** 2026-03-12
**Duration:** ~30 minutes
**Impact:** Brief confusion over Apache removal; no service outage
**Severity:** Low — all services remained functional

## Summary

`sudo do-release-upgrade -d` was run on net to upgrade from Ubuntu 24.x to 26.04
(Resolute Raccoon). The upgrade was interrupted by an error involving the `apache2`
package. Apache was then purged (`apt purge apache2`), which triggered a warning
about a non-empty `/usr/lib/cgi-bin/` directory containing `morgra2jpg.cgi` and
`morgra2jpg-orig.cgi`.

Investigation confirmed that **Apache was not used on this server** — all CGI
execution goes through nginx + fcgiwrap (FastCGI). The CGI scripts were unaffected
by the Apache removal. fcgiwrap, nginx, and all CGI scripts remained functional.

## Root Cause

The `apache2` package was installed on net as a historical leftover (possibly from
initial setup or a dependency). It was not actively used — nginx has been the web
server, with fcgiwrap handling CGI. The upgrade surfaced this vestigial package as
a conflict.

## Actions Taken

1. Purged `apache2` — safe because nginx + fcgiwrap handle everything
2. Verified fcgiwrap running with socket at `/var/run/fcgiwrap.socket`
3. Verified `morgra2jpg.cgi` and `morgra2jpg-orig.cgi` intact in `/usr/lib/cgi-bin/`
4. Verified nginx running (4 workers)
5. Fixed Tailscale apt source: pointed to `resolute` codename (was `questing`)
6. Cleaned up Tailscale upgrade leftovers (`.migrate`, `.disabled`, `.distUpgrade`)
7. Ran `dpkg --configure -a` and `apt --fix-broken install` to complete interrupted upgrade
8. Created `docs/net-talkbank-server.md` — comprehensive server reference with OS upgrade checklist

## Lessons Learned

1. **Know your service stack before upgrading.** The Apache removal panic was unnecessary
   — had we known Apache wasn't used, the purge would have been a non-event.

2. **`/usr/lib/cgi-bin/` is shared.** Both Apache and fcgiwrap use this directory.
   Purging Apache warns about non-empty dirs but does NOT delete the CGI scripts
   themselves. Still, it's alarming if you don't expect it.

3. **Tailscale apt sources need manual fixup after every Ubuntu upgrade.** The
   `do-release-upgrade` tool renames third-party sources to `.disabled` and leaves
   `.migrate` / `.distUpgrade` files. These must be cleaned up and the codename
   updated.

4. **Document server state proactively.** This incident prompted creation of
   `docs/net-talkbank-server.md` with a full service inventory and OS upgrade
   checklist. This should prevent similar confusion in the future.
