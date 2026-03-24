---
# litmus-52qn
title: Generate and curate first batch of candidate fixtures
status: todo
type: task
priority: normal
created_at: 2026-03-24T14:01:40Z
updated_at: 2026-03-24T14:01:51Z
parent: litmus-49jz
blocked_by:
    - litmus-feex
    - litmus-3lcp
---

Using findings from research + agent generation, create first batch of candidates:

Priority scenarios:
- ripgrep search results (filename, line number, match highlighting)
- bat/cat syntax-highlighted source code
- journalctl/log viewer with severity colors
- neovim/helix editor UI

For each candidate:
- Write setup.sh + command.sh
- Run capture, inspect output
- Evaluate against quality criteria
- Document assessment in REVIEW.md
- Promote good ones, discard or iterate on the rest

Depends on: staging workflow, research
