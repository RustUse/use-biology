#!/usr/bin/env bash
set -euo pipefail

DRY_RUN=0
PUSH=0
REMOTE_NAMES=()

usage() {
  cat <<'EOF'
Usage: scripts/sync-mirrors.sh [--dry-run] [--push] [remote...]

Fetches configured git remotes and optionally pushes the current branch and tags
to named mirrors. With no remotes, all non-origin remotes are used.
EOF
}

run_cmd() {
  if [ "$DRY_RUN" -eq 1 ]; then
    printf '+'
    printf ' %q' "$@"
    printf '\n'
  else
    "$@"
  fi
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      DRY_RUN=1
      ;;
    --push)
      PUSH=1
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      REMOTE_NAMES+=("$1")
      ;;
  esac
  shift
done

if ! git rev-parse --show-toplevel >/dev/null 2>&1; then
  echo "run from a git repository" >&2
  exit 1
fi

if [ "${#REMOTE_NAMES[@]}" -eq 0 ]; then
  while IFS= read -r remote; do
    [ "$remote" = "origin" ] && continue
    REMOTE_NAMES+=("$remote")
  done < <(git remote)
fi

if [ "${#REMOTE_NAMES[@]}" -eq 0 ]; then
  echo "no mirror remotes configured"
  exit 0
fi

for remote in "${REMOTE_NAMES[@]}"; do
  run_cmd git fetch --prune "$remote"
done

if [ "$PUSH" -eq 1 ]; then
  current_branch="$(git branch --show-current)"
  if [ -z "$current_branch" ]; then
    echo "cannot push mirrors from a detached HEAD" >&2
    exit 1
  fi

  for remote in "${REMOTE_NAMES[@]}"; do
    run_cmd git push "$remote" "HEAD:${current_branch}"
    run_cmd git push "$remote" --tags
  done
fi
*** Add File: c:\Users\Joshua\Desktop\RustUse\git_local\use-biology\FORGES.md
# Forges and Mirrors

The canonical repository for RustUse/use-biology is GitHub:

```text
https://github.com/RustUse/use-biology
```

Other forges may be used as public read-only mirrors, issue intake mirrors, or CI mirrors, but they do not have release authority unless maintainers update this document and [GOVERNANCE.md](GOVERNANCE.md).

## Release Authority

- crates.io publishes are made from the canonical GitHub repository.
- GitHub release tags and release artifacts are authoritative.
- Mirror CI can validate changes, but it does not publish crates.

## Mirror Sync

Maintainers can use `scripts/sync-mirrors.sh` to fetch and optionally push configured mirror remotes:

```sh
scripts/sync-mirrors.sh --dry-run
scripts/sync-mirrors.sh --push gitlab codeberg
```

The script defaults to all configured non-`origin` remotes when no remote names are passed.

## Contribution Provenance

When a contribution originates on a mirror, preserve authorship when porting it to the canonical repository. Link the mirrored discussion from the canonical pull request whenever possible.

## Security Reports

Security reports should follow the RustUse organization security policy on the canonical repository. Do not discuss unresolved vulnerabilities in public mirror issues unless maintainers have already disclosed them.