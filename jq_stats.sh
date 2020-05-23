#!/bin/bash

if [ -z "$1" ]
then
    echo "Usage: jq_stats.sh filename"
    exit 1
fi
if [ ! -f "$1" ]
then
    echo "$1 is not a valid file"
    exit 1
fi

FILE=$1

echo "Messages in channel:"
jq "length" $FILE

echo "Number of threads:"
jq '[ .[].thread | arrays ] | length' $FILE

echo "Number of messages in all threads:"
jq '[ .[].thread | arrays | .[] ] | length' $FILE

echo "Total messages:"
jq '[ .[], (.[].thread | arrays | .[]) ] | length' $FILE

echo "The most messages in a single thread is:"
jq '[ .[].thread | arrays | length ] | max' $FILE

echo "Each person sent this number of messages (including in threads):"
jq '[ .[], (.[].thread | arrays | .[]) ] | [ group_by(.name) | .[] | { (.[0].name): length } ] | add' $FILE

echo "Number of threads started by each person:"
jq '[ .[] | select(.thread) | .thread[0].name ] | [ group_by(.) | .[] | { (.[0]): length } ] | add' $FILE

echo "Dates of the first and last message sent (including in threads):"
jq '[ .[].thread[]?, .[] | .ts | tonumber | todate ] | {first_message: min, last_message: max}' $FILE

echo "Each person typed this number of words (including in threads):"
jq '[ .[], (.[].thread | arrays | .[]) | { name, words: [ .text | scan("['\''’a-zA-Z_-]+") ] | length } ] | [ group_by(.name) | .[] | { (.[0].name): [ .[] | .words ] | add } ] | add' $FILE

echo "Each person typed this number of characters (including in threads):"
jq '[ .[], (.[].thread | arrays | .[]) | { name, chars: .text | length } ] | [ group_by(.name) | .[] | { (.[0].name): [ .[] | .chars ] | add } ] | add' $FILE
