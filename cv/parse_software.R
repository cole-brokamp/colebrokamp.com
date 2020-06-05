#!/usr/bin/Rscript

library(tidyverse)

# read in yaml and parse yaml file
input <-
  readLines('software.yaml') %>%
  paste(collapse='\n')

software <-
  strsplit(input,'---',fixed=TRUE)[[1]] %>%
  tail(-1)

software.yaml <- lapply(software,yaml::yaml.load)

yaml_paste_software <- function(y) {
  out <- paste0('**','`',y$name,'`**  \n',
                y$description,'.','  \n',
                # 'url: ',
                '[',y$url,']','(',y$url,')')
  # if (!is.null(y$doi)) out <- paste0(out,'  \n','doi: ','[',y$doi,']','(',y$doi,')')
  # if (!is.null(y$vignette)) out <- paste0(out,'  \n','[Vignette](',y$vignette,')')
  # if (!is.null(y$docker)) out <- paste0(out,'  \n','[Docker Image](',y$docker,')')
  return(out)
}

software.parsed <- lapply(software.yaml,yaml_paste_software)

# make tex file
file.create('software.md')
software.parsed %>% 
  paste(collapse='\n\n') %>% 
  cat(file='software.md')

system('pandoc -o software.tex software.md')
