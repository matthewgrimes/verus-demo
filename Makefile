.PHONY: all clean demos verify notes
all: presentation.pdf
presentation.pdf: presentation.tex $(wildcard sections/*.tex)
	latexmk -pdf -interaction=nonstopmode -halt-on-error presentation.tex
notes:
	pdflatex -interaction=nonstopmode -halt-on-error -jobname=presentation-notes '\AtBeginDocument{\setbeameroption{show notes}}\input{presentation.tex}'
	pdflatex -interaction=nonstopmode -halt-on-error -jobname=presentation-notes '\AtBeginDocument{\setbeameroption{show notes}}\input{presentation.tex}'
demos:
	z3 examples/counter-bug.smt2
	z3 examples/counter-fixed.smt2
verify:
	verus examples/counter.rs
clean:
	latexmk -c presentation.tex
	latexmk -c presentation-notes.tex
