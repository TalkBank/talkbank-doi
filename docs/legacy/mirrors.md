# Mirrors and other sharing

## `rsync` mirroring
    At one point we supported `rsync` mirroring by setting up an
    `rsync` server that was accessible by only a few fixed IP
    addresses.

    A thorny question was that of how to handle *protected*
    materials. Our protection is based purely on public access to our
    Apache HTTP servers. The use of `rsync` bypasses any such
    protections, such that "mirrors" get everything.
    TODO We never resolved this dilemma.

    TODO `rsync` mirrorring is not currently supported since the migration off
    our custom Mac setup to Linux cloud servers?

    TODO (Provide full details of configuration and the commands for
    the user.)

## Enabling setting up own servers with own data
    There has been talk of enabling others to set up their own servers
    with their own CHAT data, etc.

    This is currently not possible because our huge web of
    configuration and processes is completely hardwired to every
    single aspect of our system.

    TODO We may wish to think about whether we want to put in the work
    to support such an all-in-one configurable platform. I believe
    we do want to eventually provide such a solution, and that it
    would benefit not only others but also us, to have a system that
    is easy to replicate, e.g., using tools such as Docker.
