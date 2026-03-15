# talkbank-dev Makefile — workspace orchestration
# Usage: make clone        (everything)
#        make clone-minimal (just code repos)
#        make clone-data    (corpus data)
#        make status        (git status across all repos)

# ── Repo groups ──────────────────────────────────────────────

# Core development
CORE_REPOS = talkbank-tools batchalign3

# Infrastructure & deployment
INFRA_REPOS = staging webdev gra-cgi sync-media generate-from-chat cdcs-to-csv

# Pre-commit / build tools
TOOL_REPOS = update-chat-types save-word-html-pdf talkbank-xml-schema

# Legacy CLAN
CLAN_REPOS = OSX-CLAN clan-info

# Browser & validation
UTIL_REPOS = talkbank-browser-check

# APIs & libraries
API_REPOS = TBDBr TBDBpy balite

# All code repos (everything except data and web)
CODE_REPOS = $(CORE_REPOS) $(INFRA_REPOS) $(TOOL_REPOS) $(CLAN_REPOS) $(UTIL_REPOS) $(API_REPOS)

# Corpus data (currently on GitLab, migrating to GitHub)
GITLAB_HOST = git@gitlab.talkbank.org:talkbank
DATA_REPOS = aphasia-data asd-data biling-data ca-data childes-data class-data \
             dementia-data fluency-data homebank-data motor-data phon-data \
             psychosis-data rhd-data samtale-data slabank-data tbi-data

# Web bank repos
BANK_REPOS = talkbank-web childes-bank aphasia-bank dementia-bank asd-bank \
             biling-bank ca-bank class-bank fluency-bank homebank-bank \
             motor-bank phon-bank psychosis-bank rhd-bank samtale-bank \
             slabank-bank tbi-bank

# Repos under personal GitHub (not TalkBank org)
PERSONAL_REPOS_CHEN = java-chatter-stable talkbank-ipa-fragments

# Collaborator repos (not TalkBank org)
COLLAB_REPOS = phon phontalk

# ── Phony targets ────────────────────────────────────────────

.PHONY: help status check test verify-all verify-contract-gates \
        verify-coverage-gates verify-release-gates clone clone-minimal \
        clone-code clone-data clone-web clone-collab clone-personal pull adopt

help:
	@echo "Workspace commands:"
	@echo "  make clone          Clone ALL repos (code + data + web + collaborator)"
	@echo "  make clone-minimal  Clone just talkbank-tools + batchalign3"
	@echo "  make clone-code     Clone all code repos (no data, no web)"
	@echo "  make clone-data     Clone corpus data repos from GitLab"
	@echo "  make clone-web      Clone web/bank repos"
	@echo "  make clone-collab   Clone collaborator repos (phon, phontalk)"
	@echo "  make adopt          Move existing ~/repo clones into workspace (interactive)"
	@echo ""
	@echo "Development commands:"
	@echo "  make status         Git status across all repos"
	@echo "  make pull           Pull all repos"
	@echo "  make check          Cargo check all Rust workspaces"
	@echo "  make test           Run tests across repos"
	@echo "  make verify-all     Full cross-repo verification gate"
	@echo "  make verify-contract-gates Focused release-facing contract suites"
	@echo "  make verify-coverage-gates Repo-native coverage entrypoints"
	@echo "  make verify-release-gates Contract + coverage release gates"

# ── Status & pull ────────────────────────────────────────────

status:
	@for repo in $(CODE_REPOS); do \
		if [ -d "$$repo/.git" ]; then \
			echo "=== $$repo ==="; \
			git -C "$$repo" status --short --branch; \
			echo ""; \
		fi; \
	done
	@if [ -d "data" ]; then \
		for repo in data/*/; do \
			if [ -d "$$repo/.git" ]; then \
				echo "=== $$repo ==="; \
				git -C "$$repo" status --short --branch; \
				echo ""; \
			fi; \
		done; \
	fi

pull:
	@for repo in $(CODE_REPOS); do \
		if [ -d "$$repo/.git" ]; then \
			echo "==> Pulling $$repo"; \
			git -C "$$repo" pull --rebase 2>/dev/null || echo "    (no remote or not on tracking branch)"; \
		fi; \
	done
	@if [ -d "data" ]; then \
		for repo in data/*/; do \
			if [ -d "$$repo/.git" ]; then \
				echo "==> Pulling $$repo"; \
				git -C "$$repo" pull --rebase 2>/dev/null || echo "    (no remote or not on tracking branch)"; \
			fi; \
		done; \
	fi

