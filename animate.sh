rm -r images
mkdir images

echo "Sorting..."
cargo run --release

echo "Resizing, adding audio..."
audio="./Jerma985 - sorting algorithm jerma-Z6H90OVeMI8.wav"
ffmpeg -y -r 130 -i images/%4d.ppm -i "$audio" -framerate 60 part.mp4

#echo "Converting for Discord..."
#part=part.mp4
#ffmpeg -y -i $part -vf scale=1280x720 \
#    -g 240 -threads 8 -speed 4 -row-mt 1 -tile-columns 2 -vsync cfr \
#    -c:v libvpx-vp9 -b:v 1000k -minrate 500k -maxrate 1450k -an \
#    -pass 1 -f null /dev/null && \
#
#ffmpeg -y -i $part -vf scale=1280x720 \
#    -g 240 -threads 8 -speed 2 -row-mt 1 -tile-columns 2 \
#    -c:v libvpx-vp9 -b:v 1000k -minrate 500k -maxrate 1450k -c:a libopus -b:a 96k \
#    -pass 2 out.webm
#
#echo "Done!"
