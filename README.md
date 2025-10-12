# hexler - A colorful hex printer with opinionated defaults

`hexler` is a simple hex viewer. Its most distinguishing features are:
* Automatically uses whole terminal width
* colorized output
* Defaults to using a pager (same as e.g. `git`)
* Writes code page 437 characters for better readability

![hexler screenshot](img/Screenshot_20251012_101602.png)

I wrote this program mostly to get experience with Rust. There are more powerful tools (e.g. [hexyl](https://github.com/sharkdp/hexyl)) and
faster ones (e.g. [hastyhex](https://github.com/skeeto/hastyhex)). Nevertheless, I prefer mine because of more reasonable colors and it is just prettier.

## Similar Programs

* TSCD https://github.com/fosres/TSCD
  * Red: non-printable
  * Orange: printable (Alphabetic)
  * Yellow: Base 10 digits
  * Green: ASCII Whitespace
  * Purple: Punctuation Characters
  * Gray: NUL
* hastyhex https://github.com/skeeto/hastyhex
  * green: Whitespace (0a, 0b, 0c, 0d, 20)
  * Blue: printable (21-7e)
  * yellow: nonprintable
  * gray: NUL
* hexxy https://github.com/sweetbbak/hexxy
  * 256 different colors
* xxd 
  * some colorization
* xd https://bitbucket.org/delan/xd/src/default/
  * gray & whilte
  * prints ALL characters!
  * xd --example
* hexyl https://github.com/sharkdp/hexyl
  * looks powerful, and beautiful!

## Number of bytes written when printing a random binary

Printing a 181 MB executable, tested with e.g. `time hexler --stdout filename >/dev/null` or `|wc -c`

| MB to console | Runtime | app      | sampe output                                                                       |
|--------------:|--------:|----------|------------------------------------------------------------------------------------|
|          2004 |   2.210 | hexler   | `00004e80:  74 72 45 76 00 5f 5a 4e  53 74 38 69 6f 73 5f 62  trEv._ZNSt8ios_b`    |
|          2729 |   1.002 | hastyhex | `00004e80  74 72 45 76 00 5f 5a 4e  53 74 38 69 6f 73 5f 62  trEv._ZNSt8ios_b`     |
|          2745 |  24.213 | hexyl    | `│00004e80│ 74 72 45 76 00 5f 5a 4e ┊ 53 74 38 69 6f 73 5f 62 │trEv⋄_ZN┊St8ios_b│` |
|          4195 |   4.371 | hexxy    | `0004e80: 7472 4576 005f 5a4e 5374 3869 6f73 5f62  trEv._ZNSt8ios_b`               |
|          4757 |   5.354 | xxd      | `00004e80: 7472 4576 005f 5a4e 5374 3869 6f73 5f62  trEv._ZNSt8ios_b`              |
|          8598 |  24.213 | tscd     | `00004e80:  74 72 45 76 00 5f 5a 4e 53 74 38 69 6f 73 5f 62  trEv·_ZNSt8ios_b`     |
