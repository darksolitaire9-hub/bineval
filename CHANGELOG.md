# Changelog

All notable changes to the `bineval` deterministic golden test engine will be documented in this file.

## [v0.3.1] - 2026-07-05
### Changed
- **AST Extractor Resolution**: Relocated root dot-prefixed Python helper script `.bineval_ast_extractor.py` to `scripts/ast_extractor.py`. Updated `PythonAstAdapter` (`src/adapters/python_ast.rs`) to resolve `scripts/ast_extractor.py` dynamically with backward-compatible root fallback.
### Fixed
- **Repository Hygiene**: Purged historical scratch plan `docs/numeric_subset_plan.md` from git tracking and established comprehensive `.gitignore` rules for runtime logs, locks, and `.agents/`.

## [v0.3.0] - 2026-07-05
### Added
- **Numeric Subset Operator (`numeric_subset`)**: Added quantitative claim verification operator to evaluate whether numbers in target strings/arrays are a subset of expected reference tables without introducing domain logic into core.
- **Tabular Referencing Suite**: Added `suites/numeric_referencing.json` to verify agent data extraction accuracy against quantitative reference tables.
- **Comprehensive Documentation**: Added `docs/GUIDE.md` detailing architecture, operator syntax, and agent usage patterns.

## [v0.2.0] - 2026-07-04
### Added
- **Hexagonal Architecture Refactor**: Complete architectural separation into Core (policy evaluation, primitive matching), Ports (AST extraction, audit logging), and Adapters (`PythonAstAdapter`, stdout logger, filesystem policy loaders).
- **Domain-Agnostic Core**: Purged all domain-specific biology/physics terminology from core operators and suites, aligning with CS isomorphism standards (`CONTRIBUTING.md`).

## [v0.1.0] - 2026-06-30
### Added
- **Initial Release**: Deterministic unit testing and golden test engine for autonomous agents and MCP server evaluation.
- **Policy Operators**: Supported `eq`, `noteq`, `exists`, `notexists`, and `contains` operators for JSON/YAML AST assertions.
