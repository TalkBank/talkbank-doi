# Batchalign Plugin System — Archival Reference

> **Status:** Removed in March 2026. The 4 HK providers were folded into
> `batchalign3` as built-in engines with optional extras. This document
> preserves the full design for reference if extensibility is ever needed again.

## Why It Was Removed

- Exactly one plugin ever existed (batchalign-hk-plugin), written by the same team.
- No external users wrote plugins or asked for the capability.
- The plugin system added indirection (entry-point discovery, lazy loading wrappers,
  `PluginDescriptor`/`InferenceProvider` dataclasses, probe tuples, version strings)
  for what was really just "optional inference engines with heavy dependencies."
- Folding the 4 HK engines into batchalign3 as optional extras (`pip install
  "batchalign3[tencent]"`) achieves the same dependency isolation without the
  abstraction cost.
- For experimental staging, a branch is simpler than a plugin package.

## Architecture

### Entry Point Discovery

Plugins registered via `pyproject.toml` entry points:

```toml
[project.entry-points."batchalign.plugins"]
hk = "batchalign_hk_plugin:plugin"
```

Discovery at runtime via `importlib.metadata.entry_points(group="batchalign.plugins")`.
Each entry point resolved to a `PluginDescriptor` instance.

### Core Types (`batchalign/providers/__init__.py`)

```python
@dataclass(frozen=True)
class InferenceProvider:
    """One named provider implementation for one inference task."""
    task: InferTaskName          # "asr", "fa", "morphosyntax", etc.
    name: ProviderName           # "tencent", "wav2vec_canto", etc.
    load: ProviderLoadFn         # Called once at worker startup
    infer: ProviderInferFn       # Called per batch_infer request
    probe: tuple[ModuleProbe, ...] = ()  # Packages that must be importable
    version: str = "unknown"     # Cache-busting version string

@dataclass
class PluginDescriptor:
    """Everything a provider plugin contributes to batchalign."""
    providers: list[InferenceProvider] = field(default_factory=list)
    cmd2task: dict[CommandName, TaskSpec] = field(default_factory=dict)
    command_probes: dict[CommandName, tuple[ModuleProbe, ...]] = field(default_factory=dict)
    command_base_mb: dict[CommandName, int] = field(default_factory=dict)
```

Protocol types for the function signatures:

```python
class ProviderLoadFn(Protocol):
    def __call__(self, lang: str, engine_overrides: EngineOverrides | None) -> None: ...

class ProviderInferFn(Protocol):
    def __call__(self, req: BatchInferRequest) -> BatchInferResponse: ...
```

### Discovery (`batchalign/plugins.py`, 75 lines)

```python
_ENTRY_POINT_GROUP = "batchalign.plugins"
_plugin_cache: list[PluginDescriptor] | None = None

def discover_plugins() -> list[PluginDescriptor]:
    """Load all installed plugin descriptors. Cached after first call."""
    global _plugin_cache
    if _plugin_cache is not None:
        return _plugin_cache
    descriptors = []
    for ep in importlib.metadata.entry_points(group=_ENTRY_POINT_GROUP):
        try:
            obj = ep.load()
            if isinstance(obj, PluginDescriptor):
                descriptors.append(obj)
            else:
                _L.warning("Not a PluginDescriptor — skipping")
        except Exception:
            _L.warning("Failed to load plugin — skipping", exc_info=True)
    _plugin_cache = descriptors
    return descriptors
```

### Worker Integration

**Loading** (`batchalign/worker/_main.py`):

`_load_plugin_providers()` was called at the end of `_load_models()`. It:
1. Filtered `engine_overrides` to valid `InferTaskName` keys
2. Called `discover_plugins()` to get all installed descriptors
3. Built a `(task, name) → InferenceProvider` lookup
4. For each override matching a plugin provider: checked probes, called `load()`,
   stored the `infer` handler in `_state.plugin_handlers`

**Dispatch** (`batchalign/worker/_infer.py`):

Plugin handlers were checked first, before the built-in dispatch table:

```python
def _batch_infer(req: BatchInferRequest) -> BatchInferResponse:
    # Check plugin handlers first (selected via --engine-overrides)
    if _state.plugin_handlers and req.task in _state.plugin_handlers:
        return _state.plugin_handlers[req.task](req)
    # ... built-in dispatch table ...
```

**Runtime registration** (`batchalign/runtime.py`):

`_ensure_plugin_runtime()` merged plugin metadata into module-level registries:
- `Cmd2Task` — command-to-task mapping
- `COMMAND_PROBES` — import probes for capability detection
- `COMMAND_BASE_MB` — memory budgets

Core built-ins won on key collisions (`setdefault`).

### Lazy Loading Pattern (HK Plugin)

