@REM BVEマップに出力します
@echo off
REM #jww
REM #cd
REM #hm | 自軌道指定 | スキップ |
REM #:1
REM #h1
REM #hc 自軌道を指定
REM #1  出力始点を指定
REM #g1
REM #c  出力始点のBVE距離程 (m)/_/始点距離程:
REM #:2
REM #e

goto %1

:1
MOVE JWC_TEMP.txt JWC_TEMP_0.txt
echo %2 >> JWC_TEMP_0.txt
goto END

:2
goto END

:END
echo h/sub/TRACK-X.bat > JWC_TEMP.txt
@REM echo he自軌道のエラーチェック > JWC_TEMP.txt
