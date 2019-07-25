#!/system/bin/sh

#read -p "Input date(ex20190701):" -s date
#echo ${date}

for file in ls /data/misc/scripttest/*.sh
do
$file
done