# ── Clone targets ────────────────────────────────────────────

clone: clone-code clone-data clone-web clone-collab clone-personal
	@echo ""
	@echo "All repos cloned. Run 'make status' to verify."

clone-minimal:
	@for repo in $(CORE_REPOS); do \
		if [ ! -d "$$repo" ]; then \
			echo "Cloning $$repo..."; \
			gh repo clone TalkBank/$$repo $$repo; \
		else \
			echo "$$repo already exists, skipping"; \
		fi; \
	done

clone-code: clone-minimal
	@for repo in $(INFRA_REPOS) $(TOOL_REPOS) $(CLAN_REPOS) $(UTIL_REPOS) $(API_REPOS); do \
		if [ ! -d "$$repo" ]; then \
			echo "Cloning $$repo..."; \
			gh repo clone TalkBank/$$repo $$repo; \
		else \
			echo "$$repo already exists, skipping"; \
		fi; \
	done

clone-data:
	@mkdir -p data
	@for repo in $(DATA_REPOS); do \
		if [ ! -d "data/$$repo" ]; then \
			echo "Cloning data/$$repo from GitLab..."; \
			git clone $(GITLAB_HOST)/$$repo.git data/$$repo; \
		else \
			echo "data/$$repo already exists, skipping"; \
		fi; \
	done

clone-web:
	@for repo in $(BANK_REPOS); do \
		if [ ! -d "web/$$repo" ] && [ ! -d "$$repo" ]; then \
			echo "Cloning $$repo..."; \
			gh repo clone TalkBank/$$repo $$repo; \
		else \
			echo "$$repo already exists, skipping"; \
		fi; \
	done

clone-collab:
	@if [ ! -d "phon" ]; then \
		echo "Cloning phon..."; \
		git clone https://github.com/phon-ca/phon.git phon; \
	else \
		echo "phon already exists, skipping"; \
	fi
	@if [ ! -d "phontalk" ]; then \
		echo "Cloning phontalk..."; \
		git clone https://github.com/phon-ca/phontalk.git phontalk; \
	else \
		echo "phontalk already exists, skipping"; \
	fi

clone-personal:
	@if [ ! -d "java-chatter-stable" ]; then \
		echo "Cloning java-chatter-stable..."; \
		git clone git@github.com:FranklinChen/java-chatter.git java-chatter-stable; \
	else \
		echo "java-chatter-stable already exists, skipping"; \
	fi
	@if [ ! -d "talkbank-ipa-fragments" ]; then \
		echo "Cloning talkbank-ipa-fragments..."; \
		git clone git@github.com:FranklinChen/talkbank-ipa-fragments.git talkbank-ipa-fragments; \
	else \
		echo "talkbank-ipa-fragments already exists, skipping"; \
	fi

# ── Adopt existing clones ───────────────────────────────────
# Moves repos from ~/ into this workspace and verifies remotes.

adopt:
	@echo "This will move existing repo clones from ~/ into this workspace."
	@echo "Repos that already exist here will be skipped."
	@echo ""
	@bash scripts/adopt-repos.sh

# ── Build & test ─────────────────────────────────────────────

check:
	cd talkbank-tools && cargo check --workspace --all-targets
	cd talkbank-tools/spec/tools && cargo check --all-targets
	cd batchalign3 && cargo check --manifest-path crates/Cargo.toml --all-targets
	cd batchalign3 && cargo check --manifest-path pyo3/Cargo.toml --all-targets

