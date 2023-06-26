#!/bin/bash

# Display help text
Help() {
   echo "Syntax: mig.sh [-(-i)nit] [-(-m)igrate] [-(-h)elp]"
   echo "" 
   echo "(i)nit\t\tInit the docker container"
   echo "(m)igrate\t\tMigrate SeaOrm definitions"
   echo "(h)elp\t\tDisplay help"
}

Init() {
   echo "Creating docker instance"
    . ./scripts/init_db.sh
   echo "Migrated"
}

Migrate() {
   SKIP_DOCKER=true . ./scripts/init_db.sh
   echo "Migrated"
}

while getopts "imh-" OPT; do
  # support long options: https://stackoverflow.com/a/28466267/519360
  if [ "$OPT" = "-" ]; then   # long option: reformulate OPT and OPTARG
    OPTARG="${OPTARG#$OPT}"   # extract long option argument (may be empty)
  fi
  case "$OPT" in
    i | init )     Init ;;
    m | migrate )  Migrate ;;
    h | help )     Help ;;
    : )            exit 0;;
  esac
done
shift $((OPTIND-1)) # remove parsed options and args from $@ list