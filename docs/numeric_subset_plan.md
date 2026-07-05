# bineval Implementation Plan: Numeric Subset Operator (`tabular_data_referencing_critic`)

**Status**: IMPLEMENTED (v0.3.0 Release)  
**Target Primitive**: `tabular_data_referencing_critic` (from `hope/potentials/transfers/tabular_data_referencing_critic`)  
**CS Isomorphism**: `critic_rejection_sampling` / numeric claim verification  

---

## 1. Lobster Debugging: Root Cause Analysis

- **Observe**: `bineval` is currently a purely structural evaluation kernel. `src/core/policy.rs` only supports 5 basic operators: `eq`, `noteq`, `exists`, `notexists`, and `contains`. The `suites/` directory is currently empty (only a `.gitkeep`). Meanwhile, `hope` has extracted `tabular_data_referencing_critic` (from arXiv 2606.32029v1), which proves that checking whether numbers in an output text are a subset of reference tables catches hallucinated data claims with 80%+ accuracy.
- **Hypothesize**: Why hasn't this been shipped into `bineval`? Because `bineval` lacks any mechanism to extract and compare numerical tokens from strings/arrays. Without a numeric operator, `bineval` cannot evaluate LLM outputs for quantitative hallucinations without violating its "no domain logic in core" rule (`CONTRIBUTING.md`).
- **Test**: If we introduce a pure, domain-agnostic operator called `NumericSubset` (or `"numeric_subset"`) to `Operator` in `src/core/policy.rs`, we can evaluate whether all numeric tokens in string/array `A` exist in reference set `B`. This adds powerful quantitative claim verification to `bineval` while remaining 100% domain-agnostic.
- **Resolve**: Track this plan inside `bineval/docs/` so it is permanently versioned in Git, then implement the operator, write unit tests in `policy.rs`, and populate `suites/` with its first real evaluation suite: `suites/numeric_referencing.json`.

---

## 2. User Review Required

> [!IMPORTANT]
> **No Domain Logic in Core**: In strict accordance with `CONTRIBUTING.md`, the new operator will be named `numeric_subset` (not `table_critic` or `tabular_validator`). It simply extracts floating-point and integer numbers from a target value and checks if they are a subset of the expected reference set.
> 
> **Git-Tracked Plan**: To resolve the issue of relying on external temporary folders, this plan is saved directly to `bineval/docs/numeric_subset_plan.md` in the repository.

---

## 3. Proposed Changes

### Component 1 — Core Policy Engine (`src/core/policy.rs`)

#### [MODIFY] [policy.rs](file:///f:/inputs/jun/bineval/src/core/policy.rs)
- **Enum Extension**: Add `#[serde(rename = "numeric_subset")] NumericSubset` to `pub enum Operator`.
- **Evaluation Logic**: Add a match arm for `Operator::NumericSubset` in `Policy::evaluate()`:
  - Extract all numbers from the target `val` (whether `val` is a string or array of numbers/strings) using character scanning or simple tokenization (matching `\d+(?:\.\d+)?`).
  - Extract all valid numbers from `self.expected`.
  - Verify that every number in `val` is contained in `expected` (with an exact string/numeric representation match or within epsilon `1e-6` for floats).
  - Return `Ok(())` if it is a subset, or an informative `Err("Hallucinated number found: <N> not in reference set")` if a number is missing.
- **Unit Tests**: Add unit tests inside `policy.rs` testing both `true` (valid subset) and `false` (hallucinated number present) paths.

---

### Component 2 — First Real Evaluation Suite (`suites/numeric_referencing.json`)

#### [NEW] [numeric_referencing.json](file:///f:/inputs/jun/bineval/suites/numeric_referencing.json)
- Create `suites/numeric_referencing.json` as the first production suite in `bineval`, replacing the empty directory.
- Define a suite that tests agent outputs against quantitative reference tables using `"operator": "numeric_subset"`.

---

### Component 3 — Integration Testing (`tests/integration_tests.rs`)

#### [MODIFY] [integration_tests.rs](file:///f:/inputs/jun/bineval/tests/integration_tests.rs)
- Add `test_integration_numeric_subset_passes()` and `test_integration_numeric_subset_fails()` to prove end-to-end CLI functionality when running `bineval audit` or `bineval run` with the new operator.

---

## 4. Verification Plan

### Automated Tests
1. **Unit & Integration Tests**: Run `cargo test` in `f:\inputs\jun\bineval\` to verify all unit tests and integration tests pass.
2. **Clippy & Formatting**: Run `cargo clippy -- -D warnings` and `cargo fmt --check` to ensure OSS hygiene compliance.

### Manual Verification
1. Run `./target/debug/bineval run --suite suites/numeric_referencing.json --path .` against sample primitives with valid and hallucinated numbers to verify exact exit codes (`0` vs `1`).