test:
	cd talkbank-tools && cargo nextest run --workspace
	cd batchalign3 && cargo nextest run --manifest-path crates/Cargo.toml
	cd batchalign3 && cargo nextest run --manifest-path pyo3/Cargo.toml

verify-all:
	cd talkbank-tools && make verify
	cd batchalign3 && cargo fmt --manifest-path crates/Cargo.toml --all -- --check
	cd batchalign3 && cargo clippy --manifest-path crates/Cargo.toml --all-targets -- -D warnings
	cd batchalign3 && cargo nextest run --manifest-path crates/Cargo.toml
	cd batchalign3 && cargo fmt --manifest-path pyo3/Cargo.toml --all -- --check
	cd batchalign3 && cargo clippy --manifest-path pyo3/Cargo.toml --all-targets -- -D warnings
	cd batchalign3 && cargo nextest run --manifest-path pyo3/Cargo.toml

verify-contract-gates:
	@echo "==> talkbank-tools CLI contract suites"
	cd talkbank-tools && cargo nextest run -p talkbank-cli \
		--test command_surface_manifest \
		--test cache_tests \
		--test command_matrix_tests \
		--test legacy_clan_cli_contracts \
		--test stateful_cli_integration
	@echo "==> talkbank-tools VS Code runtime/service contract suites"
	cd talkbank-tools/vscode && npm run compile
	cd talkbank-tools/vscode && npx vitest run \
		src/test/runtimeContext.test.ts \
		src/test/panelAssets.test.ts \
		src/test/effectRuntime.test.ts \
		src/test/effectCommandRuntime.test.ts \
		src/test/activationLsp.test.ts \
		src/test/validationActivation.test.ts \
		src/test/cacheManager.test.ts \
		src/test/clanIntegration.test.ts
	cd talkbank-tools/vscode && npm run lint -- --quiet
	@echo "==> batchalign3 CLI contract suites"
	cd batchalign3 && cargo nextest run -p batchalign-cli \
		--test command_surface_manifest \
		--test compat_contracts \
		--test command_matrix \
		--test cli \
		--test daemon_e2e
	@echo "==> batchalign3 runtime/worker contract suites"
	cd batchalign3 && cargo test -p batchalign-app --test worker_protocol_matrix
	cd batchalign3 && cargo test -p batchalign-app --test worker_protocol_v2_compat
	cd batchalign3 && cargo test -p batchalign-app runtime_layout_ -- --nocapture
	cd batchalign3 && cargo test -p batchalign-app python_runtime_ -- --nocapture
	cd batchalign3 && cargo test -p batchalign-app revai_credential_ -- --nocapture
	cd batchalign3 && cargo test -p batchalign-cli config_port_returns_default -- --nocapture
	cd batchalign3 && cargo test -p batchalign-cli dispatch_no_server -- --nocapture
	cd batchalign3 && cargo test -p batchalign-cli otlp_runtime_ -- --nocapture
	cd batchalign3 && uv run pytest \
		batchalign/tests/test_worker_execute_v2_matrix.py \
		batchalign/tests/test_worker_bootstrap_runtime.py \
		-q

verify-coverage-gates:
	@echo "==> talkbank-tools Rust coverage"
	cd talkbank-tools && cargo llvm-cov nextest --workspace --lcov --output-path lcov.info
	cd talkbank-tools && cargo llvm-cov --doc --lcov --output-path lcov-doc.info
	@echo "==> talkbank-tools VS Code coverage"
	cd talkbank-tools/vscode && npm run test:coverage
	@echo "==> batchalign3 Python and Rust coverage"
	cd batchalign3 && uv run pytest --cov=batchalign --cov-report=lcov:lcov-python.info batchalign --disable-pytest-warnings -k 'not test_whisper_fa_pipeline'
	cd batchalign3 && cargo llvm-cov nextest --manifest-path pyo3/Cargo.toml --lcov --output-path lcov-rust.info
	cd batchalign3 && cargo llvm-cov nextest --workspace --lcov --output-path lcov-rust-workspace.info

verify-release-gates: verify-contract-gates verify-coverage-gates
