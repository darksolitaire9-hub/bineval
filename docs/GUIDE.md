# bineval Usage & Architecture Guide: For Agents and Humans

Welcome to the canonical reference guide for `bineval`. This document is structured to serve both **autonomous agents** (via LLM tool-calling loops and MCP integrations) and **human engineers** (architecting evaluation suites, writing CI/CD pipelines, and debugging failure boundaries).

---

## 1. Core Philosophy: Why Binary Evaluation?

Traditional LLM evaluation relies on fuzzy scoring (e.g., "85% similarity" or "LLM-as-a-judge score of 4/5"). While useful for exploratory data analysis, continuous scoring fails in autonomous loops and safety gating because it lacks an unambiguous **stopping condition**.

`bineval` acts as a deterministic, negative-space kernel that enforces strict boolean logic:
- **Zero Domain Logic in Core**: The engine evaluates structured data primitives against boolean rules. Vocabulary like `PRODUCT`, `TRIAL`, `feature_flag`, or `jailbreak` is strictly forbidden inside the Rust binary (`src/core/policy.rs`).
- **All Semantics Live in Suites**: Domain definitions live entirely in external JSON/YAML configuration files (`suites/`).
- **Binary Rejection Sampling**: If any policy evaluates to `false`, `bineval` exits with code `1`. There is no "partial pass."

---

## 2. Guide for Autonomous Agents

As an autonomous agent (such as an IDE coding assistant or autonomous research agent operating under `AGENTS.md`), you interact with `bineval` either via CLI commands or through Model Context Protocol (MCP) server wrappers like `bineval_audit` and `bineval_run_suite`.

### Agent Protocol & Mandatory Rules
1. **Never Force a Positive Result**: If `bineval` returns an exit code `>0` or an error message containing `EVALUATION FAILED` / `AUDIT FAILED`, you **must not** ignore the failure, mock the output, or claim success.
2. **Read the Failed Assertions**: Parse the stderr/stdout payload. Identify the exact suite ID, policy ID, target field (`path`), and observed value that triggered the violation.
3. **Self-Correction & Rejection Sampling**:
   - For quantitative hallucination failures (where `numeric_subset` fails with a message like `"Hallucinated number found: <N> not in reference set"`), inspect why number `<N>` appeared in your output when it was not present in the reference data table.
   - Refine the generated text, code, or configuration, and re-invoke `bineval` until it exits with `0`.
4. **Pre-Shipment Verification**: Always run `cargo test` and `cargo run -- validate` before committing code changes or declaring a task complete.

### MCP Integration Example
When invoking `bineval` via an MCP tool, expect JSON or formatted text outputs:
```json
{
  "status": "FAILED",
  "exit_code": 1,
  "failed_policy": "valid_subset_basic",
  "operator": "numeric_subset",
  "details": "Hallucinated number found: 999 not in reference set"
}
```

---

## 3. Guide for Human Engineers & Suite Authors

Human engineers are responsible for defining what safety and correctness mean by authoring evaluation suites in `suites/`.

### Authoring an Evaluation Suite
Suites are written in JSON or YAML and placed inside the `suites/` directory. Each suite contains a list of `policies` evaluated against target data files (primitives).

```json
{
  "suite_id": "numeric_referencing",
  "description": "Verifies that all numeric claims in target outputs exist in reference data tables.",
  "policies": [
    {
      "id": "financial_report_check",
      "description": "Ensures generated financial summary contains no hallucinated numbers.",
      "path": "summary_text",
      "operator": "numeric_subset",
      "expected": [-1234.50, 1000000, 2026]
    }
  ]
}
```

### Quantitative Claim Verification (`numeric_subset`)
Inspired by the ACL 2026 Oral paper (*Tabular Data Referencing Critic*, arXiv:2606.32029), the `numeric_subset` operator provides domain-agnostic numerical hallucination detection:
- **How it works**: It tokenizes all numbers in the target value (whether a string or array of numbers/strings) and checks if every extracted number exists in the reference `expected` array.
- **Formatting Noise Immunity**:
  - **Commas**: Thousands separators (e.g., `1,000,000` or `-1,234.50`) are automatically stripped before parsing.
  - **Trailing Periods**: Sentence-ending decimals (e.g., `"Revenue was 120."`) are trimmed so the trailing period is not mistaken for an incomplete float.
  - **Negative Numbers**: Leading minus signs (`-`) preceded by whitespace or string boundaries are cleanly captured as negative quantities.
  - **Float Epsilon Tolerance**: Floating-point comparisons use an epsilon tolerance of `1e-6` to prevent false alarms from standard binary floating-point rounding drift.

---

## 4. Operator Reference Catalog

All policies in `src/core/policy.rs` must specify one of the following domain-agnostic operators:

| Operator | Expected Argument | Description |
| :--- | :--- | :--- |
| `eq` | Any JSON Value | Target value at `path` must strictly equal `expected`. |
| `noteq` | Any JSON Value | Target value at `path` must NOT equal `expected`. |
| `exists` | None / `null` | Target object must contain field `path` (and not be null). |
| `notexists`| None / `null` | Target object must NOT contain field `path` (or be null). |
| `contains` | Substring / Element | Target string/array must contain `expected`. |
| `numeric_subset` | Array of Numbers | All numeric tokens extracted from target at `path` must exist in `expected`. |

---

## 5. CI/CD Integration & Debugging

When debugging a suite failure in CI/CD or local development, apply **Lobster Debugging** principles:
1. **Observe**: Check the exit code (`1` = policy failure, `2` = execution error, `3` = config/audit error). Read the exact `policy_id` printed to stderr.
2. **Hypothesize**: Is the failure caused by a true hallucination/regression in the target primitive, or has the system requirement evolved such that the baseline suite needs updating?
3. **Test**: Run single-suite validation locally using `./bineval run --suite <suite_id> --path . --targets <target_dir>`.
4. **Resolve**: Either fix the target artifact to satisfy the baseline, or submit a Pull Request upgrading the suite with documented justification.

### Pre-Commit Hygiene Script
Run this sequence before submitting any Pull Request:
```bash
cargo fmt --all
cargo clippy -- -D warnings
cargo test
cargo run -- validate
```
