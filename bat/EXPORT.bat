@REM 【未実装】BVEの他線座標を計算します
@ECHO OFF
REM #jww
REM #hf
REM #cd
REM #h4
REM #0自軌道上の距離基準点を右クリック
REM #c基準点のＢＶＥ距離程 [メートル]/_/z0:
REM #1他軌道上の出力「起点」を右クリック
REM #2他軌道上の出力「終点」を右クリック
REM #c他軌道のＢＶＥトラック名 /_/track:
REM #g1
REM #c出力ファイル名 /_/file:
REM #e

ECHO %* > log.txt
for /f "delims=" %%i in ('FINDSTR /B "file=" JWC_TEMP.txt') do SET PROJECT_FILE=%%i
f5rail.exe %* "%PROJECT_FILE%"
