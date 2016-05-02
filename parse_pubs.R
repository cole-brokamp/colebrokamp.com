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
  if (is.numeric(y$year)) out <- paste0(out,y$journal,'. ',y$issue,':',y$pages,', ',y$year,'. ')
  if (is.character(y$download_link)) out <- paste0(out,'*[Download](',y$download_link,')*. ')
  if (is.character(y$note)) out <- paste0(out,'*',y$note,'*. ')
  out <- gsub(bold.name,paste0('**',bold.name,'**'),out)
  subsection <- y$note
  if (is.null(subsection)) subsection <- y$year
  if (is.null(subsection)) subsection <- NA
  return(list('text'=out,'subsection'=subsection))
}

pubs.parsed <- lapply(pubs.yaml,yaml_paste_bib,bold.name='Cole Brokamp')

# make tex file

if (file.create('pubs.md')) cat(paste(sapply(pubs.parsed,function(x) x$text),collapse='\n\n'),file='pubs.md')
system('pandoc -o pubs.tex pubs.md')


# create subsections and make html file
subsections <- sapply(pubs.parsed,function(x) x[['subsection']])
subsection.labels <- unique(subsections)

make_subsection <- function(subsection.label){
  subsection.which <- which(subsections == subsection.label)
  subsection.text <- lapply(pubs.parsed[subsection.which],function(x) x[['text']])
  return(paste(c(paste('\n\n####',subsection.label),subsection.text),collapse='\n\n'))
}

if (file.create('pubs.md')) invisible(sapply(subsection.labels,
                                             function(x) cat(make_subsection(x),file='pubs.md',append=TRUE)))

system('pandoc -o pubs.html pubs.md')

