#! /bin/bash

# == ARGS ==
# -u uppercase
# -c characters
# -l letters
# -n numbers
# --length number
# default: genpass -snlu --length 16

declare -r letters="abcdefghijklmnopqrstuvwxyz"
declare -r characters="!@#%^&*()-_=+[]{}\|/?.,<>;:"
declare -r numbers="0123456789"
declare -a gen=($letters $characters $numbers)

declare -i length=16
declare is_upper=true
declare result

if [[ "${1:0:1}" == '-' ]];
then
  gen=()
  is_upper=false

  for ((i=1; i<${#1}; i++));
  do
    case ${1:$i:1} in
      'n')
        gen+=($numbers)
        ;;

      'c')
        gen+=($characters)
        ;;

      'l')
        gen+=($letters)
        ;;
      'u')
        is_upper=true
        ;;
      *)
        echo "Bad input"
        echo "All arguments: -nclu"
        exit
        ;;
      esac
  done
fi

if [[ ${2:0:2} == "--" ]];
then
  if  [[ ${2:2} == "length" &&  $3 =~ ^[0-9]+$ ]];
  then
    length=$3
  else
    echo "Bad input"
    echo "All rguments: --length number"
    exit
  fi
fi


for ((i=0; i<$length; i++));
do
  rand_index=$(( $RANDOM % ${#gen[@]} ))
  gen_length=${#gen[$rand_index]}
  character=${gen[rand_index]:(( $RANDOM % $gen_length )):1}

  if [ $is_upper == true ] && [ $(( $RANDOM % 2 )) = 1 ];
  then
    character=${character^^}
  fi

  result+=$character
done

echo $result
