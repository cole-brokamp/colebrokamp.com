# render all content and view website
all: render_cv render_peds_cv render_biosketch render_website
  open docs/index.html

# render website
render_website:
  Rscript -e "rmarkdown::render_site('encoding' = 'UTF-8')"

# render CV
render_cv:
  #!/usr/bin/env Rscript
  rmarkdown::render(
    "_cole-brokamp-cv.Rmd",
    output_format = "pdf_document",
    output_file = "cole-brokamp-cv.pdf"
  ) |>
  browseURL()

# render UC Dept of Pediatrics formatted CV
render_peds_cv:
  #!/usr/bin/env Rscript
  rmarkdown::render(
    "_cole-brokamp-cv-peds-format.Rmd",
    output_format = "word_document",
    output_file = "cole-brokamp-cv-peds-format.docx"
  ) |>
  browseURL()

# download (in R) and edit this file for personal details:
## download.file("https://grants.nih.gov/grants/forms/biosketch-blank-format-rev-10-2021.docx", "biosketch_stub.docx")

# render biosketch in word
render_biosketch_word:
    pandoc \
    test.md \
    -o biosketch_stub.docx \
    --output="cole-brokamp-nih-biosketch.docx"

# render NIH-formatted biosketch
render_biosketch:
  #!/usr/bin/env Rscript
  rmarkdown::render(
    "_cole-brokamp-nih-biosketch.Rmd",
    output_format = "pdf_document",
    output_file = "cole-brokamp-nih-biosketch.pdf"
  ) |>
  browseURL()


# snapshot with a git commit
snapshot:
  git add .
  git commit -m "updated `date +'%Y-%m-%d %H:%M:%S'`"
