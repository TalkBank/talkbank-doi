# Building Phon and PhonTalk Locally

## Prerequisites

- Java 24+ (Temurin recommended): `java -version`
- Maven 3.x (for Phon): `mvn -version`
- Gradle 9.3+ (PhonTalk includes wrapper): `./gradlew --version`
- GitHub token with `read:packages` scope for GitHub Packages

## Credentials

GitHub Packages credentials are in `~/.m2/settings.xml` (Maven) for Phon.
PhonTalk uses Gradle and reads from environment variables or
`~/.gradle/gradle.properties`:

```bash
export GITHUB_ACTOR=TalkBank
export GITHUB_TOKEN=<token from ~/.m2/settings.xml>
```

Or in `~/.gradle/gradle.properties`:
```
gpr.user=TalkBank
gpr.key=<token>
```

## Building Phon (`~/phon`)

Phon is a Maven multi-module project. PhonTalk depends on `ca.phon:phon-project`
and transitive modules.

```bash
cd ~/phon

# Install parent POM first
mvn install -DskipTests -N

# Install the modules PhonTalk needs (skip aligned-types-db — needs ca.hedlund:tst)
mvn install -DskipTests \
  -pl language,fsa,xml,ipa,orthography,session,session-xml-2_0,session-xml-2_1,core,project
```

### Known Issues

- `ca.hedlund:tst:jar:26` is not published to any accessible Maven repo.
  Modules `aligned-types-db`, `app`, `components`, `ipadictionary` depend on it.
  Skip them with `-pl` to build only what PhonTalk needs.
- Phon HEAD is `4.0.0-SNAPSHOT` but PhonTalk may reference a specific alpha
  version (e.g., `4.0.0-alpha.59`). Update PhonTalk's
  `gradle/libs.versions.toml` to match.
- Maven artifact IDs use `phon-` prefix (e.g., `phon-project`) but PhonTalk's
  `libs.versions.toml` may reference shortened names (e.g., `ca.phon:project`).
  The published GitHub Packages also use `phon-project`. Adjust the module
  coordinates in `libs.versions.toml` if needed.
- Latest published Phon on GitHub Packages: `4.0.0-alpha.47` (as of 2026-03-10).
  PhonTalk may be ahead of this.

## Building PhonTalk (`~/phontalk`)

```bash
cd ~/phontalk

# With credentials as env vars:
GITHUB_ACTOR=TalkBank GITHUB_TOKEN=<token> ./gradlew build

# Or just compile core (skip tests):
GITHUB_ACTOR=TalkBank GITHUB_TOKEN=<token> ./gradlew :phontalk-core:compileJava
```

### PhonTalk Version Coupling

The project version encodes both Phon and Chatter versions:
`4.0.0-3.2.4-SNAPSHOT` = Phon `4.0.0` + Chatter `3.2.4`.

Key dependency versions in `gradle/libs.versions.toml`:
- `phon` — must match installed Phon version
- `talkbank-schema` — TalkBank XML schema version
- `chatter` — chatter.jar version (used for CHAT↔XML conversion)

### Using Local Phon Build

If building Phon from source, add `mavenLocal()` to PhonTalk's
`settings.gradle.kts` repository list (before `mavenCentral()`):

```kotlin
repositories {
    mavenLocal()  // ← add this
    mavenCentral()
    // ... other repos
}
```

## Architecture

```
Phon XML  →  Phon Session (Java objects)  →  TalkBank XML  →  CHAT text
             (Phon's SessionReader)          (TalkbankWriter)   (chatter.jar)
```

The bug we found (2026-03-10): `TalkbankWriter` writes `%mod`/`%pho` through
`OneToOne` alignment (truncates to orthography word count), but writes
`%xmodsyl`/`%xphosyl`/`%xphoaln` from raw `IPATranscript` (all IPA words).
See `TalkbankWriter.java` lines 604–654 vs line 346.
