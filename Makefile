REPOS = tree-sitter-talkbank talkbank-chat talkbank-chatter talkbank-clan batchalign3

.PHONY: status check test verify-all clone pull

# Git status across all repos
status:
	@for repo in $(REPOS) talkbank-private; do \
		echo "=== $$repo ==="; \
		git -C $$repo status --short --branch; \
		echo ""; \
	done

# Cargo check all Rust repos
check:
	cd talkbank-chat && cargo check --all-targets
	cd talkbank-chat/spec/tools && cargo check --all-targets
	cd talkbank-chatter && cargo check --all-targets
	cd talkbank-clan && cargo check

# Run tests across repos
test:
	cd talkbank-chat && cargo test -p talkbank-model -p talkbank-tree-sitter-parser
	cd talkbank-chatter && cargo test
	cd talkbank-clan && cargo test
	cd tree-sitter-talkbank && tree-sitter test

# Full cross-repo verification gate
verify-all:
	cd talkbank-chat && make verify
	cd talkbank-chatter && cargo fmt --all -- --check
	cd talkbank-chatter && cargo clippy --all-targets -- -D warnings
	cd talkbank-chatter && cargo test --all-targets
	cd talkbank-clan && cargo fmt --all -- --check
	cd talkbank-clan && cargo clippy --all-targets -- -D warnings
	cd talkbank-clan && cargo test --all-targets
	cd tree-sitter-talkbank && npx tree-sitter generate
	cd tree-sitter-talkbank && npx tree-sitter test
	cd tree-sitter-talkbank && for q in queries/*.scm; do npx tree-sitter query "$$q" test/corpus/main_tier/simple_utterance.txt >/dev/null; done
	cd batchalign3 && uv run mypy batchalign/cli/ batchalign/pipelines/ batchalign/serve/
	cd batchalign3 && cargo check --manifest-path rust/Cargo.toml --all-targets

# Clone all repos fresh (for new machines)
clone:
	gh repo clone TalkBank/tree-sitter-talkbank
	gh repo clone TalkBank/talkbank-chat
	gh repo clone TalkBank/talkbank-chatter
	gh repo clone TalkBank/talkbank-clan
	gh repo clone TalkBank/batchalign3
	gh repo clone TalkBank/talkbank-private

# Pull all repos
pull:
	@for repo in $(REPOS) talkbank-private; do \
		echo "==> Pulling $$repo"; \
		git -C $$repo pull --rebase 2>/dev/null || echo "    (no remote or not on tracking branch)"; \
	done
