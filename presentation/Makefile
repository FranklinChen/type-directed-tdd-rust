# Use Beamer for slides, Minted for code highlighting.
# Minted requires Pygments: http://pygments.org/ 

#PDFLATEX = pdflatex
PDFLATEX = latexmk -pdf

all:	presentation.pdf handout.pdf article.pdf feedback.pdf

article.pdf:	slides.tex

handout.pdf:	slides.tex

presentation.pdf:	slides.tex

%.pdf:	%.tex
	$(PDFLATEX) -shell-escape $*.tex

clean:
	rm -rf *.iml *.aux *.log *.nav *.out *.snm *.toc *.vrb *.pyg *.fls *_latexmk .minted*

.PHONY:	all clean
