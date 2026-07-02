# bineval

A Rust-native evaluation kernel and suite system.

## Alignment with *Ask, Don't Judge*

This project implements and expands on the philosophy defined in *Ask, Don't Judge: Binary Questions for Interpretable LLM Evaluation and Self-Improvement (arXiv:2606.27226)*.

Where we follow the paper:
- **Binary Questions**: All evaluations are framed as boolean pure functions in `src/core/policy.rs` (e.g., `no_jailbreak`).
- **Interpretability**: The system clearly scopes checks into transparent true/false policies rather than opaque sliding scales.

Where we extend the paper:
- **Infrastructure Auditing**: We apply binary questions to code metadata, repository state, and AST import graphs to build deterministic deployment gates (`can_promote_to_product`).
- **Hexagonal Architecture**: Core policies are isolated from IO.
- **Suite & Template Language**: We standardize the distribution of binary evaluation suites.
