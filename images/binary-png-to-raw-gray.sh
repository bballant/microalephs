COUNTER=1
for file in ./*.png
do
  convert "$file" -depth 1 "$COUNTER.gray"
  COUNTER=$[$COUNTER +1]
done
