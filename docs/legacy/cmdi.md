# CMDI metadata

Currently, Leonid handles generation of CMDI files periodically (but
not upon every push). All the steps are done manually, as detailed
below by Leonid.

## Instructions

In Unix Terminal `cd` to every server's repo and type:

```
git status
git pull
```

It would be best to run CHATTER on `~/data/` folder at this time too

In CLAN's `Commands` window:

Set working directory to `~/data/`

Run command to check for duplicate PIDs run command:


```
cmdi +p *.cha
```

If errors found, then fix them and run above command again
If no duplicate PIDs found, then run command:

```
cmdi *.cha
```

If languages are missing, then run the following command to get files list:

```
cmdi -l *.cha

```

After there are no more errors reported by `cmdi` do the following:

Validation of XML is part of this process, and uses Franklin's own
homegrown XML validator
https://github.com/FranklinChen/validate-xml-rust for efficiency.
TODO Brian runs this? Anyone can run this.

1.  Run Franklin's XML validator on `data-cmdi` folder by using this command
    ```
    validate-xml ~/data-cmdi 2> log.txt
    ```
2.  Check for errors in the log.txt file by using this command:
    ```
    grep -v validates log.txt
    ```
3.  If there are errors found by XML validator, then fix them
4.  In CLAN's `Commands` window run `cmdi *.cha` again
5.  Repeat from step 1.  Continue to the next step only if XML
    validator validates all .cmdi files without any errors.

Now create CMDI scripts and add PIDs to database data files:

In CLAN's `Commands` window run command:

```
cmdi +c *.cha
```

In Unix Terminal type:

run CHATTER on `~/data/` folder

If CHATTER finds error, then fix them anyway you can including deleting repos and starting from scratch

Proceed only if CHATTER does not find any errors

```
cd <to every server's repo>
git status
git commit -a
git push
```

AT THIS POINT TELL LEONID TO FINISH TRANSFER DATA TO DALI

Transfer CMDI files inside `data-cmdi` folder to `dali.talkbank.org`
server to folder `/var/www/web/data-cmdi/`

Adding PIDs to Handle Server

Transfer file `~/data/0PID_batch.txt` to `Mac Zee` server to folder
`/WORK/CLAN-data/Handle/hs`

After above files are transferred to `dali.talkbank.org` server and
Mac Zee they can be deleted.

Connect to `dali.talkbank.org` with command `ssh macw@dali.talkbank.org`

In Unix Terminal on `dali.talkbank.org` type:

```
/var/www/hs/stop.sh
cd /var/www/hs/svr_1
```

First backup and then delete folders `bdbje`, `txns` and file `txn_id`
to `/var/www/hs/bck`

```
/var/www/hs/start.sh&
```

Transfer file `/var/www/hs/svr_1/admpriv.bin` from `dali.talkbank.org`
to `Mac Zee` folder `~/Downloads/admpriv.bin`

In Unix Terminal on `Mac Zee` type:

```
cd /WORK/CLAN-data/Handle/hs/bin/
./hdl-admintool
```

Choose the menu option `Tools->Home/Unhome Prefix`

In the `Prefix` box enter `0.NA/11312`

Under `By Site Info File (siteinfo.bin)` click `Choose File...`

Select `siteinfo.json` file from directory:
`/WORK/CLAN-data/Handle/hs/siteinfo.json`

Click `Do It` button, in next window click `OK` button, Then enter password `???`

Still inside `hdl-admintool` create new PIDs:

Choose the menu option `Tools->Batch Processor`

In `Batch Processor` window click `Add` button and select file `0PID_batch.txt`

Click on `Run Batch(es)` button and hopefully it all works....


## Automation

Is there a way to integrate this into the continuous integration
and deployment system?

This system spans three different computers. It is possible to do this
from one computer.

The process requires users interaction to detect and correct errors,
so this can not be fully automated.

In one case I have to use a GUI to kill Handle server and then to rebuild it.

I don't know if that can be done through terminal command

## Providing to OAI

Leonid manually copies over CMDI files to the deployment server and
runs a script Franklin provides in order to update a `watch`
directory. That automatically triggers updates to the [OAI
provider](oai.md).

TODO Ideally, this step would be automated somehow and put into the
continuous integration and continuous deployment system. For example,
the CMDI files could be in Git and Leonid could push there.
Pushing Git is easy, but who is going to pull Git on dali server?
