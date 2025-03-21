$env:GST_DEBUG = "souphttpsrc:1,hlsdemux:1,tsdemux:1,h264parse:1,avdec_h264:1,videoconvert:1,autovideosink:1"
gst-launch-1.0 souphttpsrc location=http://172.25.200.99/live/video33.m3u8?profile=low ! `
    queue ! `
    hlsdemux ! `
    queue ! `
    tsdemux ! `
    queue ! `
    h264parse ! `
    queue ! `
    d3d11h264dec ! `
    queue ! `
    videoconvert ! `
    d3d11videosink