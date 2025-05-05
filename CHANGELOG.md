# Changelog

## [Unreleased]

### Added
- New CLI options for improved scanner performance:
  - `--max-depth` option to limit directory scanning depth
  - `--skip-default-dirs` flag to automatically skip common build directories (on by default)
- Default exclusion of common build and metadata directories: `target/`, `node_modules/`, `.git/`, etc.

### Changed
- Significantly improved scanner performance on large codebases:
  - Pre-compile all regex patterns once at startup instead of per line
  - Two-phase file processing: first collect eligible files, then analyze them
  - More efficient filtering of files before content analysis
- Enhanced CLI help documentation with better descriptions of commands and options
- Updated all repository references from "flashnet-dev-utils" to "code-status-macros" to match the new repository name

### Fixed
- Fixed potential performance bottleneck when scanning workspace projects with many files
- Improved memory usage by avoiding unnecessary string allocations 