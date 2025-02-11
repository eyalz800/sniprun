#!/bin/sh

DIRECTORY=$1

get_latest_release() {
  curl --silent "https://api.github.com/repos/michaelb/sniprun/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' # Pluck JSON value
}

local_version=v$(cat Cargo.toml | grep version | cut -d "\"" -f 2)
remote_version=$(get_latest_release)

echo "
   _____       _       _____             
  / ____|     (_)     |  __ \            
 | (___  _ __  _ _ __ | |__) |   _ _ __  
  \___ \| '_ \| | '_ \|  _  / | | | '_ \ 
  ____) | | | | | |_) | | \ \ |_| | | | |
 |_____/|_| |_|_| .__/|_|  \_\__,_|_| |_|
                | |                      
                |_|                      
"

echo -n "Version : " $local_version
if [ $local_version == $remote_version ]; then
  echo " (up-to-date)"
else
  echo  " (update to " $remote_version "is available)"
fi

echo -e "\n"

echo "| Interpreter         | language    | comments"
echo "|---------------------|-------------|---------"
for file in $DIRECTORY/*.rs; do
  if [ $(basename $file) != "example.rs" ]; then
    IFS= read -r line <$file
    if [[ ${line:0:14} == "//Interpreter:" ]]; then
      echo "${line:14}"

    fi
  fi
done
