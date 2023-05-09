callr::r(\(.) rmarkdown::render_site(
  "encoding" = "UTF-8"))

callr::r(\(.) rmarkdown::render(
  "_cole-brokamp-cv.Rmd",
  output_format = "pdf_document",
  output_file = "cole-brokamp-cv.pdf"))

callr::r(\(.) rmarkdown::render(
  '_cole-brokamp-cv-peds-format.Rmd',
  output_format = 'word_document',
  output_file = "cole-brokamp-cv-peds-format.docx"))

callr::r(\(.) rmarkdown::render(
  "_cole-brokamp-nih-biosketch.Rmd",
  output_format = "pdf_document",
  output_file = "cole-brokamp-nih-biosketch.pdf"))
