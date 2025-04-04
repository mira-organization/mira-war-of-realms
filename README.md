# Mira Turn Base

[![CI / CD](https://github.com/mira-organization/mira-war-of-realms/actions/workflows/ci.yml/badge.svg)](https://github.com/mira-organization/mira-war-of-realms/actions/workflows/ci.yml)
___
**Mira Turn Base - A Strategic Adventure in the World of Elysthiel**

Mira Turn Base is a tactical turn-based strategy game set in the mystical and frostbitten world of Elysthiel.
Inspired by the dark and enigmatic lore of Mira: Sacrifice of Light,
this project offers a deep and engaging combat system, where every decision matters,
and every move could be the difference between victory and defeat.

**Tactical Depth and Strategic Combat**

Mira Turn Base features a rich and complex battle system where players must carefully position their units, 
exploit enemy weaknesses, and make use of unique abilities.
The game emphasizes calculated planning over brute force,
rewarding those who can out think their opponents in a chess-like battlefield.

**The Characters of Elysthiel**

Players will command a variety of characters, each with their own skills, backgrounds, and combat specializations.
Mira, the mysterious elven warrior with flowing lavender hair,
takes center stage, leading a band of outcasts, rebels,
and warriors against the oppressive forces of the Church of Light.

**A World of Mystery and Conflict**

The game delves into the ongoing struggle against the Church of Light,
expanding on the rich lore of Mira: Sacrifice of Light.
Players will uncover secrets, forge alliances, and engage in battles that shape the fate of Elysthiel.
___

# Commit Message Conventions

This project follows a structured commit message format to maintain clarity and consistency in version control.

## Commit Types

### `feat` - New Features
Use `feat` when introducing a new feature to the project.
```
git commit -m "feat(JIRA-ID): Added new combat system for Mira"
```

### `ref` - Refactoring
Use `ref` for code improvements that do not change the logic or behavior.
```
git commit -m "ref(JIRA-ID): Optimized movement calculations"
```

### `ci` - CI/CD Actions
Use `ci` for changes related to continuous integration and deployment.
```
git commit -m "ci(JIRA-ID): Updated GitHub Actions for automated testing"
```

### `test` - Unit Tests
Use `test` when adding or modifying unit tests.
```
git commit -m "test(JIRA-ID): Added unit tests for damage calculation"
```

### `doc` - Documentation
Use `doc` for changes in documentation.
```
git commit -m "doc(JIRA-ID): Updated README with commit conventions"
```

### `chore` - Dependencies & Maintenance
Use `chore` for dependency updates and non-functional maintenance.
```
git commit -m "chore(JIRA-ID): Updated Rust dependencies"
```

### `fix` - Bug Fixes
Use `fix` when resolving a problem in the code.
```
git commit -m "fix(JIRA-ID): Resolved issue with health regeneration"
```

## Using JIRA-ID
Each commit message should include a JIRA-ID in parentheses to link the commit to a specific issue
or task in the JIRA system. This helps track changes and associate them with the relevant work items.

By following this structure, we ensure a clear and maintainable commit history.
___

## Release and Publishing

We create releases using the git tag system. Pushing a new tag activates the release.yml ci.
This creates a new release with the name that you set as follows:
```shell
  git tag -a "mira-wor-0.1.1.dev" -m "test(Test-ID): include new tag..."
```

With the next command you then push the tag to GitHub:
```shell
  git push --tags
```
Please only create releases with permission. This is for clarity. 
Releases with the end name "dev" are meant for staging. 
Ends with "pre" are versions that are used as test releases. 
Without any text at the end it is always a real release!
___


