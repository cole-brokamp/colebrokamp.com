# extract pubs from yaml file into html and tex files
R CMD BATCH --vanilla parse_pubs.R

# then latex the CV into a pdf
"/usr/texbin/texfot" "/usr/texbin/pdflatex" cole-brokamp-cv.tex

# clean up
rm parse_pubs.Rout
# rm cole-brokamp-cv.{log,synctex.gz}

# open cv
open cole-brokamp-cv.pdf
