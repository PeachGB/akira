## About
This proyect is meant to be an operative system in rust.
Im desinging as i'm developing so, treat this as a work of art more than something usefull.

However i think this can be educative, for me at least(i hope that for you too).
My idea for this is to be open source so i hope you can contribute to this, (im not an expert so 
if you have valuable knowledge you're more than welcome to share it)

### to compile
cargo build -Z build-std --target x86_64-akira.json
### to make a booteable image
cargo bootimage -Z build-std --target x86_64-akira.json
it will output on /akira/target/x86_64-akira/debug/bootimage-akira.bin`


### Sources
im using this as a guideline for the moment
Writing an OS in Rust Philipp Oppermann's blog: https://os.phil-opp.com/


i'll try to explain everything trough comments, i hope that everything is clear
