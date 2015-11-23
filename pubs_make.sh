# add new pubs by hand to cole_brokamp_publications.bib

# latex into cole_brokamp_publications.pdf
/usr/texbin/pdflatex -synctex=1 -interaction=nonstopmode cole_brokamp_publications.tex
/usr/texbin/bibtex cole_brokamp_publications.aux
/usr/texbin/pdflatex -synctex=1 -interaction=nonstopmode cole_brokamp_publications.tex
/usr/texbin/pdflatex -synctex=1 -interaction=nonstopmode cole_brokamp_publications.tex

# clean up
rm cole_brokamp_publications.{log,blg,aux,bbl,synctex.gz}


# copy paste to markdown file
echo " "
echo " "
echo " "
echo "**********"
echo "don't forget to copy and paste the PDF refs into the markdown file"
echo "make style changes (bold name) and add links to document from website"
echo "then rerun pubs_make.sh script"
echo "**********"
echo " "
echo " "
echo " "
