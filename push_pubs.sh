# add updated pubs files and commit
git add .
echo 'Enter the commit message:'
read commitMessage
git commit -m "$commitMessage"
git push

# pubs.html is symmlinked to website directory
# change to this directory; change html and push new site
cd ~/Documents/cole-brokamp.github.io
cp index.html index.html.backup

cat index_pre.html > index.html
cat /Users/cole/Documents/CV\ and\ Cover\ Letters\ and\ Applications/CV/pubs.html >> index.html
cat index_post.html >> index.html

git add .
git commit -m "$commitMessage"
git push

open http://colebrokamp.com