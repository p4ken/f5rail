@REM サイン半波長逓減曲線を作図する
@echo off
REM #jww
REM #cd
REM #c  開始R [m] ※無指定は直線 : /_/R1:
REM #c  終了R [m] ※無指定は直線 : /_/R2:
REM #c  TCL [m] : /_/TCL:
REM #c  間隔 [m] ※無指定は0.1 : /_/DX:
REM #e
f5rail.exe /FUNC:sin %1 %2 %3 %4 /FILE:./JWC_TEMP.TXT 2> log.txt REM 1>&2

@REM echo. > JWC_TEMP.TXT
@REM echo pl >> JWC_TEMP.TXT
@REM echo 0 0 >> JWC_TEMP.TXT
@REM echo heエラー1 >> JWC_TEMP.TXT
@REM echo 100 -100 >> JWC_TEMP.TXT
@REM echo 200 -400 >> JWC_TEMP.TXT
@REM echo 300 -900 >> JWC_TEMP.TXT
@REM echo # >> JWC_TEMP.TXT
@REM @REM echo h#注意1 >> JWC_TEMP.TXT
