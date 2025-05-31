# Changelog

## 0.2.0

- **BREAKING:** wcount is now, by default, case sensitive. This done because of the complications of unicode. If want to use it
case insensitive, use the --case_insensitive (-c) flag.
- **BREAKING:** wcount error codes of errors and warnings have been switched. Now 1 for error, 2 for warning.
- Fixed a bug where --werror wouldn't finish the process early when a file couldn't be read.
- Enbettered error and warning messages.
- Minor performance improvements.

## 0.1.0

- Initial release