# read in yaml file to string
input <- paste(readLines('pubs.yaml'),collapse='\n')
# split on '---'
pubs <- strsplit(input,'---',fixed=TRUE)[[1]]
# remove first and last string
pubs <- head(pubs,-1)
pubs <- tail(pubs,-1)

pubs.yaml <- lapply(pubs,yaml::yaml.load)
pubs.yaml
yaml.load(pubs[[1]][2])

yaml_paste_bib <- function(y,bold.name){
  out <- paste0(y$author,'. ',y$title,'. ')
  if (is.numeric(y$year)) out <- paste0(out,y$journal,'. ',y$issue,':',y$pages,', ',y$year,'. ')
  if (is.character(y$download_link)) out <- paste0(out,'*[Download](',y$download_link,')*. ')
  if (is.character(y$note)) out <- paste0(out,'*',y$note,'*. ')
  out <- gsub(bold.name,paste0('**',bold.name,'**'),out)
  return(out)
}

md.text <- lapply(pubs.yaml,yaml_paste_bib,bold.name='Cole Brokamp')

cat(paste(md.text,collapse='\n\n'),file='test_bib.md')


make_sections <- function(year) {
  md.text.year <- md.text[sapply(md.text,grepl,pattern=year)]
  cat(paste(year,md.text.year,collapse='\n\n'))
}
 
cat(make_sections(2015),file='test_bib.md')

# parse all yaml

# paste entries

# sort by date and make output