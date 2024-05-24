UA := "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"
REGION := "35°38′47″N 080°03′00″W-35°29′20″N 079°51′00″W"

topo.png: target/release/topo themes.bin

target/release/topo: Cargo.toml $(shell find src -type f)
	cargo build --release

themes.bin:
	curl -A $(UA) -o $@ https://storage.googleapis.com/fs.kellegous.com/themes-small.bin

fseprd485939.pdf:
	curl -A $(UA) -o $@ https://www.fs.usda.gov/Internet/FSE_DOCUMENTS/fseprd485939.pdf

fseprd485939.svg: fseprd485939.pdf
	pdftocairo -svg $^ $@

fseprd485939.json: fseprd485939.svg target/release/topo
	target/release/topo extract $(REGION) $< $@

topo.png: fseprd485939.json target/release/topo themes.bin 
	target/release/topo render $< $@

clean:
	rm -f topo.png
	cargo clean

nuke: clean
	rm -f themes.bin