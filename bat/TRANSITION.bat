@REM BVEの緩和曲線を作図する
@echo off
REM #jww
REM #cd
REM #hm | サイン半波長 | 直線逓減 |
REM #hc 【緩和曲線関数】 作図画面内を左クリックでサイン半波長逓減、右クリックで直線逓減（クロソイド曲線）
REM #:1
REM #c  【サイン半波長逓減】 開始半径 R0 [m] ※無指定は直線 : /_/R0:
REM #c  【サイン半波長逓減】 終了半径 R1 [m] ※無指定は直線 : /_/R1:
REM #c  【サイン半波長逓減】 緩和曲線長 TCL [m] : /_/TCL:
REM #e
REM #:2
REM #c  【直線逓減】 開始半径 R0 [m] ※無指定は直線 : /_/R0:
REM #c  【直線逓減】 終了半径 R1 [m] ※無指定は直線 : /_/R1:
REM #c  【直線逓減】 緩和曲線長 TCL [m] : /_/TCL:
REM #e

f5rail.exe /TRANSITION:%1 %2 %3 %4 /L0:0 /FILE:./JWC_TEMP.TXT 2> log.txt REM 1>&2
