---
name: creating-pull-requests-en
description: Use when a pull request aimed at an English-speaking audience is about to be created or updated — an international OSS repository, an English-operated company repo, or any repo whose review culture runs in English. Trigger at the moment the decision to open or revise a PR is made, well before running any gh command. This covers user requests like "write a PR", "open a PR", "update the PR description", "rewrite the PR", pushing a branch that needs a PR, splitting work into its own PR, or syncing a stale PR body with the current branch state. If the target repository's community works in Japanese, use creating-pull-requests-ja instead.
---

# Creating & updating pull requests (English)

A PR description is a reading guide for the reviewer and the permanent record of the change. Target: within the first screenful — before the diff is ever opened — the reviewer knows what changed, why, and which file to read first.

Practices adapted from Google's eng-practices CL-description guidance (CC-BY 3.0) and general OSS review culture; method-level inspiration from tdhopper's creating-pull-requests skill. See PROVENANCE.md.

## Hard rules

ALWAYS:

- Read the repository's `CONTRIBUTING.md` and `.github/PULL_REQUEST_TEMPLATE*` before writing anything. **A repo template beats this skill's template.** Fill the repo's template faithfully; apply this skill to the writing quality inside it.
- When the PR works around an upstream bug: link the upstream issue/PR, say plainly that the change is temporary, and name the removal condition ("drop this module once upstream X ships").
- Open PRs as drafts (`--draft`) unless the user says otherwise.

NEVER:

- Begin a sentence with "This PR introduces/adds/implements…", "In this pull request…", or "This change…". Lead with the problem, the component, or the action itself.
- Narrate the diff back in prose.

Attribution lines (Co-Authored-By, "Generated with Claude Code"): follow the user's standing practice and the repo's culture. Keep it if the user normally keeps it; drop it if the repo forbids it. Ask only when genuinely ambiguous.

## Prose baseline (always apply)

- **Active voice, present tense.** "The pool caches DNS lookups", not "DNS lookups are cached by the pool."
- **Cut filler.** "in order to", "the fact that", soft hedges ("quite", "fairly", "somewhat").
- **Load-bearing word first.** Put the word that matters at the front of every header and paragraph.
- **Numbers beat adjectives.** "startup fell from 2.1 s to 300 ms" lands; "much faster startup" doesn't.
- **Paragraphs of a few lines.** Walls of text get skimmed past.
- **Bold at most one phrase per bullet.**
- **No "In conclusion" / "Overall" / "In summary".** End on a fact or a next step.

## Size gate — fix the budget before drafting

The most consequential rule. An over-dressed small PR is exactly what teaches reviewers to skip machine-written descriptions. Measure `git diff --stat` and lock the section budget before any text exists.

- **Small (< 50 changed lines, one concern):** TL;DR + Links, nothing more. A needed reviewer note becomes the TL;DR's final sentence.
- **Medium (50–200 lines):** TL;DR + files table + at most two sections that pull their weight. How must capture the design across files, never a file-by-file walk.
- **Large (200+ lines or several concerns):** every applicable section; files table and Reviewer notes required.

**Budget check:** on small and medium PRs, the body text (files table excluded) must come out shorter than the diff. Longer means the diff is being re-described.

## Title

Pattern: `<Verb> <what> [in/for/to <context>]` — present tense, whole scope in one line.

| Good | Bad |
|---|---|
| Cache DNS lookups in the connection pool | Caching improvements |
| Reject empty payloads before queue insert | Fixed a bug |
| Move retry backoff config to per-endpoint settings | Update config.rs |

Match the local convention: check `git log --oneline -20`. Under Conventional Commits, prefix accordingly: `fix(pool): cache DNS lookups across reconnects`.

**Noun pile-ups — two consecutive nouns at most.** Three or more force the reader to backtrack and re-parse; restructure with a preposition or a verb. Say the title out loud — if it doesn't survive speech, rewrite it.

| Bad | Good |
|---|---|
| Improve upload queue retry state metrics naming | Rename metrics for the upload queue's retry state |

## First screenful

Everything above the first scroll must each work standalone:

1. **Title** → the whole scope in one line.
2. **TL;DR** → problem plus response in two sentences, anchored to a concrete number, error, or example. On its own, enough to approve a low-risk PR.
3. **Files table** → reading order, one reason per file, a *(start here)* marker.

A TL;DR that resists two clean sentences means the PR isn't understood yet. Reread the diff first.

## Body template (when the repo has no template)

Keep a section only when dropping it would cost the reviewer time.

```
## TL;DR

[Two sentences anchored to something concrete — a number, an error, an
example. Sentence one: the problem. Sentence two: what this PR does.]

**Files to review (N, +X / -Y):**

| File | Why |
|---|---|
| `path/to/start_here.py` *(start here)* | The natural entry point, in one line. |
| `path/to/other.py` | Why this file is part of the change. |

## Why

[The motivation: error output, wrong behavior, missing capability.
Before/after table when the difference is numeric or visual.
Omit when the TL;DR already carries it.]

## How

[The design, top-down — numbered when sequential, bulleted when parallel.
Decisions and tradeoffs, never a line-by-line tour.]

## Reviewer notes

- **One bolded headline per non-obvious fact,** then the explanation.
- **Focus area:** [where a second opinion is wanted].

## Tests

[What is covered, what is not, and how to run them.]

## Links

- Upstream issue / ticket / related PR
```

