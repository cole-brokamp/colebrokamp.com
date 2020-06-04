all: cv site

site: cv
		R -e "rmarkdown::render_site(encoding = 'UTF-8')"
		cp assets/cole-brokamp-cv.pdf docs/cv.pdf
		open docs/index.html

cv:
		cp ../CV/cole-brokamp-cv.pdf assets/cole-brokamp-cv.pdf
