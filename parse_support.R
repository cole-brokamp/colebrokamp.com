#!/usr/bin/Rscript

library(magrittr)

# read in yaml and parse yaml file
input <-
  readLines('support.yaml') %>%
  paste(collapse='\n')

support <-
  strsplit(input,'---',fixed=TRUE)[[1]] %>%
  tail(-1)

support.yaml <- lapply(support,yaml::yaml.load)

support_paste <- function(y){
  out <- paste0('**', y$number, '**', '  \n',
                '*', y$title, '*', '  \n',
                y$pi, ', PI', ' (', y$dates, ')  \n',
                y$description, '  \n',
                'Role: ', y$role, '  \n')
  return(list('text'=out, 'tag'=y$tag))
}

support_parsed <- lapply(support.yaml, support_paste)

# create subsections and make md file
subsections <- sapply(support_parsed,function(x) x[['tag']])
subsection.labels <- unique(subsections)

make_subsection <- function(subsection.label){
  subsection.which <- which(subsections == subsection.label)
  subsection.text <- lapply(support_parsed[subsection.which],function(x) x[['text']])
  return(paste(c(paste('\n\n###',subsection.label),subsection.text),collapse='\n\n'))
}

# make tex file for CV
sapply(c('Active', 'Pending', 'Complete'), make_subsection) %>%
  cat(file='support.md', append=FALSE)

system('pandoc -o support.tex support.md')
