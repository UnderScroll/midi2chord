# midi2chord
Converts midi keyboard inputs into possible chord names

How to:
To launch you must have rust and cargo installed.
Start with `cargo run`

Or after `cargo build --release` and execute `target/release/midi2chord`

Select your midi keyboard and that's it, it will give you a list of possible chords


## Linux
Depends on ALSA

### Ubunto 23.10 x86_64
```bash
sudo apt install alsa
sudo apt install alsa-utils
sudo apt install pkg-config

git clone https://github.com/UnderScroll/midi2chord.git
cd /midi2chord

cargo build --release
cd target/release/
./midi2chord
```
## Windows
Depends on WinMM

## Additional notes
I'm still a beginner with rust, I know it's awfully written