## Say what the diff cannot

Motivation, tradeoffs, and context outside the code are the description's territory. The mechanical what belongs to the diff.

Delete on sight:

- **Per-file walkthroughs.** The files table plus the diff already carry this.
- **Build narration.** "First a helper, then wired it into…" — present the design, not the work session.
- **Background the reviewer has.** When a ticket covers it, link it and add one sentence.
- **Prose restating a visible type/signature change.** Give the why, not the what.
- **Apologetic hedging.** "Rough first pass, open to suggestions" — turn it into a concrete question under Reviewer notes.
- **Commit-by-commit history.** The final state is what gets reviewed.

**Sentence filter:** if the diff alone teaches it, the sentence goes.

## Avoiding AI tells

One pattern is survivable; a cluster is what gets a description skipped. Avoid:

| Machine-flavored | Human-flavored |
|---|---|
| This PR implements caching for… | DNS lookups now cache inside the pool, so reconnects skip… |
| This change addresses an issue where… | Uploads stalled once the queue passed 10k entries; the drain loop… |

- **Specificity everywhere,** not just the TL;DR: "rejects payloads over 32 MiB with 413" rather than "adds validation".
- **Break structural monotony.** A run of identically shaped bold-headline bullets reads as generated text.
- **Self-sufficient context.** Trackers get migrated; git history stays. Inline the essentials, link for depth.
- **6-month test:** a stranger arriving from `git log` half a year later should still understand why. If not, add the missing context.

## Visual aids — illustrate, don't decorate

- **Before/after table** when observable behavior changes (output, API shape, metrics, errors).
- **Mermaid diagram** (< 15 nodes) when data flow or component wiring changes. Diagram only what *changed*.
- **Code snippet** when a public API surface changes and the reviewer needs the new call site.
- **Screenshots / terminal output** for UI, CLI, or log-format changes.
- **`<details>/<summary>`** for supporting evidence (benchmarks, tracebacks, large configs). The PR must read complete without expanding anything. Blank line after `<summary>` and before `</details>`.
- **GFM alerts** (`> [!IMPORTANT]`, `> [!WARNING]`) for breaking changes and must-not-miss facts.

Skip visual aids for internal refactors, renames, and test-only changes.

## External contribution to an unfamiliar OSS repo

Extra steps when opening a PR against a repo you don't maintain:

1. Read `CONTRIBUTING.md`, the PR template, and 3–5 recently merged PRs to learn the local register, section names, and title convention. Mirror them.
2. Search closed issues/PRs for the same problem; link what you find. If a related issue was closed as "upstream problem", frame the PR as an interim mitigation, not a reopening of the debate.
3. Make the change minimal and easy to revert; offer an opt-out for behavior changes when cheap (env var, flag).
4. State verification concretely: which checks pass, and what end-to-end behavior you confirmed on which platform.
5. Don't take automated bot comments ("author not in allowed list", lint bots) personally; address only what blocks CI.

## Pre-submit checklist

- [ ] Repo template respected (if one exists).
- [ ] Size gate → section count matches diff size.
- [ ] Title → whole scope, active voice, ≤ 2 consecutive nouns, local prefix convention.
- [ ] TL;DR → problem + response, concrete number or example.
- [ ] No sentence starts with "This PR" / "This change".
- [ ] Every sentence teaches something the diff can't.
- [ ] Files table marks *(start here)* (medium+).
- [ ] Focus area stated if specific feedback is wanted.
- [ ] 6-month test passes.

## Process

1. **Detect create vs update:** `gh pr view --json number,title,body,baseRefName,url 2>/dev/null`
2. **Gather context:**
   ```
   BASE=$(gh pr view --json baseRefName -q '.baseRefName' 2>/dev/null || echo "main")
   git diff $BASE...HEAD --stat
   git diff $BASE...HEAD
   git log $BASE..HEAD --oneline
   ```
   Read the actual diff, not just the stat.
3. **Find links** before asking the user: `git log --all --oneline --grep="keyword"`, issue tracker, branch-name ticket numbers. Preserve every existing link when updating.
4. **Classify and draft.** Size gate first, TL;DR sketched first, then only the sections the budget allows.
5. **Post-generation review.** Reread the diff; walk the pre-submit checklist sentence by sentence.
6. **Apply.** Write the body to a temp file and use `--body-file` (never inline `--body` or HEREDOC):
   ```
   gh pr create --draft --title "..." --body-file /tmp/pr-body.md
   gh pr edit <number> --title "..." --body-file /tmp/pr-body.md
   ```

## Updating an existing PR

Rewrite the body to describe the branch's **present state against base**, as though the PR were brand new. "Also adds", "additionally", "now includes" mark a changelog, not a description — remove them.
