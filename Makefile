all: site cole-brokamp-cv.pdf cole-brokamp-cv-peds-format.docx

site: index.Rmd pubs.yaml talks.yaml
		R -e "rmarkdown::render_site(encoding = 'UTF-8')"
		open docs/index.html

cole-brokamp-cv.pdf: cole-brokamp-cv.Rmd pubs.yaml talks.yaml
		R -e "rmarkdown::render('cole-brokamp-cv.Rmd', output_format = 'pdf_document')"
		open cole-brokamp-cv.pdf

cole-brokamp-cv-peds-format.docx: cole-brokamp-cv-peds-format.Rmd pubs.yaml talks.yaml reference.docx
		R -e "rmarkdown::render('cole-brokamp-cv-peds-format.Rmd', output_format = 'word_document')"
		open cole-brokamp-cv-peds-format.docx



