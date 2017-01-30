# extract info from yaml file into md and tex files
R CMD BATCH --vanilla parse_pubs.R
R CMD BATCH --vanilla parse_talks.R
R CMD BATCH --vanilla parse_software.R

# then latex the CV into a pdf
texfot pdflatex cole-brokamp-cv.tex

# clean up
rm *.Rout
# rm cole-brokamp-cv.{log,synctex.gz}

# open cv
open cole-brokamp-cv.pdf
