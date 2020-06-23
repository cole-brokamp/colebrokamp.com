.PHONY: all site clean

all: cole-brokamp-cv.pdf site clean

site: pubs.md talks.md cole-brokamp-cv.pdf
		R -e "rmarkdown::render_site(encoding = 'UTF-8')"
		cp cole-brokamp-cv.pdf docs/cv.pdf
		open docs/index.html

pubs.md pubs.tex talks.md talks.tex: pubs.yaml talks.yaml
		R CMD BATCH --vanilla parse.R

cole-brokamp-cv.pdf: pubs.tex talks.tex cole-brokamp-cv.tex
		texfot pdflatex cole-brokamp-cv.tex
		open cole-brokamp-cv.pdf

clean:
		rm cole-brokamp-cv.log
		rm parse.Rout
		rm talks.md talks.tex publications.md pubs.tex


