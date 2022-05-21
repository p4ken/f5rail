@REM ひとつ上のフォルダをご覧ください
@echo off
REM #jww
REM #cd
REM #hm | 他軌道指定 | スキップ |
REM #:1
REM #h1
REM #hc 他軌道を指定
REM #g1
REM #c  BVEのトラック名 /_/トラック名:
REM #:2
REM #e

goto %1

:1
MOVE JWC_TEMP.txt JWC_TEMP_X.txt
echo %2 >> JWC_TEMP_X.txt
goto END

:2
goto END

:END
echo h/sub/TRACK-SAVE.bat > JWC_TEMP.txt
@REM echo he他軌道のエラーチェック > JWC_TEMP.txt
