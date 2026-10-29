# Code Verification in Rust with Verus

A 28-slide, widescreen Beamer talk for a graduate mathematics department, designed for approximately 30–35 minutes plus questions. Assumes comfort with mathematical proofs, quantifiers, and induction; no Rust or formal-methods background is required. Covers the five requested sections: Introduction; What is code verification; Z3; Verus; Verification as LLM Guardrails.

The mathematical thread is: a specification as a theorem, execution as a relation, Hoare triples and substitution, satisfiability relative to a theory, and loop invariants as induction. The final section treats LLM output as a proposed construction whose theorem and hypotheses must remain fixed.

## Files

- `presentation.pdf`: audience slides.
- `presentation.tex`: editable source, including speaker notes and clickable primary-source references.
- `presentation-notes.pdf`: slides interleaved with speaker notes.
- `examples/counter.rs`: complete Verus example (contract, callers, loop invariant).
- `examples/counter-bug.smt2` and `examples/counter-fixed.smt2`: standalone Z3 demos.

## Build

Requires a TeX distribution with Beamer, listings, TikZ, Latin Modern, and latexmk; no custom theme or shell escape is needed.

```sh
make          # audience PDF
make notes    # interleaved speaker notes
make demos    # requires z3 on PATH
make verify   # requires verus on PATH
```

Set the author and date in `presentation.tex` if desired. Notes are hidden in the audience PDF. The title slide has no note page, so the notes edition has 55 pages.

## Demo narrative

1. Define the bounded successor mathematically and exhibit the boundary counterexample. Introduce Rust syntax only in the Verus section.
2. Run `z3 examples/counter-bug.smt2`: expect `sat` and a boundary witness.
3. Run `z3 examples/counter-fixed.smt2`: expect `unsat` for a bound violation.
4. With Verus installed, run `verus examples/counter.rs`.
5. For a negative demonstration, change only the body of `advance` from `<` to `<=`; keep the contract fixed. Verification should fail. Restore the body afterward.

The SMT scripts model mathematical integer arithmetic with bounded inputs. They demonstrate the bound obligation, not a full encoding of Rust execution. The Verus example additionally specifies the exact result and asks the verifier to check executable arithmetic.

## Validation

- Audience PDF compiled successfully with TeX Live 2026 / latexmk.
- Speaker-notes PDF compiled successfully.
- No overfull box warnings in either final TeX log.
- Both Z3 scripts executed: buggy `sat`, fixed `unsat`.
- Verus is not installed in the build environment. The Rust/Verus example was checked against the official guide but has **not been run through Verus locally**. Run `make verify` with a compatible Verus release before a live demo.

## Sources

- [Verus Tutorial and Reference](https://verus-lang.github.io/verus/guide/)
- [Verus source and setup](https://github.com/verus-lang/verus)
- [Microsoft Z3 Guide](https://microsoft.github.io/z3guide/)

The LLM workflow slides propose an engineering acceptance policy; they do not claim that Verus automatically enforces repository permissions or validates the intent of a specification.
