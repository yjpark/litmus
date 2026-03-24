---
# litmus-sk2k
title: Audit existing fixtures against quality criteria
status: in-progress
type: task
priority: normal
created_at: 2026-03-24T14:01:44Z
updated_at: 2026-03-24T15:46:21Z
parent: litmus-49jz
---

Review the 7 existing fixtures against the quality criteria:

1. Color variety (≥4 distinct ANSI colors)
2. Instant recognition
3. Fits 80x24
4. Deterministic
5. Self-contained

Fix any that don't meet criteria. Known issues:
- htop fallback may not be deterministic (real top output varies)
- Some fixtures rely on tool color defaults which may vary by system
- shell-prompt and python-repl are simulated (printf/echo) — check if they still make sense as fixtures or should be replaced with real tool output

This can run in parallel with new fixture work.
