pushd ../blarggs-tests/cpu_instrs/source
# ./build.sh "06-ld r,r"
#  ./build.sh "01-special"
./build.sh "02-interrupts"
popd
cargo run -- --boot_rom ../gb_test_roms/DMG_ROM.bin --game_rom ../blarggs-tests/cpu_instrs/source/test.gb
