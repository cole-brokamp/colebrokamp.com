# render all content and view website
all: render_cv render_website
    open docs/index.html

# render website
render_website:
    Rscript -e "rmarkdown::render_site('encoding' = 'UTF-8')"

# render CV
render_cv:
    #!/usr/bin/env Rscript
    rmarkdown::render(
      "src/peds-cv.Rmd",
      output_format = "word_document",
      output_file = "../out/peds-cv-brokamp.docx"
    )

# snapshot with a git commit
snapshot:
    git add .
    git commit -m "updated `date +'%Y-%m-%d %H:%M:%S'`"
