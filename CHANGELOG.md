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

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 58 commits contributed to the release.
 - 9 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #1 from martinus/mla/2025-10-14/we-must-pretend-to-be-professional ([`28a21b6`](https://github.com/martinus/hexler/commit/28a21b64be8a77a83e1601c3b51e5c9fd788865e))
    - Remove obsolete benchmark workflow from GitHub Actions ([`03d0841`](https://github.com/martinus/hexler/commit/03d0841ee6d4f67e43ddcbeb216d2b90d3b0f5ab))
    - Enhance caching strategy in CI workflow for improved performance and separation of toolchains ([`4f258dc`](https://github.com/martinus/hexler/commit/4f258dc37200d47d6bc62d5d18400054cddad4c7))
    - Update README with new features and screenshots; remove outdated image ([`f01f220`](https://github.com/martinus/hexler/commit/f01f22063e1fc0fed0319beb9b443cca1f39fcb7))
    - Update color constants in ByteToColor for clarity and consistency ([`5614be9`](https://github.com/martinus/hexler/commit/5614be90bb9b4ecbed227f74ca6e3d7b90d32446))
    - Improve buffer management in dump function by reusing formatted lines buffer and optimizing chunk processing ([`ec3388b`](https://github.com/martinus/hexler/commit/ec3388bdd59a7b3cece4224db68d6c4a3aa02a7c))
    - Optimize line formatting in dump function by reusing formatted lines vector and simplifying write_line method ([`0936fdc`](https://github.com/martinus/hexler/commit/0936fdca1440e84a3723689ac29c97bf5baf150e))
    - Add rayon for parallel processing in dump function and update line_writer for stateless line formatting ([`bb1b4a8`](https://github.com/martinus/hexler/commit/bb1b4a8401571a4ca22d3ab2136330ec8f3d08ea))
    - Enhance buffer management in dump function with triple buffering and improved channel capacity ([`7160b91`](https://github.com/martinus/hexler/commit/7160b910216bd6b56e04d5588a562ff1c20a38bd))
    - Bump version to 1.0.0 and update authors in Cargo.toml ([`221b341`](https://github.com/martinus/hexler/commit/221b34177deaa7917554736e0fca114acadf35b5))
    - Refactor dump and demo functions to improve thread safety and performance by using owned writer types, enabling better buffer management and reducing unnecessary copying. ([`9afea15`](https://github.com/martinus/hexler/commit/9afea152f5b0f74c944926a80b622ff7518fad58))
    - No need for buffered writer as we write large chunks anyways ([`bc2452b`](https://github.com/martinus/hexler/commit/bc2452b24bc1257cc21facf75070ed4257623796))
    - Refactor dump function to eliminate unnecessary copying and improve performance by writing directly from the buffer slice ([`d544be8`](https://github.com/martinus/hexler/commit/d544be8b0585e99f989bc527cb68677935d92513))
    - Refactor LineWriter and dump function to remove writer references, improving flexibility and performance ([`acf99ba`](https://github.com/martinus/hexler/commit/acf99ba8caa6db2b78cc6f636afc0e8a272e69cb))
    - Refactor to use byte arrays for border, color, and hex formatting, improving performance and memory efficiency ([`9aef868`](https://github.com/martinus/hexler/commit/9aef8685d383f3f6c7736d42f7e733b6752e9e73))
    - Refactor AsciiRenderer to use byte arrays for character representation and update LineWriter to handle byte arrays for color reset and rendering ([`ff2a6a7`](https://github.com/martinus/hexler/commit/ff2a6a79340ea4bc99eaf1b7898a5646b8597fe6))
    - Refactor ByteToColor to use byte arrays for color codes and optimize LineWriter for better performance ([`c50a307`](https://github.com/martinus/hexler/commit/c50a3074755e52cda42189ef87730423ac802e0a))
    - Optimize HexFormatter and LineWriter for improved performance and memory usage ([`f323fc4`](https://github.com/martinus/hexler/commit/f323fc4b5926a36dae84e358289ffbc163167ad4))
    - Some minor performance optimizations ([`5cebdb2`](https://github.com/martinus/hexler/commit/5cebdb2ada04b4ffad765ede5e5d20a7c3b25037))
    - Update Cargo.toml to include homepage, keywords, and categories ([`9a22c37`](https://github.com/martinus/hexler/commit/9a22c3762cec5f1cce7d73bf64af00d6dcbd30c8))
    - Fixed all clippy warnings ([`3dc6146`](https://github.com/martinus/hexler/commit/3dc6146ce820bb64cafd04ce1fb0a508f197bd8f))
    - Run cargo fmt ([`9599c62`](https://github.com/martinus/hexler/commit/9599c621a1ac2caea889499c6d0d4f3c1ced4f4f))
    - Converted to library, this allows doctest to run with 'cago test --doc'. ([`1eeedbd`](https://github.com/martinus/hexler/commit/1eeedbd86480821ad816a8c10fdc764104b475d4))
    - Enhance documentation across multiple modules with detailed comments and examples ([`6923e93`](https://github.com/martinus/hexler/commit/6923e9366966476bb9a54021c4d7ac31667adb5e))
    - Add AsciiRenderer, BorderWriter, HexFormatter, and update LineWriter for hex dump output ([`44d1374`](https://github.com/martinus/hexler/commit/44d1374ae609c28df25dd4b0da47e7d69a4516f4))
    - Add unit tests for ByteToColor, HexlerError, and LineWriter functionality ([`d65d1b1`](https://github.com/martinus/hexler/commit/d65d1b1f0ab0a20b70806a9cf88dc1ebc3646229))
    - Fix escape sequences for color constants in ByteToColor and LineWriter ([`40d6233`](https://github.com/martinus/hexler/commit/40d62335fb00a078093706cd39f23c7e072f2364))
    - Implement custom error handling with HexlerError and update related functions ([`1e3873a`](https://github.com/martinus/hexler/commit/1e3873a9e78c467f48c42c1d1624fb6199088643))
    - Update README to clarify preference for hexler features over competitors ([`059ce6d`](https://github.com/martinus/hexler/commit/059ce6df6dc0cebe9ee175ab552d3f1f8c40b02d))
    - Add screenshot to README and adjust image placement ([`c02d648`](https://github.com/martinus/hexler/commit/c02d64816951cb9ddf4281e4c345476633605de1))
    - Pass usize as copy, not reference ([`14bfba2`](https://github.com/martinus/hexler/commit/14bfba2d56554d5b29e97655529e49a4f18c6c61))
    - Minor cleanup ([`c6d2c79`](https://github.com/martinus/hexler/commit/c6d2c791fc9de8bd2efe5207a573c42d71103df5))
    - Don't copy writer, add lifetime, adds a test. ([`f45e1b4`](https://github.com/martinus/hexler/commit/f45e1b49db79c0dc29a3f5a600fba308d6488cae))
    - Compiles again, readme ([`11202ea`](https://github.com/martinus/hexler/commit/11202ea979c40bbdf95d3405f3a5ca3d998e053c))
    - Writes hex offset with grey leading zeroes ([`5c4d367`](https://github.com/martinus/hexler/commit/5c4d36722aee4bc4dbe493ef5fcea459a3c17c68))
    - Pager, add footer ([`5fa29fa`](https://github.com/martinus/hexler/commit/5fa29fa9762cd6d72f38015f475c6bbe7c298510))
    - Better dark gray ([`66bf888`](https://github.com/martinus/hexler/commit/66bf8881a5d68ba73b747a8d7f684b40b25545c6))
    - Refactoring ([`84a1b60`](https://github.com/martinus/hexler/commit/84a1b60835f9071e2b61c54664bb7b91300c4a6c))
    - Slightly better header ([`c716216`](https://github.com/martinus/hexler/commit/c7162167704a26bbc1191a3de08ff98185f12284))
    - Cleanup ([`3d9fa15`](https://github.com/martinus/hexler/commit/3d9fa155a467a6731acec5453c981f7b628e62ee))
    - Cleanup ([`1154e4a`](https://github.com/martinus/hexler/commit/1154e4a02ffe9d22c3953df52be3af2062184d27))
    - Print file & file size ([`97fb196`](https://github.com/martinus/hexler/commit/97fb1969776a186a9786230a600faf45dc3a2f64))
    - Cleanup ([`a3e9b2d`](https://github.com/martinus/hexler/commit/a3e9b2dd04776595158a5d15de248abf159e7f1d))
    - Better width logic ([`101a5e9`](https://github.com/martinus/hexler/commit/101a5e9ec564d3213647c0b1c5f6daf5018c4c11))
    - Add some arguments ([`375cc18`](https://github.com/martinus/hexler/commit/375cc1892573358035ff25d1b40123f60d0c1de1))
    - Minor cleanup ([`677fdd6`](https://github.com/martinus/hexler/commit/677fdd6e8b53895268986eba3021649671ccedf4))
    - Better output, bugfix ([`29ddc7d`](https://github.com/martinus/hexler/commit/29ddc7d1cf692d9e7aec2c479ebe2b2e20a0b351))
    - Works ([`c4b7e7b`](https://github.com/martinus/hexler/commit/c4b7e7b3271167b3ad570bcaaf2be49ed20e26ef))
    - Hex is generated, with space ([`3ea5f75`](https://github.com/martinus/hexler/commit/3ea5f752b8b65a52647a33ef9ab2499fd97248dd))
    - Output works nicely, but it's a bit slow ([`5150f3e`](https://github.com/martinus/hexler/commit/5150f3e795458188b136a1e9e2b7208be8b04a29))
    - WIP ([`d8f8439`](https://github.com/martinus/hexler/commit/d8f843902e92a27040deb3fdd49c6c7534387a22))
    - Works, but  looks a bit complicated ([`85f7bcb`](https://github.com/martinus/hexler/commit/85f7bcb94fcbf0632d6d244bfb2f74eb2bf56f4f))
    - Split up everything, writing hex ([`d850550`](https://github.com/martinus/hexler/commit/d850550a55c273b68a29e50b6dbe19eb75ea7c3d))
    - WIP ([`a909669`](https://github.com/martinus/hexler/commit/a90966969e4d511d2a9fccdc7167f491949d04ab))
    - WIP ([`7821c0e`](https://github.com/martinus/hexler/commit/7821c0e8b32cb1f0f41373c0765bc84e70230688))
    - Adds testdata ([`1c64c2b`](https://github.com/martinus/hexler/commit/1c64c2b794dbc97047dfb38cb64f5a5cd7fb64fd))
    - Cargo init ([`8d2c35c`](https://github.com/martinus/hexler/commit/8d2c35c57af2db8a8463d68d1623b8a2fd8d618e))
    - Initial commit ([`2070e8e`](https://github.com/martinus/hexler/commit/2070e8eda931e763a6b122e74eb9663a99500c11))
</details>

