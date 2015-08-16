# first pandoc the markdown file into html and tex
pandoc publications.md -o publications.html
pandoc -s publications.md -o publications.tex
# pandoc publications.md --latex-engine=xelatex -o publications.pdf

# then extract only the body of the .tex file to file to include in CV latex document
sed -n '/begin{document}/,/end{document}/p' publications.tex | sed '1d' | sed '$d' > publications-content.tex

# then latex the CV into a pdf
"/usr/texbin/pdflatex" -synctex=1 -interaction=nonstopmode cole-brokamp-cv.tex

# clean up
rm cole-brokamp-cv.{log,synctex.gz}
