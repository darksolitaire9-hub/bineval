# Changelog

All notable changes to the `bineval` deterministic golden test engine will be documented in this file.

## [v0.3.0] - 2026-07-05
### Added
- **Numeric Subset Operator (`numeric_subset`)**: Added quantitative claim verification operator to evaluate whether numbers in target strings/arrays are a subset of expected reference tables without introducing domain logic into core.
- **Tabular Referencing Suite**: Added `suites/numeric_referencing.json` to verify agent data extraction accuracy against quantitative reference tables.
- **Comprehensive Documentation**: Added `docs/GUIDE.md` detailing architecture, operator syntax, and agent usage patterns.
### Fixed
- **Repository Hygiene**: Removed dot-prefixed root helper scripts (`.bineval_ast_extractor.py` moved to `scripts/ast_extractor.py`), eliminated temporary scratch plans from git tracking, and established standardized `.gitignore` rules.

## [v0.2.0] - 2026-07-04
### Added
- **Hexagonal Architecture Refactor**: Complete architectural separation into Core (policy evaluation, primitive matching), Ports (AST extraction, audit logging), and Adapters (`PythonAstAdapter`, stdout logger, filesystem policy loaders).
- **Domain-Agnostic Core**: Purged all domain-specific biology/physics terminology from core operators and suites, aligning with CS isomorphism standards (`CONTRIBUTING.md`).

## [v0.1.0] - 2026-06-30
### Added
- **Initial Release**: Deterministic unit testing and golden test engine for autonomous agents and MCP server evaluation.
- **Policy Operators**: Supported `eq`, `noteq`, `exists`, `notexists`, and `contains` operators for JSON/YAML AST assertions.
