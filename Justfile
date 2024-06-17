# render all content and view website
all: render_cv render_peds_cv render_website
  open docs/index.html

# render website
render_website:
  Rscript -e "rmarkdown::render_site('encoding' = 'UTF-8')"

# render CV webpage
render_cv_web:
  #!/usr/bin/env Rscript
  rmarkdown::render(
    "src/cole-brokamp-cv.Rmd",
    output_format = "html_document",
    output_file = "../out/cole-brokamp-cv.html"
  ) |>
  browseURL()

# render CV
render_cv:
  #!/usr/bin/env Rscript
  rmarkdown::render(
    "src/cole-brokamp-cv.Rmd",
    output_format = "word_document",
    output_file = "../out/cole-brokamp-cv.docx"
  ) |>
  browseURL()

# render UC Dept of Pediatrics formatted CV
render_peds_cv:
  #!/usr/bin/env Rscript
  rmarkdown::render(
    "src/cole-brokamp-cv.Rmd",
    output_format = "word_document",
    output_file = "../out/cole-brokamp-cv-peds-format.docx"
  ) |>
  browseURL()

# snapshot with a git commit
snapshot:
  git add .
  git commit -m "updated `date +'%Y-%m-%d %H:%M:%S'`"
