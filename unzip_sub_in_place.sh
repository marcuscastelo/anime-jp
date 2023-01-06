file=$1
echo "Unzipping file: '$file'"

# Unzip the file in place.
unzip -o "$file" -d "$(dirname "$file")"