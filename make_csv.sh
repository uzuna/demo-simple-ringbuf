#!/bin/bash

output_file="output.csv"

# Initialize the output file with header
echo '"filename","Run R0S","Run R1S","Run R2S","Run R2M (0,0)","Run R2M (0,1)","Run R2M (0,4)","Run R3S","Run R3M (0,0)","Run R3M (0,1)","Run R3M (0,4)","filesize"' > $output_file

# Loop through each input file
for input_file in bench*.txt; do
    variation=$(echo $input_file | awk -F'[_.]' '{print $2}')
    binary_name=target/$variation/simple-ringbuf
    file_size=$(ls -l $binary_name | awk '{print $5}')
    ms_data=$(awk '{for (i=1; i<=NF; i++) {if ($i == "ms") {print $(i-1)}}}' $input_file | paste -sd "," -)
    echo "$input_file,$ms_data,$file_size" >> $output_file
done
