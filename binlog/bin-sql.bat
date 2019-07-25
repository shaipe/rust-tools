:: 把binlog文件转换成sql文件
@echo off
echo text input

set input=
set /p input=:
echo %input% is input
cd %input%

for  %%a in (mysql-bin*) do (
    mysqlbinlog -vv --base64-output=decode-rows  %%a --result-file=%%a.sql
)

timeout /t  20
