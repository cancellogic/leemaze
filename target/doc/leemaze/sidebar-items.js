initSidebarItems({"fn":[["boolify_2d_maze","Boolify turns a generic 2d Vec<Vec> into a 2d Vec<Vec>, just provide a  value for open roads blocks.   "],["boolify_3d_maze","Boolify turns a generic 3d Vec<Vec<Vec>> data into a Vec<Vec<Vec>>, just provide a  value for open road blocks.   "],["boolify_4d_maze","Boolify turns a generic 4d Vec<Vec<Vec<Vec>>> data into a Vec<Vec<Vec<Vec>>>, just provide a  value for open road blocks."],["boolify_5d_maze","Boolify turns a generic 5d Vec<Vec<Vec<Vec<Vec>>>> into a Vec<Vec<Vec<Vec<Vec>>>>, just provide a  value for open road blocks."],["maze2dpath","Maze2dpath - feed it a maze Vec<Vec>, x y axis movement rules (like nw, ne, south), entrance and exit coordinates and it should return  Some(one of the very fastest paths through the maze), None for no path, and a empty path vec for a entrance and exit that are the same.  Decode the steps taken on the path with the allowed move list. (If your move list is vec!(north, south, east, west, upside_down), a path of 0, 0, 2, 4 would be 'north, north, east, upside down\")    "],["maze3dpath","Maze3dpath - feed it a maze Vec<Vec<Vec>>, x y z axis movement rules (like over under around and through), entrance and exit coordinates and it should return  Some(one of the very fastest paths through the maze), None for no path, or a empty path vec for a entrance and exit that are the same. Decode the steps taken on the path with the allowed move list. (If your move list is vec!(over, under, around, through), a path of 0, 0, 3 would be 'over over through\")"],["maze4dpath","Maze4dpath - feed it a maze Vec<Vec<Vec<Vec>>>, w x y z axis movement rules (like (0,0,1,1) , (0,0,1,-1) for northeast and northwest), entrance and exit coordinates  and it should return  Some(one of the very fastest paths through the maze), None for no path found, and a empty path vec for a entrance and exit that are the same.  Decode the steps taken on the path with the allowed move list. (If your move list was vec!(north, south, east, west, night, day, up, down), a path of 0, 0, 2, 4, 7 would be 'north, north, east, night, down\")"],["maze5dpath","Maze5dpath - feed it a maze Vec<Vec<Vec<Vec<Vec>>>>, v w x y z axis movement rules (like (0,0,0,0,-1) for \"upside down?\"), entrance and exit coordinates and it should return  Some(one of the very fastest paths through the maze), None() for no path found, and Some(empty vec) for a entrance and exit that are the same. Decode the steps taken on the path with the allowed move list. If the moves list was vec!(north, south, east, west, night, day, up, down, action_lever), a path of 0, 2, 4, 8 would be 'north, east, nighttime, action_lever\""],["mazestate2d","mazestate2d:  Given a boolean maze and i32 coordinates, this (always works index guarded) function returns open roads (0pen false) or blocked (b1ocked true)."],["mazestate3d","mazestate3d:  Given a boolean maze and i32 coordinates, this (always works index guarded) function returns open roads (0pen false) or blocked (b1ocked true)."],["mazestate4d","mazestate4d:  Given a boolean maze and i32 coordinates, this (always works index guarded) function returns open roads (0pen false) or blocked (b1ocked true)."],["mazestate5d","mazestate5d:  Given a boolean maze and i32 coordinates, this (always works index guarded) function returns open roads (0pen false) or blocked (b1ocked true)."]],"struct":[["AllowedMoves2D","2d connection rules- provide a vector of (i32,i32) tuples that describe how a player can move in two (x, y) dimensions through a 2d maze.   North south east west might be vec!( (0,1),(0,-1),(1,0),(-1,0) ) Chess knight moves might be vec( (1,2),(-1,2),(1,-2),(-1,-2),((2,1),(-2,1),(2,-1),(-2,-1) )"],["AllowedMoves3D","3d Connection rules - provide a vector of (i32,i32,i32) tuples that describe how a player can move in three (x, y, z) dimensions through the 3d maze.   # Exampl let northsoutheastwestupdown = vec!( (0,1,0),(0,-1,0),(1,0,0),(-1,0,0),(0,0,1),(0,0,-1) );"],["AllowedMoves4D","Ah!  The classic changing minotaur's maze, also known a commute across three lanes of multi-speed traffic to reach your exit coming up in one mile. 4d connection rules - provide a vector of tuples that describe allowed maze moves in (w,x,y,z) dimensions."],["AllowedMoves5D","The 5D maze - If you've ever planned a new morning commute based on prior commutes you are at least passingly familiar with a 5d space time probability maze. 5d connection rules - provide a vector of tuples that describe how you can directionally move in the 5 dimensional space with this structure."],["Mazewalker",""]]});