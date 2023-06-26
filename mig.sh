#!/bin/bash

# Display help text
Help() {
   echo "Syntax: mig.sh [(i)nit] [(m)igrate] [(h)elp]"
   echo "" 
   echo "(i)nit\t\tInit the docker container"
   echo "(m)igrate\t\tMigrate SeaOrm definitions"
   echo "(h)elp\t\tDisplay help"
}

while getopts "imh" option; do