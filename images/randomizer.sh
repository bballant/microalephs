COUNTER=1
for file in ./*.png
do
  cp -- "$file" "rand/$RANDOM-$COUNTER.png"
  #convert "$file" -depth 1 "`printf %03d $COUNTER`.gray"
  COUNTER=$[$COUNTER +1]
done
