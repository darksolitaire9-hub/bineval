# bineval: a Rust-native binary eval kernel for AI infra and safety.

## Table of Contents
- [ELI5](#eli5)
- [Human Quickstart](#human-quickstart)
- [Agent Integration](#agent-integration)
- [Failure Semantics](#failure-semantics)
- [Integrity & Tampering](#integrity--tampering)
- [FAQs](#faqs)

---

## ELI5
`bineval` is a small robot that checks if your AI system is wired correctly and behaving safely. It asks strict, yes/no (binary) questions and shows exactly where things fail. Instead of giving you a fuzzy "85% safe" score, it points directly to the exact file or LLM response that broke the rules.

## Human Quickstart
**Installation:**
```bash
cargo build --release
```

**Running an Audit:**
An audit scans your local repository to verify that all primitives and components meet strict metadata guidelines.
```bash
bineval audit
```
It prints a human-readable text report and generates JSON schemas in `audit/*.json`.

**Running a Suite:**
Suites group templates to run real evaluations against endpoints.
```bash
bineval run suite safety_jailbreak_core --target http://localhost:8080
```

## Agent Integration
`bineval` provides strict JSON schemas and stable CLI commands designed for autonomous agents.

**Commands for Agents:**
- `bineval audit --json`
- `bineval run suite <name> --target <url> --json`
- `bineval validate suites`

**Agent Rules:**
- Agents **must** query bineval for binary answers (yes/no) to drive their self-improvement loop, mirroring the idea of self-improvement via binary evaluations.
- Agents **must not** override or bypass failures by forcing positive status updates.

## IDE & MCP Integration
### Local IDE (VS Code)
Wire bineval directly into your `.vscode/tasks.json` so you can dogfood it with a single click:
```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "bineval validate",
      "type": "shell",
      "command": "tools/bineval validate suites",
      "problemMatcher": []
    },
    {
      "label": "bineval audit",
      "type": "shell",
      "command": "tools/bineval audit",
      "problemMatcher": []
    }
  ]
}
```

### Agent MCP Integration
Add the following FastMCP tool definitions so your agents can deterministically run suites:
```python
@mcp.tool()
def bineval_audit() -> str:
    \"\"\"Runs the bineval audit command over the local repository.\"\"\"
    return run_shell("tools/bineval audit --json")

@mcp.tool()
def bineval_run_suite(suite_name: str, target: str) -> str:
    \"\"\"Runs a specific evaluation suite against a target endpoint.\"\"\"
    return run_shell(f"tools/bineval run suite {suite_name} --target {target} --json")
```

## Failure Semantics
We practice "Negative Space Programming." Unknowns and failures never silently pass.

- **Execution failure**: The endpoint was unreachable, timed out, or crashed. The suite failed to run meaningfully.
- **Evaluation failure**: The suite ran successfully, but one or more policies returned `false` (e.g. jailbreak detected).

**Exit Codes**:
- `0`: Success. No `Error` severity failures, no execution errors.
- `>0`: Any failure, or a partial audit caused by missing files or broken AST parsers.

## Integrity & Tampering
To prevent quiet tampering by agents or configuration drift:
- Run `bineval validate suites` before making any commits. It statically checks that all policies map to known Rust functions.
- **Baseline Suites** (e.g., `safety_jailbreak_core`) are protected via GitHub Actions CI. Direct unreviewed changes to baseline suites will fail CI checks.

## FAQs
**Why Rust and Tokio?**
Rust provides the strong type system needed to eliminate silent failures, while Tokio allows for massive concurrency when querying remote LLM endpoints for suite evaluation.

**How to add a new policy?**
Add a pure boolean function to `src/core/policy.rs`, map it in the `PolicyRegistry`, and write a unit test confirming its behavior.

**What to do when `AUDIT: PARTIAL` appears?**
This means the AST extractor or JSON config loader failed. Fix the underlying file system error or syntax error preventing parsing; do not ignore partial audits.

## References
This kernel heavily implements and extends the principles from:
- **Ask, Don't Judge: Binary Questions for Interpretable LLM Evaluation and Self-Improvement** ([arXiv:2606.27226](https://arxiv.org/abs/2606.27226))
We apply these binary evals not just to model outputs, but to the actual system wiring and metadata promotion logic.
