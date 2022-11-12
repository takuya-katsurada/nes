
mod tests {
    use rom::Rom;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore]
    fn load_rom() -> std::io::Result<()> {
        let mut file = File::open("roms/nes-test-roms/other/demo.nes")?;
        let mut contents = vec![];
        file.read_to_end(&mut contents);

        let isValid = Rom::isValid(&mut contents);
        let rom = Rom::new(contents);
        assert!(isValid);
        assert_eq!(rom.prg_rom_bytes, 1);
        assert_eq!(rom.chr_rom_bytes, 1);
        Ok(())
    }
}