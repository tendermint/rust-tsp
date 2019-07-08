# Contributing to rust-ABCI

Thank you for taking the time to contribute to our organization and this repository.

Please follow standard github best practices: fork the repo, branch from the tip of develop, make some commits, and submit a pull request to develop. See the open (issues)[https://github.com/tendermint/rust-abci/issues] for things to contribute.

## Code of Conduct

---

All contributors are expected to follow (Code of Conduct)[./CODE_OF_CONDUCT.md]

## Feature requests and bug reports

---

Feature requests or bug reports can be posted as a (Github issue)[https://github.com/tendermint/rust-abci/issues/new]. In the issue, please follow the template and describe to the best of your ability what you did, what you expected and what happened instead.

If you'd like to solve a issue, please comment that you would like to claim the issue. This makes sure that no one else will work on it.

## Forking

---

Once you have a issue that you would to work on and commented on it to claim it:

- Fork the repository to your Github account.
- Clone your fork to your local machine
- Create a local branch `git checkout -b <branch_name>`.
- Commit & push your changes to your branch on github.
- Make sure that you are working off the most recent version, if not, pull the develop branch and rebase your committed history.
- Before opening a pull request read the [development section](#development)
- Open a [Pull Request](#pull-requests) for others to review the branch.

## Pull Requests

---

To accommodate review process we suggest that PRs are categorically broken up. Ideally each PR addresses only a single issue. Additionally, as much as possible code refactoring and cleanup should be submitted as a separate PRs from bugfixes/feature-additions.

If you are working on a issue and plan on contributing, please submit your PR early and make sure its opened as a `Draft`, even if it's incomplete, this indicates to the community you're working on something and allows them to provide comments early in the development process. When the code is complete it should be marked using Github's `Mark Ready` feature. This will let the maintainers know that there is a open PR for review.

## Development

---

The code should follow [Rust Style Guide](https://github.com/rust-lang/rfcs/tree/master/style-guide). Much of the code style is captured by `rustfmt`.

Before opening a Pull Request please run the checks below:

- Install rustfmt: `rustup component add rustfmt`
- Install clippy: `rustup component add clippy`

### Testing

Run the test suite with

`cargo test --all-features`

### Format checking (rustfmt)

Make sure your code is well-formatted by running

`cargo fmt`

### Lint (clippy)

Lint your code (i.e. check it for common issues) with:

`cargo clippy`

## Changelog

Every fix, improvement, feature, or breaking change should be made in a
pull-request that includes an update to the `CHANGELOG.md` file.

Changelog entries should be formatted as follows:

```
- [module] \#xxx Some description about the change (@contributor)
```

Here, `module` is the part of the code that changed (typically a
top-level Go package), `xxx` is the pull-request number, and `contributor`
is the author/s of the change.

It's also acceptable for `xxx` to refer to the relevent issue number, but pull-request
numbers are preferred.
Note this means pull-requests should be opened first so the changelog can then
be updated with the pull-request's number.
There is no need to include the full link, as this will be added
automatically during release. But please include the backslash and pound, eg. `\#2313`.

Changelog entries should be ordered alphabetically according to the
`module`, and numerically according to the pull-request number.

Changes with multiple classifications should be doubly included (eg. a bug fix
that is also a breaking change should be recorded under both).

## License

Copyright © 2018-2019 Tendermint

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
