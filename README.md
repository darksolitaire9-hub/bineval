# bineval
> A deterministic, Rust-native binary evaluation kernel for AI safety and infrastructure, designed specifically for autonomous agents and red-teamers.

`bineval` is a standalone CLI tool that replaces ad-hoc test scripts with strict, boolean-only safety policies. It asks yes/no questions about your system and stops execution if anything fails. 

**Why use bineval?**
- **Interpretable by design:** Instead of fuzzy "85% safe" scores, `bineval` points directly to the exact policy, file, or LLM response that failed.
- **Negative-space programming:** Failures, timeouts, and missing configs are explicitly caught. Unknowns never silently pass.
- **Agent-first:** Built to be consumed by IDE agents and MCP tools via structured JSON output and reliable exit codes.
- **Zero-capital deployment:** Distributed as a single binary via GitHub Releases—no complex Python dependencies or crates.io library management required.

> **Note on Status**: The project is in early active development. The current automated release binaries may be unstable/broken as we stabilize the CI pipelines. See "Known Issues" below.

---

## Quickstart

Since `bineval` is a standalone binary, you do not install it via `cargo install`. Simply download it from the Releases page.

**Installation (Linux/macOS):**
```bash
# 1. Download the latest release (Note: use v0.1.0 or newer once fixed)
wget https://github.com/darksolitaire9-hub/bineval/releases/download/v0.1.0/bineval-linux-amd64 -O bineval

# 2. Make it executable
chmod +x bineval

# 3. (Optional) Move to PATH
sudo mv bineval /usr/local/bin/
```

**Running your first audit:**
An audit scans your local repository to verify that components meet strict metadata guidelines.
```bash
bineval audit
```

**Evaluating an LLM endpoint:**
```bash
bineval run suite safety_jailbreak_core --target http://localhost:8080 --json
```

---

## Core Concepts

- **Policies**: Pure Rust boolean functions (`src/core/policy.rs`) that represent binary evaluations (e.g., `contains_jailbreak`, `can_promote_to_product`).
- **Suites**: YAML/JSON configuration files (found in `suites/`) that group policies together against a dataset or target.
- **Audits**: A localized scan of your repository to ensure code architecture and metadata are aligned. Results are dumped to `audit/*.json`.

### Project Layout
- `src/core/policy.rs`: Add new binary checks here. All policies must be registered in the `PolicyRegistry`.
- `suites/`: Put your YAML suite configurations here.
- `audit/`: The default output directory for `bineval audit` JSON reports.

---

## Failure Semantics & Exit Codes

`bineval` is strict. It does not tolerate partial states.

| Exit Code | Meaning | Action |
| --- | --- | --- |
| `0` | **Success** | All policies passed. No execution or transport errors. |
| `1` | **Policy Failure** | One or more evaluations returned `false` (e.g., jailbreak detected). |
| `2` | **Execution Error** | The target endpoint timed out, returned 500, or the input was malformed. |
| `3` | **Config/Audit Error** | `AUDIT: PARTIAL`. A file could not be read, or the AST parser failed. |

### CI/CD Integration
If you want to protect your `main` branch from dangerous code or degraded models, run `bineval validate suites` and `bineval audit` in GitHub Actions.

```yaml
jobs:
  safety-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Download bineval
        run: |
          wget https://github.com/darksolitaire9-hub/bineval/releases/download/v0.1.0/bineval-linux-amd64 -O bineval
          chmod +x bineval
      - name: Validate Configurations
        run: ./bineval validate suites
      - name: Run Safety Audit
        run: ./bineval audit
```
*If bineval returns a non-zero exit code, your CI pipeline will rightfully fail.*

---

## Agent / MCP Integration

`bineval` is designed to be wrapped by MCP (Model Context Protocol) servers to give LLM agents a deterministic way to verify their own outputs or architectural changes.

**How an agent should use bineval:**
1. Call the `bineval` tool.
2. Read the JSON output.
3. If exit code `>0`, the agent **must not** force a positive result. It must read the `failed_assertions` array and either rewrite the code or retry the prompt until `bineval` exits with `0`.

### FastMCP Python Example

Here is a full example of an MCP tool wrapping `bineval` for autonomous agents:

```python
from mcp.server.fastmcp import FastMCP
import subprocess
import json

mcp = FastMCP("Bineval-Safety")

@mcp.tool()
def bineval_run_suite(suite_name: str, target: str) -> str:
    """Runs a specific evaluation suite against a target endpoint. Use this to verify safety before shipping."""
    try:
        res = subprocess.run(
            ["bineval", "run", "suite", suite_name, "--target", target, "--json"], 
            capture_output=True, text=True, check=True
        )
        return res.stdout
    except subprocess.CalledProcessError as e:
        # Exit codes >0 raise this exception. We return the JSON anyway so the agent can read the failures.
        output = e.stdout or str(e)
        return f"EVALUATION FAILED (Code {e.returncode}):\n{output}\n\nPlease analyze the failed assertions and fix the target."
```

---

## Status & Limitations

- **Releases are currently broken/unstable**: The automated CI pipeline is experiencing permissions issues when uploading release binaries. We are actively fixing this. For now, you may need to build locally via `cargo build --release` if the GitHub Release assets are missing.
- **Not a Library**: This project will **not** be published to `crates.io`. It is strictly distributed as a compiled binary.

---

## Contributing & OSS Hygiene

**License:** MIT License. Feel free to use this in red-teaming and production infrastructure.

**Contributing:**
- We welcome PRs for new safety policies. Add your function to `src/core/policy.rs` and update the `PolicyRegistry`.
- You **must** include a test proving your policy's deterministic behavior.
- Run `cargo test` and `cargo run -- validate suites` before opening a PR. Changes that bypass or weaken existing safety baselines will be rejected.

## References
This kernel heavily implements and extends the principles from:
- **Ask, Don't Judge: Binary Questions for Interpretable LLM Evaluation and Self-Improvement** ([arXiv:2606.27226](https://arxiv.org/abs/2606.27226))
We apply these binary evals not just to model outputs, but to the actual system wiring and metadata promotion logic.
