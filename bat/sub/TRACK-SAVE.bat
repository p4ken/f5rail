@REM ひとつ上のフォルダをご覧ください
@echo off
REM #jww
REM #cd
REM #hf
REM #c  出力ファイル名 ※引用符(")NG/_/出力ファイル名:
REM #e

f5rail.exe /TRACK:X "%*" /出力間隔:5 /TEMP_0:JWC_TEMP_0.TXT /TEMP_X:JWC_TEMP_X.TXT /TEMP:JWC_TEMP.TXT 2> log.txt REM 1>&2
