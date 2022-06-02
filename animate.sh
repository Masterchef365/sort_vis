#ffmpeg -i images/%4d.ppm -framerate 60 out.mp4
audio="./Jerma985 - sorting algorithm jerma-Z6H90OVeMI8.wav"
ffmpeg -r 120 -i images/%4d.ppm -i "$audio" -framerate 60 part.mp4
#convert images/0000.ppm 0000.jpg
#ffmpeg -i part.mp4 -i 0000.jpg -map 0 -map 1 -c copy -c:v:1 png -disposition:v:1 attached_pic out.mp4

part=part.mp4
ffmpeg -y -i $part -vf scale=1280x720 \
    -g 240 -threads 8 -speed 4 -row-mt 1 -tile-columns 2 -vsync cfr \
    -c:v libvpx-vp9 -b:v 1000k -minrate 500k -maxrate 1450k -an \
    -pass 1 -f null /dev/null && \
    #-ss 00:00:10 -to 00:00:20 \

ffmpeg -i $part -vf scale=1280x720 \
    -g 240 -threads 8 -speed 2 -row-mt 1 -tile-columns 2 \
    -c:v libvpx-vp9 -b:v 1000k -minrate 500k -maxrate 1450k -c:a libopus -b:a 96k \
    -pass 2 out.webm -y
    #-ss 00:00:10 -to 00:00:20 \
