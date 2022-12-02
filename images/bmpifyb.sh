COUNTER=1

FILLS=(
''
'-fill #65DA5F -opaque #FFFFFF'
'-fill #DD1D37 -opaque #FFFFFF'
'-fill #F81082 -opaque #FFFFFF'
'-fill #71B3FF -opaque #FFFFFF'
)
FORES=(
''
'-fill #FD161D -opaque #000000'
'-fill #FA269D -opaque #000000'
'-fill #2FF812 -opaque #000000'
'-fill #A91020 -opaque #FFFFFF' // A91020
)

for file in ~/tmp/eink/128x160/binary/*.png
do
  convert $file\
    -type truecolor\
    -define colorspace:auto-grayscale=false\
    ${FILLS[`shuf -i0-4 -n1`]}\
    ~/tmp/eink/128x160/bmp/b$RANDOM-$COUNTER.bmp
  COUNTER=$[$COUNTER +1]
done

COUNTER=1
for file in ~/tmp/eink/128x160/bmp/b*.bmp
do
  convert $file\
    -type truecolor\
    -define colorspace:auto-grayscale=false\
    ${FORES[`shuf -i0-3 -n1`]}\
    ~/tmp/eink/128x160/bmp/a`printf %03d $COUNTER`.bmp
  COUNTER=$[$COUNTER +1]
done
