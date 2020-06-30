# Contributing to assign!

assign! welcomes contribution via issues or code.

## Issues

Task tracking, enhancements, and bugs go to [issues](https://github.com/Kelerchian/assign/issues).

## Submitting code

To submit code, please follow these steps:

1. Fork the repository, or create branch if you are a contributor
2. Push commits in your fork or branch. Commit pushes will be automatically tested. See versioning [Versioning section](#versioning) below.
3. Create Pull Request from your branch to the repository
4. Pull Request will be reviewed and merged after it is approved

### Documentation and Tests

Write [documentation tests](https://doc.rust-lang.org/rustdoc/documentation-tests.html) to document and test code in one go.

Use `cargo doc` to review the documentation before submitting code.

## Versioning

assign! uses [semver](https://semver.org/) for versioning. Adjust the version in `Cargo.toml` accordingly.

## Publishing to crates.io

Publishing assign! to crates.io is manually done from local machine.

```
cargo publish
```

You need to `cargo login` before publishing. Contact owner or contributor for ownership or API token.
