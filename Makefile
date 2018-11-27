all: site cv

site:
		R -e "rmarkdown::render_site(encoding = 'UTF-8')"
		open docs/index.html

cv:
		cp ../CV/cole-brokamp-cv.pdf cv.pdf
