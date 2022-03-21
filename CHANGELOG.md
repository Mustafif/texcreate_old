## 2022-03-21, Version v2.0.0 - Beta 1
### Commits
- [`a6a25eeb52`](http://github.com/MKProj/texcreate.git/commit/a6a25eeb5282a7d7b2310d5cd272d4e9237a8caa) Update (Mustafif)
- [`6d04615633`](http://github.com/MKProj/texcreate.git/commit/6d046156331a7810a4a7804d5c6aec7bf6f14781) Added desc and license (Mustafif)
- [`805a71cb61`](http://github.com/MKProj/texcreate.git/commit/805a71cb61cacaeb1ad583f633f2764d13c22aae) Added desc and license (Mustafif)
- [`16e521f383`](http://github.com/MKProj/texcreate.git/commit/16e521f38325f55ae691a509513923396e70c5a6) Update (Mustafif)
- [`69fd1c5b92`](http://github.com/MKProj/texcreate.git/commit/69fd1c5b926f835f431228393d92aec26fcf1f17) Fix small issue (Mustafif)
- [`346ca4c7b3`](http://github.com/MKProj/texcreate.git/commit/346ca4c7b32b641a45dcb21160dc7b18daeb57df) Fix small issue (Mustafif)
- [`08932f65f2`](http://github.com/MKProj/texcreate.git/commit/08932f65f28fb3108bab05fee40aeda155ad91c5) Added desc and license (Mustafif)
- [`d7c1991c9b`](http://github.com/MKProj/texcreate.git/commit/d7c1991c9b0112703a52db2104a47589b1fa200c) Update (Mustafif)
- [`ceb3e95fc0`](http://github.com/MKProj/texcreate.git/commit/ceb3e95fc0a7abd89e9e9228b89fd8777633385c) Small fix (Mustafif)
- [`c091475247`](http://github.com/MKProj/texcreate.git/commit/c0914752476c78dd24c59eb08523bf4b2e2325ad) Changed tex_rs::element::* to tex_rs::*; (Mustafif)
- [`285a6e5e43`](http://github.com/MKProj/texcreate.git/commit/285a6e5e4326860886e3d94ad41c792d580b5265) Updated workspace depenedency to versions (Mustafif)
- [`4aef34c1af`](http://github.com/MKProj/texcreate.git/commit/4aef34c1af1924cd5f4e367a285bd0638fcfc2d2) (cargo-release) version 2.0.0 (Mustafif)
- [`477b939415`](http://github.com/MKProj/texcreate.git/commit/477b9394159fc12b06cb86b92b1ef5cb62d1b23d) Fixed version on texc-config and texc-utils (Mustafif)
- [`ebb6099d34`](http://github.com/MKProj/texcreate.git/commit/ebb6099d344d53a8d486c6aef4168bd387829462) (cargo-release) version 2.0.0 (Mustafif)
- [`d589c332ec`](http://github.com/MKProj/texcreate.git/commit/d589c332ec15fb607dafa9dd6845a2b9c7309b2b) Update (Mustafif)
- [`a98e7ed058`](http://github.com/MKProj/texcreate.git/commit/a98e7ed058f14fc0e6544f475d5226ed51b23af7) (cargo-release) version 2.0.0 (Mustafif)
- [`3a695942f0`](http://github.com/MKProj/texcreate.git/commit/3a695942f043c4bae303e5d990620ba7079c2fdc) Changed README to follow v2 info (Mustafif)
- [`9f0df1a5f5`](http://github.com/MKProj/texcreate.git/commit/9f0df1a5f5738b9d5b3438232293882f02cbfb60) Updated changes (Mustafif)
- [`80e368aed3`](http://github.com/MKProj/texcreate.git/commit/80e368aed3f186f335de6c3e6072356f73fe49c4) Changes for texcreate v2.0.0-beta.1 (Mustafif)

### Stats
```diff
 .github/workflows/main.yml               |   21 +-
 .gitignore                               |    7 +-
 .idea/misc.xml                           |   18 +-
 .idea/modules.xml                        |    3 +-
 .idea/texc_rev2.iml                      |   20 +-
 .idea/texcreate.iml                      |   14 +-
 .idea/texcreate2.iml                     |    2 +-
 Basic/Basic.tex                          |   16 +-
 Basic/structure.tex                      |    2 +-
 CHANGELOG.md                             |   26 +-
 Cargo.lock                               | 1059 ++++++++---
 Cargo.toml                               |   31 +-
 Code/Code.tex                            |   28 +-
 Code/structure.tex                       |   30 +-
 Makefile                                 |   21 +-
 PLAN.md                                  |   11 +-
 README.md                                |  166 +--
 Templates/Basic/main.aux                 |    2 +-
 Templates/Basic/main.fdb_latexmk         |   38 +-
 Templates/Basic/main.fls                 |  118 +-
 Templates/Basic/main.log                 |  118 +-
 Templates/Basic/main.pdf                 | Bin 9011 -> 0 bytes
 Templates/Basic/main.synctex.gz          | Bin 559 -> 0 bytes
 Templates/Basic/main.tex                 |   16 +-
 Templates/Basic/structure.tex            |    2 +-
 Templates/Beamer/main.aux                |   40 +-
 Templates/Beamer/main.fdb_latexmk        |  193 +--
 Templates/Beamer/main.fls                | 1267 +-------------
 Templates/Beamer/main.log                |  733 +-------
 Templates/Beamer/main.nav                |   19 +-
 Templates/Beamer/main.out                |    2 +-
 Templates/Beamer/main.pdf                | Bin 47466 -> 0 bytes
 Templates/Beamer/main.snm                |    0
 Templates/Beamer/main.synctex.gz         | Bin 9251 -> 0 bytes
 Templates/Beamer/main.tex                |   38 +-
 Templates/Beamer/main.toc                |    2 +-
 Templates/Beamer/structure.tex           |    7 +-
 Templates/Code/main.aux                  |    3 +-
 Templates/Code/main.fdb_latexmk          |   72 +-
 Templates/Code/main.fls                  |  337 +---
 Templates/Code/main.log                  |  225 +--
 Templates/Code/main.pdf                  | Bin 53078 -> 0 bytes
 Templates/Code/main.synctex.gz           | Bin 3927 -> 0 bytes
 Templates/Code/main.tex                  |   28 +-
 Templates/Code/main.toc                  |    1 +-
 Templates/Code/structure.tex             |   30 +-
 Templates/Math/.aux                      |   19 +-
 Templates/Math/main.aux                  |    2 +-
 Templates/Math/main.fdb_latexmk          |   62 +-
 Templates/Math/main.fls                  |  256 +---
 Templates/Math/main.log                  |  276 +---
 Templates/Math/main.pdf                  | Bin 18833 -> 0 bytes
 Templates/Math/main.synctex.gz           | Bin 900 -> 0 bytes
 Templates/Math/main.tex                  |   19 +-
 Templates/Math/main.toc                  |    0
 Templates/Math/structure.tex             |   78 +-
 Templates/Novel/main.aux                 |    6 +-
 Templates/Novel/main.log                 |  147 +-
 Templates/Novel/main.pdf                 | Bin 43396 -> 0 bytes
 Templates/Novel/main.tex                 |   22 +-
 Templates/Novel/main.toc                 |    1 +-
 Templates/Novel/structure.tex            |   10 +-
 Templates/Theatre/main.aux               |    4 +-
 Templates/Theatre/main.fdb_latexmk       |   41 +-
 Templates/Theatre/main.fls               |  119 +-
 Templates/Theatre/main.log               |  171 +--
 Templates/Theatre/main.pdf               | Bin 30673 -> 0 bytes
 Templates/Theatre/main.synctex.gz        | Bin 1192 -> 0 bytes
 Templates/Theatre/main.tex               |   30 +-
 Templates/Theatre/main.toc               |    2 +-
 Templates/Theatre/structure.tex          |   11 +-
 book.toml                                |    7 +-
 config.toml                              |   28 +-
 config_plan.toml                         |   12 +-
 doc/SUMMARY.md                           |   15 +-
 doc/adding_new_templates.md              |  155 +--
 doc/build.md                             |   21 +-
 doc/code_template_example.md             |  103 +-
 doc/create.md                            |   13 +-
 doc/edit.md                              |    2 +-
 doc/init.md                              |    1 +-
 doc/intro.md                             |   54 +-
 doc/list.md                              |    1 +-
 doc/pics/code-example.png                | Bin 15948 -> 0 bytes
 doc/pics/test.exe                        | Bin 1927680 -> 0 bytes
 doc/pics/test.go                         |   18 +-
 doc/web.md                               |    1 +-
 index.html                               |   76 +-
 list.json                                |   34 +-
 matex/Cargo.toml                         |    8 +-
 matex/src/lib.rs                         |    8 +-
 project/README.md                        |   24 +-
 project/out/project.aux                  |    3 +-
 project/out/project.log                  |  259 +++-
 project/out/project.pdf                  | Bin 0 -> 58733 bytes
 project/src/project.tex                  |   30 +-
 project/src/structure.tex                |   30 +-
 project/texcreate.toml                   |    2 +-
 src/list.rs                              |   60 +-
 src/main.rs                              |  461 +-----
 tex-rs/Cargo.toml                        |   14 +-
 tex-rs/src/element.rs                    |  446 ++++-
 tex-rs/src/error.rs                      |   26 +-
 tex-rs/src/latex.rs                      |  398 ++++-
 tex-rs/src/lib.rs                        |   57 +-
 tex-rs/src/traits/attatch.rs             |   55 +-
 tex-rs/src/traits/convert.rs             |    6 +-
 tex-rs/src/traits/mod.rs                 |    8 +-
 texc-config/Cargo.toml                   |   18 +-
 texc-config/src/error.rs                 |  119 +-
 texc-config/src/extra.rs                 |   82 +-
 texc-config/src/lib.rs                   |  182 ++-
 texc-core/Cargo.toml                     |    8 +-
 texc-core/src/lib.rs                     |    8 +-
 texc-error/Cargo.toml                    |   12 +-
 texc-error/src/lib.rs                    |  110 +-
 texc-latex/Cargo.toml                    |   10 +-
 texc-latex/src/lib.rs                    |    2 +-
 texc-latex/src/templates/basic.rs        |   28 +-
 texc-latex/src/templates/beamer.rs       |   11 +-
 texc-latex/src/templates/code.rs         |   72 +-
 texc-latex/src/templates/mod.rs          |   22 +-
 texc-utils/Cargo.toml                    |   12 +-
 texc-utils/src/lib.rs                    |  237 ++-
 texc-web/Cargo.toml                      |    8 +-
 texc-web/src/lib.rs                      |    8 +-
 texcreate.html                           |   76 +-
 texcreate_lib/Cargo.lock                 | 3097 +-------------------------------
 texcreate_lib/Cargo.toml                 |   22 +-
 texcreate_lib/src/Config/config.rs       |  181 +--
 texcreate_lib/src/Config/config_multi.rs |  188 +--
 texcreate_lib/src/Config/consts.rs       |   45 +-
 texcreate_lib/src/Config/error.rs        |   79 +-
 texcreate_lib/src/Config/mod.rs          |   11 +-
 texcreate_lib/src/Config/template.rs     |   29 +-
 texcreate_lib/src/Templates/basic.rs     |   21 +-
 texcreate_lib/src/Templates/beamer.rs    |   49 +-
 texcreate_lib/src/Templates/book.rs      |   38 +-
 texcreate_lib/src/Templates/code.rs      |   61 +-
 texcreate_lib/src/Templates/lachaise.rs  |  394 +----
 texcreate_lib/src/Templates/math.rs      |   40 +-
 texcreate_lib/src/Templates/mod.rs       |   17 +-
 texcreate_lib/src/Templates/novel.rs     |   36 +-
 texcreate_lib/src/Templates/theatre.rs   |   40 +-
 texcreate_lib/src/Web/mod.rs             |    4 +-
 texcreate_lib/src/Web/tc.rs              |  137 +-
 texcreate_lib/src/Web/web.rs             |   18 +-
 texcreate_lib/src/lib.rs                 |    8 +-
 texcreate_lib/src/routes.rs              |  137 +-
 tmp/Project1/Project1.tex                |   19 +-
 tmp/Project1/structure.tex               |   19 +-
 tmp/Project2/Project2.tex                |   38 +-
 tmp/Project2/structure.tex               |    7 +-
 tmp/config.toml                          |   26 +-
 154 files changed, 3328 insertions(+), 10936 deletions(-)
```


## 2022-01-24, Version v1.2.0
### Commits
- [`bd14e01467`](http://github.com/MKProj/texcreate.git/commit/bd14e0146769eecaa4aae80c042c7a8a8d3be656) Update (Mustafif)
- [`f24633760c`](http://github.com/MKProj/texcreate.git/commit/f24633760c98955eb307196bdd53082d597ecdbd) Update (Mustafif)
- [`2d974d4557`](http://github.com/MKProj/texcreate.git/commit/2d974d45570282ccaa06eddda3b211a9d5b57880) Update (Mustafif)

### Stats
```diff
 Cargo.lock                               |   4 +-
 Cargo.toml                               |   4 +-
 Makefile                                 |   2 +-
 Project/Project.tex                      |  16 +--
 Project/structure.tex                    |   2 +-
 config.toml                              |  23 +++-
 src/main.rs                              |  69 +++++------
 texcreate_lib/Cargo.lock                 |   2 +-
 texcreate_lib/Cargo.toml                 |   2 +-
 texcreate_lib/src/Config/config.rs       |   2 +-
 texcreate_lib/src/Config/config_multi.rs | 210 +++++++++++++++-----------------
 texcreate_lib/src/Config/error.rs        |   2 +-
 texcreate_lib/src/Web/tc.rs              |   7 +-
 texcreate_lib/src/Web/web.rs             |   6 +-
 14 files changed, 169 insertions(+), 182 deletions(-)
```


