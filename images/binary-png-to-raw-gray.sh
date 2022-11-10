COUNTER=1
for file in ../../image-workspace/128x64/png/*.png
do
  convert "$file" -depth 1 "128x64/`printf %03d $COUNTER`.gray"
  COUNTER=$[$COUNTER +1]
done
