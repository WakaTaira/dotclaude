---
name: grill-me
description: Interview the user relentlessly about a plan or design until reaching shared understanding, resolving each branch of the decision tree — while capturing the domain model as you go (glossary terms into CONTEXT.md, hard-to-reverse decisions into ADRs). Use when user wants to stress-test a plan, get grilled on their design, or mentions "grill me" / "詰めて" / "問い詰めて".
origin: community
source: https://github.com/mattpocock/skills/tree/main/skills/engineering/grill-with-docs
---

# grill-me (docs-enabled)

Two disciplines run together in this session: a relentless interview, and active domain modeling. The interview sharpens the plan; the modeling writes down what crystallises, so decisions outlive the conversation. (Upstream, this is the composition of `grilling` + `domain-modeling`; here it is flattened into one skill.)

## The interview

Interview me relentlessly about every aspect of this plan until we reach a shared understanding. Walk down each branch of the design tree, resolving dependencies between decisions one-by-one. For each question, provide your recommended answer.

Ask the questions one at a time, waiting for feedback on each question before continuing. Asking multiple questions at once is bewildering.

If a *fact* can be found by exploring the codebase, look it up rather than asking me. The *decisions*, though, are mine — put each one to me and wait for my answer.

Do not enact the plan until I confirm we have reached a shared understanding.

## Domain modeling as we go

Actively build and sharpen the project's domain model while interviewing — challenging terms, inventing edge-case scenarios, and writing the glossary and decisions down the moment they crystallise.

### File structure

Most repos have a single context: one `CONTEXT.md` at the repo root (glossary), plus `docs/adr/` for architectural decision records (`0001-slug.md`, `0002-slug.md`, …). If a `CONTEXT-MAP.md` exists at the root, the repo has multiple contexts and the map points to where each `CONTEXT.md` lives.

Create files lazily — only when you have something to write. If no `CONTEXT.md` exists, create one when the first term is resolved. If no `docs/adr/` exists, create it when the first ADR is needed.

### Challenge against the glossary

When the user uses a term that conflicts with the existing language in `CONTEXT.md`, call it out immediately. "Your glossary defines 'cancellation' as X, but you seem to mean Y — which is it?"

### Sharpen fuzzy language

When the user uses vague or overloaded terms, propose a precise canonical term. "You're saying 'account' — do you mean the Customer or the User? Those are different things."

### Discuss concrete scenarios

When domain relationships are being discussed, stress-test them with specific scenarios. Invent scenarios that probe edge cases and force the user to be precise about the boundaries between concepts.

### Cross-reference with code

When the user states how something works, check whether the code agrees. If you find a contradiction, surface it: "Your code cancels entire Orders, but you just said partial cancellation is possible — which is right?"

### Update CONTEXT.md inline

When a term is resolved, update `CONTEXT.md` right there. Don't batch these up — capture them as they happen. Use the format in [CONTEXT-FORMAT.md](./CONTEXT-FORMAT.md).

`CONTEXT.md` should be totally devoid of implementation details. Do not treat `CONTEXT.md` as a spec, a scratch pad, or a repository for implementation decisions. It is a glossary and nothing else.

### Offer ADRs sparingly

Only offer to create an ADR when all three are true:

1. **Hard to reverse** — the cost of changing your mind later is meaningful
2. **Surprising without context** — a future reader will wonder "why did they do it this way?"
3. **The result of a real trade-off** — there were genuine alternatives and you picked one for specific reasons

If any of the three is missing, skip the ADR. Use the format in [ADR-FORMAT.md](./ADR-FORMAT.md).

### Language of the docs

Write `CONTEXT.md` and ADRs in the language the repo's documentation already uses; for repos with no existing documentation, default to Japanese.
