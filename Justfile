set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

# render the website, writing, CV, and BibTeX
build: render

# render one target: all, site, writing, bib, or cv
render target="all":
    cargo run --quiet -- {{ quote(target) }}

# render everything, serve the site locally, and open the browser
preview: build
    (sleep 1 && open http://localhost:8000) & python3 -m http.server --directory _site 8000

# snapshot with a git commit
snapshot:
    git add .
    git commit -m "updated `date +'%Y-%m-%d %H:%M:%S'`"
