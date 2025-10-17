# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.0.3-test (2025-10-17)

### Bug Fixes

 - <csr-id-7a726e7e39005f3a284e44daae5692fb0cc0e1a6/> set git author/committer identity for cargo-smart-release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Set git author/committer identity for cargo-smart-release ([`7a726e7`](https://github.com/martinus/hexler/commit/7a726e7e39005f3a284e44daae5692fb0cc0e1a6))
</details>

## v1.0.2-test (2025-10-17)

<csr-id-834cd977ce5de9a1a0384444a5b154d38f29cbbe/>

### Refactor

 - <csr-id-834cd977ce5de9a1a0384444a5b154d38f29cbbe/> enhance release workflow with improved caching and branch handling

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Enhance release workflow with improved caching and branch handling ([`834cd97`](https://github.com/martinus/hexler/commit/834cd977ce5de9a1a0384444a5b154d38f29cbbe))
</details>

## v1.0.1-test (2025-10-16)

<csr-id-f751c4055d86470edf8da766e37de8680220b92b/>
<csr-id-bfca9376b3b15b471ddd290b78ffaafa9b71ca72/>
<csr-id-04e40893298aea45c4fbef15d29142bf7869dc7e/>
<csr-id-c6b718e27c45a81db5f31b2fda2ed669675b1176/>
<csr-id-2718ffafbff1c3ccd9f3c7e8fa5f43d28b43b8e2/>

### Chore

 - <csr-id-f751c4055d86470edf8da766e37de8680220b92b/> bump size from 0.4.1 to 0.5.0
   Bumps [size](https://github.com/neosmart/prettysize-rs) from 0.4.1 to 0.5.0.
   - [Release notes](https://github.com/neosmart/prettysize-rs/releases)
   - [Commits](https://github.com/neosmart/prettysize-rs/compare/0.4.1...0.5.0)
   
   ---
   updated-dependencies:
   - dependency-name: size
     dependency-version: 0.5.0
     dependency-type: direct:production
     update-type: version-update:semver-minor
   ...
 - <csr-id-bfca9376b3b15b471ddd290b78ffaafa9b71ca72/> bump thiserror from 1.0.69 to 2.0.17
   Bumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.69 to 2.0.17.
   - [Release notes](https://github.com/dtolnay/thiserror/releases)
   - [Commits](https://github.com/dtolnay/thiserror/compare/1.0.69...2.0.17)
   
   ---
   updated-dependencies:
   - dependency-name: thiserror
     dependency-version: 2.0.17
     dependency-type: direct:production
     update-type: version-update:semver-major
   ...
 - <csr-id-04e40893298aea45c4fbef15d29142bf7869dc7e/> bump actions/checkout from 4 to 5
   Bumps [actions/checkout](https://github.com/actions/checkout) from 4 to 5.
   - [Release notes](https://github.com/actions/checkout/releases)
   - [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)
   - [Commits](https://github.com/actions/checkout/compare/v4...v5)
   
   ---
   updated-dependencies:
   - dependency-name: actions/checkout
     dependency-version: '5'
     dependency-type: direct:production
     update-type: version-update:semver-major
   ...
 - <csr-id-c6b718e27c45a81db5f31b2fda2ed669675b1176/> bump codecov/codecov-action from 4 to 5
   Bumps [codecov/codecov-action](https://github.com/codecov/codecov-action) from 4 to 5.
   - [Release notes](https://github.com/codecov/codecov-action/releases)
   - [Changelog](https://github.com/codecov/codecov-action/blob/main/CHANGELOG.md)
   - [Commits](https://github.com/codecov/codecov-action/compare/v4...v5)
   
   ---
   updated-dependencies:
   - dependency-name: codecov/codecov-action
     dependency-version: '5'
     dependency-type: direct:production
     update-type: version-update:semver-major
   ...

### Refactor

 - <csr-id-2718ffafbff1c3ccd9f3c7e8fa5f43d28b43b8e2/> simplify release workflow by using cargo-smart-release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Simplify release workflow by using cargo-smart-release ([`2718ffa`](https://github.com/martinus/hexler/commit/2718ffafbff1c3ccd9f3c7e8fa5f43d28b43b8e2))
    - Merge pull request #2 from martinus/dependabot/github_actions/codecov/codecov-action-5 ([`5f2d89c`](https://github.com/martinus/hexler/commit/5f2d89c301acb8adfa0b26d2ccf656025b1c45fd))
    - Merge pull request #3 from martinus/dependabot/github_actions/actions/checkout-5 ([`4692c52`](https://github.com/martinus/hexler/commit/4692c5290da3e47161e4d629026d33f1c2f70d8f))
    - Merge pull request #4 from martinus/dependabot/cargo/thiserror-2.0.17 ([`d26cc48`](https://github.com/martinus/hexler/commit/d26cc48cdd1b4a882e371475685a18b8fa0ca188))
    - Merge pull request #5 from martinus/dependabot/cargo/size-0.5.0 ([`48bee8e`](https://github.com/martinus/hexler/commit/48bee8e194f1a0cfcd041d234e5113b37cf4de33))
    - Bump size from 0.4.1 to 0.5.0 ([`f751c40`](https://github.com/martinus/hexler/commit/f751c4055d86470edf8da766e37de8680220b92b))
    - Bump thiserror from 1.0.69 to 2.0.17 ([`bfca937`](https://github.com/martinus/hexler/commit/bfca9376b3b15b471ddd290b78ffaafa9b71ca72))
    - Bump actions/checkout from 4 to 5 ([`04e4089`](https://github.com/martinus/hexler/commit/04e40893298aea45c4fbef15d29142bf7869dc7e))
    - Bump codecov/codecov-action from 4 to 5 ([`c6b718e`](https://github.com/martinus/hexler/commit/c6b718e27c45a81db5f31b2fda2ed669675b1176))
</details>

## v1.0.0 (2025-10-17)

<csr-id-03d0841ee6d4f67e43ddcbeb216d2b90d3b0f5ab/>
<csr-id-4f258dc37200d47d6bc62d5d18400054cddad4c7/>
<csr-id-5614be90bb9b4ecbed227f74ca6e3d7b90d32446/>
<csr-id-ec3388bdd59a7b3cece4224db68d6c4a3aa02a7c/>
<csr-id-0936fdca1440e84a3723689ac29c97bf5baf150e/>
<csr-id-7160b910216bd6b56e04d5588a562ff1c20a38bd/>
<csr-id-108cb8ed014421a82a60dc2245dce39b96ba51d1/>

### Documentation

 - <csr-id-f01f22063e1fc0fed0319beb9b443cca1f39fcb7/> Update README with new features and screenshots; remove outdated image

### New Features

 - <csr-id-bb1b4a8401571a4ca22d3ab2136330ec8f3d08ea/> Add rayon for parallel processing in dump function and update line_writer for stateless line formatting
 - <csr-id-221b34177deaa7917554736e0fca114acadf35b5/> Bump version to 1.0.0 and update authors in Cargo.toml

### Refactor

 - <csr-id-03d0841ee6d4f67e43ddcbeb216d2b90d3b0f5ab/> Remove obsolete benchmark workflow from GitHub Actions
 - <csr-id-4f258dc37200d47d6bc62d5d18400054cddad4c7/> Enhance caching strategy in CI workflow for improved performance and separation of toolchains
 - <csr-id-5614be90bb9b4ecbed227f74ca6e3d7b90d32446/> Update color constants in ByteToColor for clarity and consistency
 - <csr-id-ec3388bdd59a7b3cece4224db68d6c4a3aa02a7c/> Improve buffer management in dump function by reusing formatted lines buffer and optimizing chunk processing
 - <csr-id-0936fdca1440e84a3723689ac29c97bf5baf150e/> Optimize line formatting in dump function by reusing formatted lines vector and simplifying write_line method
 - <csr-id-7160b910216bd6b56e04d5588a562ff1c20a38bd/> Enhance buffer management in dump function with triple buffering and improved channel capacity

### Chore

 - <csr-id-108cb8ed014421a82a60dc2245dce39b96ba51d1/> add CHANGELOG.md to document project updates and version history

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release hexler v1.0.0 ([`7ed7c81`](https://github.com/martinus/hexler/commit/7ed7c816b2da9e7b92a4d81b6717d5700c19f073))
    - Add CHANGELOG.md to document project updates and version history ([`108cb8e`](https://github.com/martinus/hexler/commit/108cb8ed014421a82a60dc2245dce39b96ba51d1))
</details>

