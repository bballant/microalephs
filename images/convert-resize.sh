COUNTER=1
for file in *.png
do
  convert $file -resize 64x128^ -gravity center -crop 64x128+0+0 +repage ../64x128/$file
  COUNTER=$[$COUNTER +1]
done
