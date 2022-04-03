@REM 緩和曲線を作図します
@echo off
REM #jww
REM #cd
REM #hm | サイン(L) | 直線逓減(R) |
REM #hc v(VERSION)
REM #:1
REM #:2
REM #c  【開始半径】R0 (m) ※マイナスは左曲線, 無指定は直線/_/R0:
REM #c  【終了半径】R1 (m) ※マイナスは左曲線, 無指定は直線/_/R1:
REM #c  【緩和曲線長】TCL (m)/_/TCL:
REM #e

f5rail.exe /TRANSITION:%1 %2 %3 %4 /L0:0 /FILE:./JWC_TEMP.TXT 2> log.txt REM 1>&2
