# Code Verification in Rust with Verus

## Files

- `presentation.pdf`: audience slides.
- `presentation.tex`: shared preamble, title slide, and section inputs.
- `sections/*.tex`: one editable file per section, including its slides, speaker notes, and references.
- `presentation-notes.pdf`: slides interleaved with speaker notes.
- `examples/counter.rs`: complete Verus example (contract, callers, loop invariant).
- `examples/counter-bug.smt2` and `examples/counter-fixed.smt2`: standalone Z3 demos.

## Editing individual sections

Edit the relevant file, then run `make` from the project root:

- `sections/introduction.tex`
- `sections/what-is-code-verification.tex`
- `sections/z3.tex`
- `sections/verus.tex`
- `sections/verification-as-llm-guardrails.tex`

Section files are loaded with `\input` and share the preamble in `presentation.tex`; compile the main document rather than a section file directly. Section order is controlled by the input lines in `presentation.tex`. Changes to section files trigger a rebuild.

## Build

Requires a TeX distribution with Beamer, listings, TikZ, Latin Modern, and latexmk; no custom theme or shell escape is needed.

```sh
make          # audience PDF
make notes    # interleaved speaker notes
make demos    # requires z3 on PATH
make verify   # requires verus on PATH
```

Set the author and date in `presentation.tex` if desired. Notes are hidden in the audience PDF. The title slide has no note page, so the notes edition has 55 pages.

## Sources

- [Verus Tutorial and Reference](https://verus-lang.github.io/verus/guide/)
- [Verus source and setup](https://github.com/verus-lang/verus)
- [Microsoft Z3 Guide](https://microsoft.github.io/z3guide/)

The LLM workflow slides propose an engineering acceptance policy; they do not claim that Verus automatically enforces repository permissions or validates the intent of a specification.
