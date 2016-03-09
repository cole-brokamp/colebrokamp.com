# add new pubs by hand to cole_brokamp_publications.bib

# make my name bold
# sed 's/Cole/\\bf{Cole}/g' cole_brokamp_publications.bib | 
# sed 's/Brokamp/\\bf{Brokamp}/g'  > cole_brokamp_publications_bolded.bib


# pandoc cole_brokamp_publications.md -o test.pdf --bibliography cole_brokamp_publications.bib --csl bib.csl

# latex into cole_brokamp_publications.pdf
/usr/texbin/pdflatex -synctex=1 -interaction=nonstopmode cole_brokamp_publications.tex
/usr/texbin/bibtex cole_brokamp_publications.aux
/usr/texbin/pdflatex -synctex=1 -interaction=nonstopmode cole_brokamp_publications.tex
/usr/texbin/pdflatex -synctex=1 -interaction=nonstopmode cole_brokamp_publications.tex

# clean up
rm cole_brokamp_publications.{log,blg,out,aux,bbl,synctex.gz}


# copy paste to markdown file
echo " "
echo " "
echo " "
echo "**********"
echo "don't forget to copy and paste the PDF refs into the markdown file"
echo "make style changes (bold name)"
echo "then run cv_make.sh script"
echo "**********"
echo " "
echo " "
echo " "
