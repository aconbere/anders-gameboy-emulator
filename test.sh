pushd ../blarggs-tests/cpu_instrs/source
# ./build.sh "06-ld r,r"
#  ./build.sh "01-special.gb"
./build.sh "02-interrupts.gb"
popd
cargo run -- --boot_rom ../gb_test_roms/DMG_ROM.bin --game_rom ../blarggs-tests/cpu_instrs/source/test.gb