Heavy SDK imports were deferred via a closure wrapper:

```python
def _lazy(module: str, attr: str) -> Callable[..., object]:
    _fn = None
    def wrapper(*args, **kwargs):
        nonlocal _fn
        if _fn is None:
            _fn = getattr(importlib.import_module(module), attr)
        return _fn(*args, **kwargs)
    return wrapper

plugin = PluginDescriptor(providers=[
    InferenceProvider(
        task="asr", name="tencent",
        load=_lazy("batchalign_hk_plugin.tencent_asr", "load_tencent_asr"),
        infer=_lazy("batchalign_hk_plugin.tencent_asr", "infer_tencent_asr"),
        probe=("tencentcloud",),
        version="tencent-asr-v1",
    ),
    # ... 3 more providers ...
])
```

## Files Removed

| File | Lines | Purpose |
|------|-------|---------|
| `batchalign/plugins.py` | 77 | Entry-point discovery + caching |
| `batchalign/providers/__init__.py` | 95 | `InferenceProvider`, `PluginDescriptor`, protocols |
| `batchalign/providers/models.py` | 47 | Re-exported inference models for plugin convenience |
| `batchalign/tests/test_plugins.py` | 226 | Discovery, caching, error handling tests |

Code removed from existing files:
- `batchalign/worker/_main.py`: `_load_plugin_providers()` (~90 lines)
- `batchalign/worker/_infer.py`: plugin handler check (~2 lines)
- `batchalign/worker/_types.py`: `plugin_handlers`, `plugin_versions` fields
- `batchalign/runtime.py`: `_ensure_plugin_runtime()` (~25 lines)

## The HK Plugin That Was Folded In

### Providers

| Engine | Task | Type | Dependencies |
|--------|------|------|-------------|
| `tencent` | ASR | Cloud (Tencent Cloud) | `tencentcloud-sdk-python-{common,asr}`, `cos-python-sdk-v5`, `opencc-python-reimplemented` |
| `aliyun` | ASR | Cloud (Alibaba NLS) | `aliyun-python-sdk-core`, `alibabacloud-nls-python-sdk`, `opencc-python-reimplemented` |
| `funaudio` | ASR | Local (FunASR) | `funasr`, `opencc-python-reimplemented` |
| `wav2vec_canto` | FA | Local (Wave2Vec + pycantonese) | `pycantonese` |

### Cantonese Text Pipeline

All ASR engines apply shared normalization:
1. OpenCC `s2hk` conversion (Simplified → HK Traditional)
2. Manual replacement table (`_CANTONESE_REPLACEMENTS`)
3. Punctuation stripping

### Credential Configuration

Cloud engines read from `~/.batchalign.ini`:

```ini
[asr]
engine.tencent.id = <secret_id>
engine.tencent.key = <secret_key>
engine.tencent.region = ap-guangzhou
engine.tencent.bucket = <bucket_name>

engine.aliyun.ak_id = <access_key_id>
engine.aliyun.ak_secret = <access_key_secret>
engine.aliyun.ak_appkey = <app_key>
```

### Source File Inventory (pre-folding)

| File | Lines | Purpose |
|------|-------|---------|
| `__init__.py` | 67 | Plugin descriptor + lazy wrappers |
| `common.py` | 164 | Cantonese normalization, config, language codes |
| `_asr_types.py` | 48 | Internal TypedDicts |
| `tencent_asr.py` | 139 | Tencent provider load/infer |
| `tencent_api.py` | 204 | TencentRecognizer: COS upload + ASR polling |
| `aliyun_asr.py` | 365 | Aliyun WebSocket provider + token caching |
| `funaudio_asr.py` | 160 | FunASR provider load/infer |
| `funaudio_common.py` | 183 | FunAudioRecognizer + segment parsing |
| `cantonese_fa.py` | 272 | Cantonese FA: hanzi→jyutping + Wave2Vec |
| **Total** | **1,602** | |

Tests: 2,131 lines across 10 files (unit + integration + compat).

## How to Restore

If extensibility is ever needed again:

1. Restore `batchalign/plugins.py` and `batchalign/providers/` from git history
   (commit before removal).
2. Re-add `_load_plugin_providers()` to `_main.py` and the plugin handler check
   to `_infer.py`.
3. Re-add `_ensure_plugin_runtime()` to `runtime.py`.
4. Re-add `plugin_handlers`/`plugin_versions` fields to `_WorkerState`.
5. The entry-point contract is standard Python (`importlib.metadata`), so any
   package declaring `[project.entry-points."batchalign.plugins"]` will be
   discovered automatically.

Estimated effort: ~30 minutes to restore from git, ~2 hours to re-test.

---
Archived: 2026-03-09
