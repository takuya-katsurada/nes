#[cfg(test)]
mod tests {
    use nes::Nes;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore]
    fn load_rom() -> std::io::Result<()> {
        let mut file = File::open("roms/nes-test-roms/other/demo.nes")?;
        let mut contents = vec![];
        file.read_to_end(&mut contents);

        let nes = Nes::from(&mut contents).unwrap();

        let snapshot = nes.snapshot();
        assert_eq!(snapshot.prg_rom_bytes, 1);
        assert_eq!(snapshot.chr_rom_bytes, 1);
        Ok(())
    }
}