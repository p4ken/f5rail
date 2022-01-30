all: bin/DRAW.bat bin/DRAW.exe

bin/DRAW.bat: utf8/DRAW.bat
	iconv -f utf8 -t sjis utf8/DRAW.bat > $@

bin/DRAW.exe: draw/target/release/draw.exe
	cp -p draw/target/release/draw.exe $@

.PHONY: run
run:
	bin/DRAW.exe
