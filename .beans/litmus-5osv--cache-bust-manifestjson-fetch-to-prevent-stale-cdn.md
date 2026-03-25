---
# litmus-5osv
title: Cache-bust manifest.json fetch to prevent stale CDN cache
status: todo
type: task
created_at: 2026-03-25T15:35:21Z
updated_at: 2026-03-25T15:35:21Z
---

The browser cached an old CDN manifest.json, causing screenshots for newer fixtures to appear missing. The manifest URL has no cache-busting — switching from CDN to local dev still serves the cached CDN response.

## Design

Add a hash or timestamp query parameter to the manifest.json fetch URL to prevent browser caching. Options:
- Use a build-time hash (e.g. from git rev or compile timestamp) appended as `?v=<hash>`
- Or set `cache: 'no-cache'` on the fetch request in the JS code

The simplest fix is adding `cache: 'no-cache'` to the fetch options in main.rs's tryFetch function, since the manifest itself is small and should always be fresh.

## Tasks
- [ ] Add cache-busting to manifest.json fetch in main.rs (tryFetch)
- [ ] Verify dev and production both get fresh manifests
- [ ] Test that screenshot images still load (they already have `?v=<checksum>` via cache_busted_url)
