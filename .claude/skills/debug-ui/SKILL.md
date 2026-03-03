---
name: debug-ui
description: Cross-cutting UI debugging across all 5 TalkBank UI systems (VS Code webviews, TUI, React dashboard, Tauri desktop, axum static serving). Symptom-based triage for any UI problem.
disable-model-invocation: true
allowed-tools: Bash, Read, Glob, Grep, Agent
---

# UI Debugging — Cross-System Triage

Diagnose UI problems across any TalkBank interface. `$ARGUMENTS` describes the symptom. This skill routes to the right subsystem and provides targeted diagnostic steps.

## Step 1: Identify Which UI System

| If the problem is in... | System | Repo-specific skill |
|------------------------|--------|---------------------|
| VS Code (analysis panel, graph, media, waveform) | VS Code Webview | `/ui-webview` in talkbank-chatter |
| Terminal (chatter validate --interactive) | ratatui TUI | `/ui-tui` in talkbank-chatter |
| Browser dashboard (http://host:8000/dashboard) | React SPA | `/ui-dashboard` in batchalign3 |
| Desktop app (Tauri window) | Tauri + React | `/ui-dashboard` in batchalign3 |
| API returning wrong HTML/assets | axum static serving | Check `dashboard.rs` in batchalign-server |

## Step 2: Symptom-Based Triage

### "Blank screen / nothing renders"

**VS Code webview:**
- Is `enableScripts: true` set? (required for any JS)
- Check Content Security Policy `<meta>` tag
- Right-click webview → "Developer: Open Webview Developer Tools" → Console for JS errors
- Check if LSP returned data: Output panel → "TalkBank Language Server"

**React dashboard:**
- Check if `frontend/dist/` exists: `ls ~/.batchalign3/dashboard/index.html`
- Check `BATCHALIGN_DASHBOARD_DIR` env var
- Check browser DevTools console for JS errors
- Check network tab — are assets (JS/CSS) loading?
- Try: `curl -s http://localhost:8000/dashboard | head -5` (should return HTML, not 404)

**TUI:**
- Is terminal alternate screen active? Try `reset` to restore
- Check terminal size: ratatui needs minimum dimensions
- Check TERM env var: `echo $TERM` (needs xterm-256color or similar)

**Tauri:**
- Is Vite dev server running? (`npm run dev` in frontend/)
- Check Tauri window console: Cmd+Opt+I (macOS DevTools)

---

### "Data not updating / stale data"

**VS Code webview:**
- Webview gets data at creation time — no auto-refresh
- Save the document to trigger re-validation by LSP
- Close and reopen the panel to force refresh

**React dashboard:**
- Check WebSocket connection status in Zustand store
- Check React Query devtools: `npm run dev` includes devtools by default
- Force invalidation: browser console `queryClient.invalidateQueries()`
- Check if server is sending WebSocket updates: `wscat -c ws://localhost:8000/ws`

**TUI:**
- Streaming mode auto-updates; static mode is snapshot-only
- Press `r` to rerun validation

---

### "Layout broken / elements overlapping"

**VS Code webview:**
- Check viewport meta tag: `<meta name="viewport" content="width=device-width, initial-scale=1.0">`
- Check CSS for absolute positioning without containment
- Test in both narrow and wide panel widths

**React dashboard:**
- Check Tailwind responsive breakpoints (`sm:`, `md:`, `lg:`)
- Check `overflow-auto` on scrollable containers
- Test at different browser window sizes

**TUI:**
- Terminal too small? ratatui constraints may underflow
- Use `Constraint::Min()` for flexible regions
- Check for hardcoded `Length()` values that don't fit

---

### "Wrong colors / unreadable text"

**VS Code webview:**
- Using hardcoded colors instead of `var(--vscode-*)` CSS variables?
- Test with both dark and light VS Code themes
- Check: `var(--vscode-foreground)`, `var(--vscode-editor-background)`

**React dashboard:**
- Tailwind opacity classes (e.g., `text-white/80`) may be too faint
- Check contrast ratios for accessibility
- CSS custom properties in `app.css` may override Tailwind defaults

**TUI:**
- Terminal color support varies (8, 256, truecolor)
- Check theme: `~/.config/chatter/theme.toml`
- Test with `--theme light` on light terminal backgrounds
- Check `$COLORTERM` env var (should be `truecolor` for RGB colors)

---

### "Keyboard/mouse not working"

**VS Code webview:**
- Webview captures focus — extension keyboard shortcuts may not work inside webview
- Use `retainContextWhenHidden: true` if panel should keep state when tabbed away

**TUI:**
- Check key bindings in `validation_tui/mod.rs`
- Raw mode must be enabled (`enable_raw_mode()`)
- Mouse events not implemented — keyboard only
- Modifier keys: `KeyModifiers::CONTROL`, `KeyModifiers::SHIFT`

**React dashboard:**
- Check event handlers attached to correct elements
- Check for `preventDefault()` or `stopPropagation()` blocking events

---

### "Performance / lag"

**VS Code webview:**
- Large DOT graphs cause Graphviz WASM lag → consider limiting graph size
- Waveform canvas redraws on every scroll → throttle with `requestAnimationFrame`
- Full HTML regeneration on every update → consider incremental DOM updates

**React dashboard:**
- Large job lists without virtualization → add `react-window` or `react-virtualized`
- React re-renders cascade → check React Compiler auto-memoization is working
- WebSocket message flood → batch state updates with `requestAnimationFrame`

**TUI:**
- `poll(Duration::from_millis(100))` throttles to 10 FPS — increase interval if still slow
- Avoid blocking operations in the render loop
- Text processing (Unicode width) on large outputs can be slow

---

### "Media won't play" (VS Code only)

- Check `@Media:` header exists in .cha file
- Check file exists at resolved path (media resolver tries multiple locations)
- Check `localResourceRoots` includes the media file's directory
- Check `asWebviewUri()` conversion is applied to the path
- Check CSP allows media: `media-src ${webview.cspSource}`
- Check audio format is browser-supported (WAV, MP3, OGG — not all codecs)

---

### "Build fails"

**VS Code extension:**
```bash
cd /Users/chen/talkbank/talkbank-chatter/vscode && npm run compile 2>&1 | head -20
```

**React dashboard:**
```bash
cd /Users/chen/talkbank/batchalign3/frontend && npm run build 2>&1 | head -20
```

**TUI (Rust):**
```bash
cd /Users/chen/talkbank/talkbank-chatter && cargo check -p talkbank-cli 2>&1 | head -20
```

**Tauri:**
```bash
cd /Users/chen/talkbank/batchalign3/apps/dashboard-desktop && npm run build 2>&1 | head -20
```

## Step 3: Cross-System Concerns

### API Contract Between Frontend and Backend

The React dashboard depends on the Rust server's API shape. After server changes:
```bash
cd /Users/chen/talkbank/batchalign3/frontend
npm run generate:schema   # Regenerate TypeScript types
npm run check:api          # CI gate: detect drift
```

### Dashboard Asset Deployment

Production flow: `npm run build` → `dist/` → copy to server machine's `~/.batchalign3/dashboard/`

The Rust server discovers dashboard at (in order):
1. `$BATCHALIGN_DASHBOARD_DIR` env var
2. `~/.batchalign3/dashboard/`

### Theme Consistency

All UIs should respect user's color preferences:
- VS Code: `var(--vscode-*)` CSS variables
- TUI: `~/.config/chatter/theme.toml`
- React: Tailwind dark mode (always dark currently)
- Tauri: Inherits React theme
