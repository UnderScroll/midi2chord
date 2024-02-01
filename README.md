# midi2chord
Converts midi keyboard inputs into possible chord names

Launch the program
Select your midi keyboard (it will be automatically selected if there is just one)
And that's it!

Output exemple:
```
C4 E4 G4 A#4/B♭4 
Chords:
        C7 [5]
        Asus(2/#4)(13)/C [18]
        Gmin(sus4)(13)/C [22]
        Emin(sus#4)(♭13)/C [24]
```
First there is the chord name, then the chord weight in between brakets
The wieght indicate how "complexe" the chord is
All chord names should be valid (even though some can be a bit weird)
6th chords aren't implemented

## Linux - Ubunto 23.10 x86_64
Depends on ALSA (for the midir crate)
### Installation
```bash
sudo apt install alsa
sudo apt install alsa-utils
sudo apt install pkg-config

git clone https://github.com/UnderScroll/midi2chord.git
cd /midi2chord
```
### Execution
```
cargo build --release
cd target/release/
./midi2chord
```
or
```
cargo run
```
## Windows
Depends on WinMM
### Installation
```batch
git clone https://github.com/UnderScroll/midi2chord.git
cd /midi2chord
```
### Execution
```batch
cargo build --release
cd target/release/
.\midi2chord.exe
```
or
```
cargo run
```

## Additional notes
I'm still a beginner with rust, I know it's awfully written
