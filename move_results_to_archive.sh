current_date=$(date +%Y-%m-%d_%H-%M-%S)
mkdir quiddler_games_archive
mv quiddler_games ./quiddler_games_archive/quiddler_games_$current_date
mv quiddler_games_scored ./quiddler_games_archive/quiddler_games_scored_$current_date
