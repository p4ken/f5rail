@REM サイン半波長逓減曲線を作図する
@echo off
REM #jww
REM #cd
REM #c  開始半径 R1 [m] ※無指定は直線 : /_/R1:
REM #c  終了半径 R2 [m] ※無指定は直線 : /_/R2:
REM #c  TCL [m] : /_/TCL:
REM #c  間隔 DX [m] ※無指定は0.1 : /_/DX:
REM #e
f5rail.exe /FUNC:sin %1 %2 %3 %4 /FILE:./JWC_TEMP.TXT 2> log.txt 1>&2
