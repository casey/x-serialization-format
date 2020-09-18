default: watch

version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml | head -1`

bt := '0'

export RUST_BACKTRACE := bt

log := 'warn'

export RUST_LOG := log

# watch filesystem for changes and rerun tests
watch +ARGS='':
	cargo +nightly watch --clear --exec 'test --all {{ARGS}}'

push:
	! git branch | grep '* master'
	git push github

# clean up feature branch BRANCH
done BRANCH=`git rev-parse --abbrev-ref HEAD`:
	git checkout master
	git diff --no-ext-diff --quiet --exit-code
	git pull --rebase github master
	git diff --no-ext-diff --quiet --exit-code {{BRANCH}}
	git branch -D {{BRANCH}}

test:
	cargo +nightly test --all

clippy:
	cargo +nightly clippy --all

fmt:
	cargo +nightly fmt --all

lint:
	./bin/lint

dev-deps:
	brew install grip
	cargo install cargo-watch
	cargo install cargo-outdated

check-minimal-versions:
	./bin/check-minimal-versions

check: test clippy lint check-minimal-versions
	git diff --no-ext-diff --quiet --exit-code
	cargo +nightly fmt --all -- --check

draft: push
	hub pull-request -o --draft

pr: check push
	hub pull-request -o

merge BRANCH=`git rev-parse --abbrev-ref HEAD`:
	#!/usr/bin/env bash
	set -euxo pipefail
	while ! hub ci-status --verbose {{BRANCH}}; do
		sleep 5
	done
	just done {{BRANCH}}

publish-check: check
	cargo +nightly outdated --exit-code 1

publish: publish-check
	#!/usr/bin/env bash
	set -euxo pipefail
	while ! hub ci-status --verbose; do
		sleep 5
	done
	git tag -a {{version}} -m 'Release {{version}}'
	git push github {{version}}
	cargo +nightly publish
	just merge

preview-readme:
	grip -b README.md
