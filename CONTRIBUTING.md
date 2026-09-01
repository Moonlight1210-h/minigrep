# Contributing to minigrep

Thanks for your interest in contributing! Please follow these guidelines so we can review and merge changes quickly.

- Fork the repository and create a branch named `feature/short-description` or `fix/short-description`.
- Keep changes small and focused. One logical change per PR.
- Write tests for new features or bug fixes. Run `cargo test` before opening a PR.
- Ensure code is formatted: `cargo fmt`.
- Lint locally: `cargo clippy -- -D warnings`.
- Commit message style: `type(scope): short description` (e.g., `feat(cli): add ignore-case flag`).

When opening a PR, include:
- A concise description of the change.
- Any relevant issue number.
- Testing steps and expected behavior.

Maintainers will review and merge when ready. Thanks!
