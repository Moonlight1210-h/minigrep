# Contributing to minigrep

Thanks for your interest in contributing to `minigrep`! Please follow these guidelines so we can review, test, and merge your changes efficiently.

---

## Workflow Guidelines

1. **Branch Naming**:
   - Fork the repository and create a descriptive branch:
     - Features: `feature/short-description`
     - Bug fixes: `fix/short-description`
     - Documentation: `docs/short-description`

2. **Commit Conventions**:
   - Follow the Conventional Commits specification: `type(scope): short description`
   - *Examples*:
     - `feat(search): add regex pattern matching support`
     - `test(cli): add integration test for line-number flag`
     - `docs(readme): update usage examples for v0.2.0`

3. **Keep PRs Focused**:
   - Keep changes small and focused. Submit one logical change per Pull Request.

---

## Testing & Quality Assurance

Before submitting a Pull Request, ensure all local checks and tests pass.

### 1. Code Formatting & Linting
Ensure your code adheres to standard Rust formatting and passes all static analysis lints:
```bash
# Format code automatically
cargo fmt

# Check for warnings and enforce clean code standards
cargo clippy -- -D warnings