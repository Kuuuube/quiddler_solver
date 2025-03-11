current_date=$(date +%Y-%m-%d_%H-%M-%S)
echo $current_date
mkdir puzzles_archive
wget --output-document=./puzzles_archive/puzzle_$current_date https://www.setgame.com/quiddler/puzzle
cp ./puzzles_archive/puzzle_$current_date puzzle
