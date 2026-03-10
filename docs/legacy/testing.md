# Testing

## Automated testing

We have slowly been migrating from manual to [automated
testing](checkers.md) when possible, but still have a long way to go,
because other things have taken priority.

## Manual testing

We still do a tremendous amount of manual testing.

For example, we don't yet have automated testing for

- [TalkBank browser](talkbank-browser.md), e.g., to make sure it works
  properly when things are changed. [Cypress](https://www.cypress.io/)
  would probably serve very well for that.
- Password protection: to make sure that protected parts of Web sites
  are protected properly.
