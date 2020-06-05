library(magrittr)
library(purrr)
library(glue)

## publications
pubs <- yaml::yaml.load_file("pubs.yaml")

make_pubs_md <- function(x) {
  md <- with(x, glue("{author}. {title}. *{journal}*. {issue_pages}. {year}."))
  if (! x$download_link == "") md <- glue("{md} [*Download*]({x$download_link}).")
  gsub("Cole Brokamp", "**Cole Brokamp**", md)
  return(md)
}

pubs_md <- map_chr(pubs, make_pubs_md)

# create tex file
temp_file <- glue(tempfile(), ".md")
cat(paste(pubs_md, collapse = "\n\n"), file = temp_file)
system(glue("pandoc -o pubs.tex {temp_file}"))

# create md file
file.create("pubs.md")
cat(paste(c("# Publications", pubs_md), collapse = "\n\n"), file = "pubs.md")

## talks
talks <- yaml::yaml.load_file("talks.yaml")

make_talk_md <- function(x) {
  md <- with(x, glue("{title}. {event}. {location}. {year}."))
  if (! x$download_link == "") md <- glue("{md} [*Download*]({x$download_link}).")
  return(md)
}

talks_md <- map_chr(talks, make_talk_md)

# create tex file
temp_file <- glue(tempfile(), ".md")
cat(paste(talks_md, collapse = "\n\n"), file = temp_file)
system(glue("pandoc -o talks.tex {temp_file}"))

# create md file
file.create("pubs.md")
cat(paste(c("# Talks", talks_md), collapse = "\n\n"), file = "talks.md")

###

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
