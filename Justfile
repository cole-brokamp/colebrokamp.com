# render all content and view website
all: render_cv render_biosketch render_website
  open docs/index.html

# render website
render_website:
  Rscript -e "rmarkdown::render_site('encoding' = 'UTF-8')"

# render biosketch
render_biosketch:
  #!/usr/bin/env Rscript
  rmarkdown::render(
    "src/nih_biosketch.Rmd",
    output_format = "word_document",
    output_file = "../out/nih-biosketch-brokamp.docx"
  )
  
# render CV
render_cv:
  #!/usr/bin/env Rscript
  rmarkdown::render(
    "src/cole-brokamp-cv.Rmd",
    output_format = "word_document",
    output_file = "../out/peds-cv-brokamp.docx"
  )

# snapshot with a git commit
snapshot:
  git add .
  git commit -m "updated `date +'%Y-%m-%d %H:%M:%S'`"
