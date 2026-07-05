# Contributing to bineval

Thank you for contributing to `bineval`! This project acts as a strict, negative-space kernel for AI safety baselines. 

Because autonomous agents rely on our binary evaluations to correctly measure their own failure and safety posture, we maintain a very high standard for new code.

## 1. Adding New Policies

All policies must be strictly boolean (true/false) evaluations.
1. Add your pure Rust boolean function to `src/core/policy.rs`.
2. Register the policy in the `PolicyRegistry` inside the same file.
3. Your policy should not have side effects. It should only evaluate data returned by adapters.
4. **CRITICAL**: No domain-specific logic is allowed in `bineval` core. You must not introduce or rely on domain vocabulary (e.g. `PRODUCT`, `TRIAL`, `experiment`, `feature flag`). All rules belong in external suite configurations, and your policy must remain generic.

## 2. Writing Deterministic Tests

We don't do fuzzy testing here.
1. Any new policy MUST have a corresponding unit test in `src/core/policy.rs`.
2. You must test both the `true` (success) path and the `false` (failure) path explicitly.

## 3. Writing Evaluation Suites

Evaluation suites live in the `suites/` directory as `.json` or `.yaml` files. They define collections of policies that are evaluated against target primitive data files.

### Suite Schema
A valid suite file must match the following schema:
```json
{
  "suite_id": "unique_suite_name",
  "description": "Optional human-readable summary of what this suite evaluates.",
  "policies": [
    {
      "id": "policy_case_name",
      "description": "Optional explanation of this policy rule.",
      "path": "target_field_name",
      "operator": "eq | noteq | exists | notexists | contains | numeric_subset",
      "expected": "expected_value_or_reference_array"
    }
  ]
}
```
- **`suite_id`** (or `id`): Uniquely identifies the suite when running `bineval run --suite <suite_id>`.
- **`policies`**: An array of policy evaluation objects.
  - **`path`**: The JSON key inside the target primitive object to inspect (if empty `""`, evaluates the root primitive value itself).
  - **`operator`**: The boolean comparison operator to apply.
  - **`expected`**: The reference value, array, or constant required by operators like `eq`, `contains`, or `numeric_subset`.

## 4. Pre-Commit Checklist

Before opening a Pull Request, you must run the following locally and ensure they exit with `0`:

```bash
# 1. Format code
cargo fmt --all

# 2. Lint code (no warnings allowed)
cargo clippy -- -D warnings

# 3. Run all unit and integration tests
cargo test

# 4. Verify baseline configurations
cargo run -- validate suites
```

If your change causes a baseline suite in `suites/` to fail validation, your PR will be blocked by our CI pipeline. If you need to upgrade a baseline suite, include it in your PR and explain why the safety requirement changed.
