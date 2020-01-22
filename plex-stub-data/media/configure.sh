#!/bin/sh

set -e

cd $(dirname $0)

mkdir -p "TV-Shows/Game of Thrones"
for s in $(seq 1 2)
do
  for e in $(seq 1 9)
  do
    cp white_noise_720p.mkv "TV-Shows/Game of Thrones/Game.of.Thrones.S0${s}E0${e}.mkv"
  done
done

mkdir -p "TV-Shows/The 100"
for s in $(seq 1 2)
do
  for e in $(seq 1 9)
  do
    cp white_noise_720p.mkv "TV-Shows/The 100/The.100.S0${s}E0${e}.mkv"
  done
done

for f in "Cats" "Cats/Cats in bed" "Cats/Cats not in bed"
do
  for e in $(seq 1 3)
  do
    mkdir -p "Photos/$f"
    cp white_noise_720p.jpg "Photos/$f/Picture$e.jpg"
  done
done

mkdir -p "Music/System of a Down - Toxicity (1999)"
cp white_noise.aac "Music/System of a Down - Toxicity (1999)/01 - Toxicity.aac"
cp white_noise.aac "Music/System of a Down - Toxicity (1999)/02 - Marmalade.aac"
cp white_noise.aac "Music/System of a Down - Toxicity (1999)/03 - Metro.aac"

mkdir -p "Music/System of a Down - Aerials (2002)"
cp white_noise.aac "Music/System of a Down - Aerials (2002)/01 - Aerials.aac"
cp white_noise.aac "Music/System of a Down - Aerials (2002)/02 - Streamline (album version).aac"
cp white_noise.aac "Music/System of a Down - Aerials (2002)/03 - Sugar (live).aac"

mkdir -p "Music/Skrillex - Try It Out (2003)"
cp white_noise.aac "Music/Skrillex - Try It Out (2003)/01 - TRY IT OUT (NEON MIX).aac"
cp white_noise.aac "Music/Skrillex - Try It Out (2003)/02 - Try It Out (Try Harder Mix).aac"
cp white_noise.aac "Music/Skrillex - Try It Out (2003)/03 - Try It Out (Put Em Up Mix).aac"
