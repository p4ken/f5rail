
@echo off
REM #jww
REM #cd
REM #hf
REM #c  出力ファイル名 /_/出力ファイル名:
REM #e

:END
echo heエラー3 >> JWC_TEMP.txt
f5rail.exe /TRACK:X %1 /出力間隔:1 /TEMP_0:JWC_TEMP_0.TXT /TEMP_X:JWC_TEMP_X.TXT /TEMP:JWC_TEMP.TXT 2> log.txt REM 1>&2
