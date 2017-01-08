git add .
echo 'Enter the commit message:'
read commitMessage
git commit -m "$commitMessage"
git push

# in case pubs, talks, or software are changed rebuild website and push to website S3 bucket
R -e \"rmarkdown::render_site(input='/Users/cole/Documents/CV_website_more/colebrokamp.com',
encoding = 'UTF-8')\"


aws s3 sync /Users/cole/Documents/CV_website_more/colebrokamp.com/_site/ s3://colebrokamp.com