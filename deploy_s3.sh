#!/bin/bash

BUCKET=colebrokamp.com

aws s3 sync _site/ s3://$BUCKET

# http://colebrokamp.com.s3-website-us-east-1.amazonaws.com/