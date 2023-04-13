# Media stubs

The folder stores media files used in the test library.

The `testsrc_*.mkv` files were generated using the following commands:

```shell
ffmpeg -f lavfi -i smptebars=duration=300:size=1280x720:rate=23.98 -filter_complex \
  "sine=d=10:f=100:b=8,volume=random(0):enable='random(1)',pan=stereo|FL=c0|FR=c0[beep1];
      sine=d=35:f=800:b=4,adelay=10s,pan=stereo|FL=c0|FR=c0[intro];
      sine=d=10:f=1400:b=2,volume=random(2):enable='random(3)',adelay=45s,pan=stereo|FL=c0|FR=c0[beep4];
      sine=d=30:f=800:b=4,adelay=270s,pan=stereo|FL=c0|FR=c0[beep5];
      [intro][beep1][beep4] [beep5]amix=inputs=4" \
  -i testsrc_metadata.txt -map_metadata 1 \
  -ac 2 -c:a aac -q:a 2 \
  -vf "drawtext=textfile=testsrc_credits.txt:x=0:y=h-70*(t-260):
       fontsize=36:fontcolor=yellow@0.9:box=1:boxcolor=black@1:
       enable='between(t,260,300)'" \
  -y testsrc_720p_h264_v1.mkv; \
ffmpeg -f lavfi -i smptebars=duration=300:size=1280x720:rate=23.98 -filter_complex \
  "sine=d=10:f=1400:b=2,volume=random(4):enable='random(5)',pan=stereo|FL=c0|FR=c0[beep1];
      sine=d=35:f=800:b=4,adelay=10s,pan=stereo|FL=c0|FR=c0[intro];
      sine=d=10:f=100:b=8,volume=random(6):enable='random(7)',adelay=45s,pan=stereo|FL=c0|FR=c0[beep4];
      sine=d=30:f=800:b=4,adelay=270s,pan=stereo|FL=c0|FR=c0[beep5];
      [intro][beep1][beep4][beep5]amix=inputs=4" \
  -i testsrc_metadata.txt -map_metadata 1 \
  -ac 2 -c:a aac -q:a 2 \
  -vf "drawtext=textfile=testsrc_credits.txt:x=0:y=h-70*(t-260):
       fontsize=36:fontcolor=yellow@0.9:box=1:boxcolor=black@1:
       enable='between(t,260,300)'" \
  -y testsrc_720p_h264_v2.mkv
```

Intros were automatically detected using Plex 1.32.
