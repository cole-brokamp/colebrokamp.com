library(magrittr)
library(purrr)
library(glue)
library(dplyr)
library(tidyr)

## publications
pubs <-
  yaml::yaml.load_file("pubs.yaml") %>%
  tibble::enframe()

pubs$year <- map_chr(pubs$value, "year")

make_pubs_md <- function(x = pubs[[1]]) {
  md <- with(x, glue("{author}. {title}. *{journal}*. {issue_pages}. {year}."))
  if (! x$download_link == "") md <- glue("{md} [*Download*]({x$download_link}).")
  md <- gsub("Cole Brokamp", "**Cole Brokamp**", md)
  return(md)
}

pubs$md <- map_chr(pubs$value, make_pubs_md)

# create md file by year
file.create("publications.md")

pubs_years <-
  pubs %>%
  group_by(year) %>%
  nest() %>%
  arrange(desc(year)) %>%
  rowwise()

pubs_years$md <- map_chr(pubs_years$data, ~ paste(.$md, collapse = "\n\n"))

paste(glue("\n\n\n### {pubs_years$year}\n\n\n"), pubs_years$md) %>%
  walk(cat, file = "publications.md", append = TRUE)

# create tex file
temp_file <- glue(tempfile(), ".md")
cat(paste(pubs$md, collapse = "\n\n"), file = temp_file)
system(glue("pandoc -o pubs.tex {temp_file}"))

## talks

talks <-
  yaml::yaml.load_file("talks.yaml") %>%
  tibble::enframe()

talks$year <- map_chr(talks$value, "year")

make_talk_md <- function(x) {
  md <- with(x, glue("**{title}**. *{event}*. {location}. {year}."))
  if (! x$download_link == "") md <- glue("{md} [*Download*]({x$download_link}).")
  return(md)
}

talks$md <- map_chr(talks$value, make_talk_md)

# create md file by year
file.create("talks.md")

talks_years <-
  talks %>%
  group_by(year) %>%
  nest() %>%
  arrange(desc(year)) %>%
  rowwise()

talks_years$md <- map_chr(talks_years$data, ~ paste(.$md, collapse = "\n\n"))

paste(glue("\n\n\n### {talks_years$year}\n\n\n"), talks_years$md) %>%
  walk(cat, file = "talks.md", append = TRUE)

# create tex file
temp_file <- glue(tempfile(), ".md")
cat(paste(talks$md, collapse = "\n\n"), file = temp_file)
system(glue("pandoc -o talks.tex {temp_file}"))

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
