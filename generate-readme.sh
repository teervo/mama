#!/bin/bash
if [ -f ~/todo.txt ]
then
    BACKUP=$(mktemp -t todo.XXXXXXXX)
    mv ~/todo.txt $BACKUP
fi

echo $(date -u +"%Y-%m-%d" -d "last wednesday") Implement new color scheme for @kitchen appliances +kitchenmakeover > ~/todo.txt
echo \(C\) $(date -u +"%Y-%m-%d" -d "-2 weeks") Renew subscription to Hobby Horse Monthly >> ~/todo.txt
echo \(A\) $(date -u +"%Y-%m-%d" -d "-3 days") Replenish the pisco decanters in the @office >> ~/todo.txt

echo '# mama'
echo "**A command line application for managing [todo.txt](http://todotxt.org/)**"
echo

echo '## Installation'
echo **With cargo**
echo '```console'
echo '$ cargo install mama'
echo '```'

echo **From git source**
echo '```console'
echo '$ cargo install --git https://github.com/teervo/mama.git'
echo '```'
echo

echo '## Usage'
echo '```console'
cargo run help 2> /dev/null
echo '```'
echo

echo '## Examples'
echo **List all tasks**
echo '```console'
echo '$ mama'
cargo run 2> /dev/null
echo '```'
echo

echo **List tasks matching words**
echo '```console'
echo '$ mama ls kitchen office'
cargo run ls kitchen office 2> /dev/null
echo '```'
echo

echo **Long listing includes creation date, completion date and priority**
echo '```console'
echo '$ mama ls -l'
cargo run ls -l 2> /dev/null
echo '```'
echo

echo **Add a new task**
echo '```console'
echo '$ mama add Alphabetize spice rack @kitchen +kitchenmakeover'
cargo run add Alphabetize spice rack @kitchen +kitchenmakeover 2> /dev/null
echo '```'
echo

echo **Add a new task with priority level A**
echo '```console'
echo '$ mama add -p A Procure ingredients for the Odelmaß'
cargo run add -p A Procure ingredients for the Odelmaß 2> /dev/null
echo '```'
echo

echo **Mark a task as completed**
echo '```console'
echo '$ mama complete 3'
cargo run complete 3 2> /dev/null
echo '```'
echo

echo **Remove one or more tasks**
echo '```console'
echo '$ mama add rm 2 3'
cargo run rm 2 3 2> /dev/null
echo '```'
echo

if [ -n BACKUP ]
then
    mv $BACKUP ~/todo.txt
fi
