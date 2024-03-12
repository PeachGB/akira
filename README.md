## About
This proyect is meant to be an operative system in rust.
Im desinging as i'm developing so, treat this as a work of art more than something usefull.

However i think this can be educative, for me at least(i hope that for you too).
My idea for this is to be open source so i hope you can contribute to this, (im not an expert so 
if you have valuable knowledge you're more than welcome to share it)



### to just compile
cargo build -Z build-std --target x86_64-akira.json

### to make a booteable image
cargo bootimage -Z build-std --target x86_64-akira.json
it will output on /akira/target/x86_64-akira/debug/bootimage-akira.bin`


### Sources
[Writing an OS in Rust Philipp Oppermann's blog](https://os.phil-opp.com/)
[OSdevWiki](https://wiki.osdev.org/Expanded_Main_Page)
[Bitwise operators in Rust](https://rustlabs.github.io/docs/rust101/bitwise_operators/)



i'll try to explain everything trough comments, i hope that everything is clear
it may be overcommented but its for the people that doesn't understand much about binary operations or low level programming
