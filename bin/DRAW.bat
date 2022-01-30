@REM 緩和曲線を作図する
@echo off
REM #jww
REM #cd
REM #c  開始R [m] ※無指定は直線 : /_/R1:
REM #c  終了R [m] ※無指定は直線 : /_/R2:
REM #c  TCL [m] : /_/TCL:
REM #c  間隔 [m] ※無指定は0.1 : /_/DX:
REM #e
draw.exe /_/FUNC:SIN %1 %2 %3 %4 /IO:JWC_TEMP.TXT
