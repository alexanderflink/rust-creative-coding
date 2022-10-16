cd $1
cargo run --release
ffmpeg -r 60 -i ./render/frame%d.png "./render/$1.mp4"
rm ./render/*.png