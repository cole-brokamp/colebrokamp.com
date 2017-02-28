#!/usr/bin/Rscript

library(magrittr)

# read in yaml and parse yaml file
input <-
  readLines('pubs.yaml') %>%
  paste(collapse='\n')

pubs <-
  strsplit(input,'---',fixed=TRUE)[[1]] %>%
  tail(-1)

pubs.yaml <- lapply(pubs,yaml::yaml.load)

yaml_paste_bib <- function(y,bold.name){
  out <- paste0(y$author,'. ',y$title,'. ')
  if (!is.null(y$journal)) out <- paste0(out,y$journal,'. ')
  if (!is.null(y$issue)) out <- paste0(out,y$issue,'. ')
  if (!is.null(y$pages)) out <- paste0(out,y$pages,'. ')
  if (!is.null(y$year)) out <- paste0(out,y$year,'. ')
  if (!is.null(y$download_link)) out <- paste0(out,'*[Download](',y$download_link,')*. ')
  if (!is.null(y$note)) out <- paste0(out,'*',y$note,'*. ')
  out <- gsub(bold.name,paste0('**',bold.name,'**'),out)
  subsection <- y$note
  if (is.null(subsection)) subsection <- y$year
  if (is.null(subsection)) subsection <- NA
  return(list('text'=out,'subsection'=subsection))
}

pubs.parsed <- lapply(pubs.yaml,yaml_paste_bib,bold.name='Cole Brokamp')

# make tex file for CV

if (file.create('pubs.md')) cat(paste(sapply(pubs.parsed,function(x) x$text),collapse='\n\n'),file='pubs.md')
system('pandoc -o pubs.tex pubs.md')

# create subsections and make md file
subsections <- sapply(pubs.parsed,function(x) x[['subsection']])
subsection.labels <- unique(subsections)
subsection.labels <- sort(subsection.labels,decreasing=TRUE)

make_subsection <- function(subsection.label){
  subsection.which <- which(subsections == subsection.label)
  subsection.text <- lapply(pubs.parsed[subsection.which],function(x) x[['text']])
  return(paste(c(paste('\n\n###',subsection.label),subsection.text),collapse='\n\n'))
}



if (file.create('pubs_sections.md')) invisible(sapply(subsection.labels,
                                             function(x) cat(make_subsection(x),file='pubs_sections.md',append=TRUE)))