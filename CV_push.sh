# add and commit all for CV repo
git add .
echo 'Enter the commit message:'
read commitMessage
git commit -m "$commitMessage"
git push

# copy cv to website folder
cp cole-brokamp-cv.pdf /Users/cole/Library/Mobile\ Documents/com~apple~CloudDocs/CV_website_more/colebrokamp.com/cv.pdf



# move to website folder; build and push to website repo
cd /Users/cole/Library/Mobile\ Documents/com~apple~CloudDocs/CV_website_more/colebrokamp.com
R -e "rmarkdown::render_site(encoding = 'UTF-8')"

git add .
git commit -m "$commitMessage"
git push

# open site to double check the updates
open http://colebrokamp.com
