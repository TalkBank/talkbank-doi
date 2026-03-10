# Chatter

## Purpose

Chatter parses a ChAT file and generates validated XML (conforming to
the TalkBank XML Schema), or failing that, generates useful error
messages.

There is a rudimentary app that does caching as well as generates
`0errors.cex` files in every folder recursively.

## Testing Chatter

`testchat` is used to make sure Chatter does the right thing.

TODO add details

## History and internals

TODO Explain legacy decisions given development started in 2002.

### Information from [CLAN info](clan-info.md)

TODO There is no automatic triggering of rebuilding Chatter based on
changes to `clan-info` files such as `ISO-639.cut` and
`depfile.cut`. Franklin currently periodically reruns a bunch of
scripts and manually copies and pastes material into Chatter source
code.

### Lexing

### Parsing to CHAT AST

TODO

JavaCC. ANTLR 2.

We use an old version of ANTLR (ANTLR 3).

### Transforming CHAT AST to StringTemplate

TODO

We use the ANTLR 3 feature of tree walking (this feature was removed
in ANTLR 4) to generate `StringTemplate` material for both XML and for
CHAT.

### Transforming CHAT XML to CHAT

TODO

Handwritten translator from CHAT XML, using JAXB, to CHAT AST, and
then using the tree walker to generate CHAT.

## Building Chatter

TODO Building Chatter currently is not fully automated. Maven is used,
but also there are order dependencies. Also, there are Perl scripts
that are used to generate material that is manually copied and pasted
into sections of various source files.

TODO sort out the Perl and shell scripts, e.g.

add-bar-to-ids.pl                  fix-tab-space.pl
alternate.pl                       fix-xxx-pho.pl
analyze-nonviable.pl               generate-ca-delimiters.pl
change-replacement-groups.pl       generate-ca-elements.pl
change-replacement-order.pl        generate-dependent.pl
chat-fold-diff.pl                  generate-diffs.pl
chat-fold-in-place.pl              generate-flex-from-cut.pl
chat-fold.pl                       generate-form-markers.pl
clean-participants.pl              generate-ipa.pl
convert-lang-codes.pl              generate-languages.pl
convert-media-header.pl            generate-sterminators.pl
convert-media.pl                   generate-terminators.pl
convert-quotations.pl              generate-three-lang-chas.pl
find-all-chat-begin.pl             generate-token-creators.pl
find-amp-equals-colon.pl           generate-valid-number-languages.pl
find-bad-ca.pl                     get-participant-roles.pl
find-event-scope.pl                get-pho-chars.pl
find-mor-star-space.pl             harvest-chat-bookmarks.pl
find-mor-star.pl                   harvest-langs.pl
find-mor-zero.pl                   insert-media-unlinked.pl
find-multiple-bullets.pl           make-zip.pl
find-multiple-form-markers.pl      parse-media-header.pl
find-old-ca.pl                     remove-diff-xml.pl
find-quotation.pl                  remove-mor-star.pl
find-replacement.pl                rid-utf8.pl
find-test.pl                       strip-bullets.pl
find-trn-glottal-words.pl          to-ses.pl
fix-media-names.pl

chat-viewer.sh            deploy-xsddoc.sh          test-jip.sh
chat2chat-file-debug.sh   dt2chat-file.sh           test-roundtrip.sh
chat2chat-file.sh         dt2chat.sh                test-xml2chat.sh
chat2chat.sh              dt2xml-file.sh            validate-test.sh
chat2xml-file-debug.sh    dt2xml.sh                 validate-xml.sh
chat2xml-file-filter.sh   find-media-chat.sh        xml2chat-debug.sh
chat2xml-file.sh          generate-XmlAntlrUtils.sh xml2chat-file-debug.sh
chat2xml-fr.sh            generate-letters.sh       xml2chat-file-filter.sh
chat2xml-java5.sh         generate-xsddoc.sh        xml2chat-file.sh
chat2xml-jip.sh           make-dist.sh              xml2chat-heapdump.sh
chat2xml-local.sh         run-test.sh               xml2chat-local.sh
chat2xml.sh               sendToCLAN.sh             xml2chat.sh
chat2xml64.sh             test-jip-junk.sh          xml2xml-file.sh

### TODO To fix

#### Corpus names in ID fields

TODO Corpus names in ID fields should be consistent, e.g. the
following should be invalid.

``` text
@ID:	eng|EllisWeismer|CHI|2;06.|female|LT||Target_Child||ec|
@ID:	eng|EllisWeismer30ec|INV|||||Investigator|||
```

#### TODO Handling of Unicode characters

Need to be careful about using `char` because that is a UTF-16 code
unit, not actually a Unicode "character"!

Use `StandardCharsets.UTF_8`.

#### TODO File handling

`java.nio.file.Path`

`try` with closeable.

#### TODO New Java features

lambdas

streams

switch statements

Objects.requireNonNull etc.

#### TODO Run static checkers

null checking

https://errorprone.info/

https://www.archunit.org/

https://github.com/google/auto

etc.

#### TODO Remove sources of Java deprecation

Date
GregorianCalendar

#### TODO Make comprehensive JavaDoc documentation

#### TODO Stop building such a fat JAR

#### TODO Use GraalVM to build native app?

#### macOS app

In the past, a macOS app was provided that enabled drag and drop and
other macOS-native features.

TODO Use Java 14's [`jpackage`](https://openjdk.java.net/jeps/343)?

TODO The `send2clan` feature allowing triple-clicking on an error line
to launch CLAN stopped working reliably at some point. Is it still
broken?

https://www.mojohaus.org/maven-native/native-maven-plugin/
https://github.com/huangxuwei/jni-maven-example

#### Windows app

TODO I have not tested Chatter on Windows for a long time. All I
remember is that the `send2clan` functionality broke many years ago
and we never got around to trying to fix it.


## Not yet implemented

TODO Phon syllabification support not yet implemented.

## Release as open source?

TODO The build is not yet fully automated. Maven is used, but there
are also many, many scripts to regenerate source code when making
changes to Chatter.

## Rewrite?

TODO Possibly rewrite in TypeScript or some other language that can
easily target the Web, and for more accessibility to those wanting to
embed Chatter as an API.

TODO Maintain reasonable performance. Chatter uses hacks in an attempt
to be fast.
