The PDF version (and tex source) of my CV.

In order to create CV:

- add new publications to `cole_brokamp_publications.bib`
- run `pubs_make.sh`
- copy/paste the entries to `publications.md` file
- make style changes (bold name) and add links to PDFs (after updating in publications folder on 
website)
- run `cv_make.sh`
    - this will auto convert the md file into tex and retain only the content
    - this will also latex the actual CV, which includes a pointer to the new publications
- manually paste in the html version of publications into index.html for website by year
