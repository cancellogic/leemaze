/// Lee Algorithm in rust provides one of the shortest paths through a maze.
///
/// Maze may be 2d (Vec<Vec<bool>>) or 3d, 4d or 5d (Yea! 5D - plan your nth morning commute through shifting traffic and 2% driver crazies to arrive at a place and time... probably.)
/// Maze need not be composed of same length vectors.  Movement rules (allowed directions) through maze need to be defined in a vector.  
/// The maze may be exceptionally large, but please remember that the Lee algorithm can be memory hungry.
/// Only one fastest path is returned, even if there are many.  Walls are true, paths are zeros.
/// decode the returned maze direction path with your list of allowed directional maze moves by index value.
/// Mazes & minataurs remind me of that song Woolly Bully, so thanks Sam the Sham for that ear-worm.
///
fn main() {
    let _thisisanexamplenonrectangularthreedmaze = vec![
        vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 1],
            vec![0, 0, 1, 1, 0],
        ],
        vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 0, 1, 0],
        ],
    ];
}

/// 2d connection rules- provide a vector of (i32,i32) tuples that describe how a player can move in two (x, y) dimensions through a 2d maze.  
/// North south east west might be vec!( (0,1),(0,-1),(1,0),(-1,0) )
/// Chess knight moves might be vec( (1,2),(-1,2),(1,-2),(-1,-2),((2,1),(-2,1),(2,-1),(-2,-1) )
#[derive(Debug, Clone)]
pub struct AllowedMoves2D {
    rules: Vec<(i32, i32)>,
    //let northsoutheastwest = AllowedMoves2D {moves:vec![(0,1),(0,-1),(1,0),(-1,0)]};
}

/// 3d Connection rules - provide a vector of (i32,i32,i32) tuples that describe how a player can move in three (x, y, z) dimensions through the 3d maze.  
/// # Exampl
/// let northsoutheastwestupdown = vec!( (0,1,0),(0,-1,0),(1,0,0),(-1,0,0),(0,0,1),(0,0,-1) );
#[derive(Debug, Clone)]
pub struct AllowedMoves3D {
    rules: Vec<(i32, i32, i32)>,
    //let northsoutheastwestupdown = AllowedMoves2D {moves:vec![(0,1,0),(0,-1,0),(1,0,0),(-1,0,0),(0,0,1),(0,0,-1)]};
}

///Ah!  The classic changing minotaur's maze, also known a commute across three lanes of multi-speed traffic to reach your exit coming up in one mile.
///4d connection rules - provide a vector of tuples that describe allowed maze moves in (w,x,y,z) dimensions.
#[derive(Debug, Clone)]
pub struct AllowedMoves4D {
    rules: Vec<(i32, i32, i32, i32)>,
}

///The 5D maze - If you've ever planned a new morning commute based on prior commutes you are at least passingly familiar with a 5d space time probability maze.
///5d connection rules - provide a vector of tuples that describe how you can directionally move in the 5 dimensional space with this structure.
#[derive(Debug, Clone)]
pub struct AllowedMoves5D {
    rules: Vec<(i32, i32, i32, i32, i32)>,
}

/// Boolify turns a generic 2d Vec<Vec<T>> into a 2d Vec<Vec<bool>>, just provide a <T> value for open roads blocks.   
pub fn boolify_2d_maze<T: Ord>(openroadvalue: &T, maze: &Vec<Vec<T>>) -> Vec<Vec<bool>> {
    //Goal is to turn a Vec<Vec<T>> into a Vec<Vec<bool>> where open paths are considered to be less than some value
    //assert!("Booly Booly" == "Woolly Bully")  Thanks Sham and & The Pharaohs for that curious ear worm
    let mut hatty = vec![];
    for bully in maze {
        // watch X now watch X
        let mut matty = vec![];
        for wooly in bully {
            if wooly == openroadvalue {
                matty.push(false);
            } else {
                matty.push(true)
            }
        }
        //Matty told hatty, about a thing she saw, two big horns and a woolly jaw..
        hatty.push(matty);
    }
    return hatty;
}

/// Boolify turns a generic 3d Vec<Vec<Vec<T>>> data into a Vec<Vec<Vec<bool>>>, just provide a <T> value for open road blocks.   
pub fn boolify_3d_maze<T: Ord>(openroadvalue: &T, maze: &Vec<Vec<Vec<T>>>) -> Vec<Vec<Vec<bool>>> {
    //Goal is to turn a 3d Vec<Vec<Vec<T>>> into a Vec<Vec<Vec<bool>>> where open paths are equal to the openroadvalue
    let mut woolybully = vec![];
    for level in maze {
        woolybully.push(boolify_2d_maze(openroadvalue, level));
    }
    return woolybully;
}

/// Boolify turns a generic 4d Vec<Vec<Vec<Vec<T>>>> data into a Vec<Vec<Vec<Vec<bool>>>>, just provide a <T> value for open road blocks.
pub fn boolify_4d_maze<T: Ord>(
    openroadvalue: &T,
    maze: &Vec<Vec<Vec<Vec<T>>>>,
) -> Vec<Vec<Vec<Vec<bool>>>> {
    //Goal is to turn a 4d Vec<Vec<Vec<Vec<T>>>> into a Vec<Vec<Vec<Vec<bool>>>> where open paths are equal to the openroadvalue
    //Does your labyrinth shift like a car in predictable traffic?  untested, I think this works...
    let mut not_be_l_seven = vec![];

    for layer in maze {
        not_be_l_seven.push(boolify_3d_maze(openroadvalue, layer));
    }
    return not_be_l_seven;
}

/// Boolify turns a generic 5d Vec<Vec<Vec<Vec<Vec<T>>>>> into a Vec<Vec<Vec<Vec<Vec<bool>>>>>, just provide a <T> value for open road blocks.
pub fn boolify_5d_maze<T: Ord>(
    openroadvalue: &T,
    maze: &Vec<Vec<Vec<Vec<Vec<T>>>>>,
) -> Vec<Vec<Vec<Vec<Vec<bool>>>>> {
    //Goal is to turn a 4d Vec<Vec<Vec<Vec<T>>>> into a Vec<Vec<Vec<Vec<bool>>>> where open paths are equal to the openroadvalue
    //Does your labyrinth shift like your real life commute - with traffic that contains 2% crazy drivers that make improbable decisions?  You can model as navigation through a 5D (3d+time+probability) maze. I think this works... um goodluck,
    let mut not_be_l_seven = vec![];

    for eachprobability in maze {
        not_be_l_seven.push(boolify_4d_maze(openroadvalue, eachprobability));
    }
    return not_be_l_seven;
}

///mazestate2d:  Given a boolean maze and i32 coordinates, this (always works index guarded) function returns open roads (0pen false) or blocked (b1ocked true).
pub fn mazestate2d(maze: &Vec<Vec<bool>>, pos: &(i32, i32)) -> bool {
    //False = Open (open can be spelled 0pen, 0 for false),  Wall (Walls can be spelled wa11, 1 for true.)
    let (x, y) = *pos;
    if x < 0i32 {
        return true;
    }
    if y < 0i32 {
        return true;
    }
    if y >= maze.len() as i32 {
        return true;
    }
    if x >= (maze[y as usize]).len() as i32 {
        return true;
    }
    let x = x as u32; //x cannot be negative or out of range
    let y = y as u32; //y cannot be negative or out of range
    let x = x as usize;
    let y = y as usize;
    return (maze[y])[x];
}

///mazestate3d:  Given a boolean maze and i32 coordinates, this (always works index guarded) function returns open roads (0pen false) or blocked (b1ocked true).
pub fn mazestate3d(maze: &Vec<Vec<Vec<bool>>>, pos: &(i32, i32, i32)) -> bool {
    let (x, y, z) = *pos;

    if x < 0i32 {
        return true;
    }
    if y < 0i32 {
        return true;
    }

    if z >= (maze.len() as i32) {
        return true;
    }
    if z < 0i32 {
        return true;
    }
    let z = z as u32;
    let z = z as usize; //Z is safe to value check

    if y >= (maze[z].len() as i32) {
        return true;
    }
    let y = y as u32;
    let y = y as usize; //Y is safe to value check

    if x >= ((maze[z])[y]).len() as i32 {
        return true;
    }
    let x = x as u32;
    let x = x as usize; //X is safe to value check

    return ((maze[z])[y])[x];
}

///mazestate4d:  Given a boolean maze and i32 coordinates, this (always works index guarded) function returns open roads (0pen false) or blocked (b1ocked true).
pub fn mazestate4d(maze: &Vec<Vec<Vec<Vec<bool>>>>, pos: &(i32, i32, i32, i32)) -> bool {
    let (w, x, y, z) = *pos; //you can use any x y z time arrangement you like
    if w < 0i32 {
        return true;
    }
    if x < 0i32 {
        return true;
    }
    if y < 0i32 {
        return true;
    }
    if z < 0i32 {
        return true;
    }

    if z >= (maze.len() as i32) {
        return true;
    }
    let z = z as u32;
    let z = z as usize; //Z is safe to value check

    if y >= (maze[z].len() as i32) {
        return true;
    }
    let y = y as u32;
    let y = y as usize; //Y is safe to value check

    if x >= ((maze[z])[y]).len() as i32 {
        return true;
    }
    let x = x as u32;
    let x = x as usize; //X is safe to value check

    if w >= (((maze[z])[y])[x]).len() as i32 {
        return true;
    }
    let w = w as u32;
    let w = w as usize; //t is safe to value check

    (((maze[z])[y])[x])[w]
}
///mazestate5d:  Given a boolean maze and i32 coordinates, this (always works index guarded) function returns open roads (0pen false) or blocked (b1ocked true).
pub fn mazestate5d(maze: &Vec<Vec<Vec<Vec<Vec<bool>>>>>, pos: &(i32, i32, i32, i32, i32)) -> bool {
    let (v, w, x, y, z) = *pos;
    if v < 0i32 {
        return true;
    }
    if w < 0i32 {
        return true;
    }
    if x < 0i32 {
        return true;
    }
    if y < 0i32 {
        return true;
    }
    if z < 0i32 {
        return true;
    } // i32's limit maze size to 2.1 billion by 2.1 billion blocks which is likely enough.

    if z >= (maze.len() as i32) {
        return true;
    }

    let z = z as u32;
    let z = z as usize; //Z is safe to value check

    if y >= (maze[z].len() as i32) {
        return true;
    }
    let y = y as u32;
    let y = y as usize; //Y is safe to value check

    if x >= ((maze[z])[y]).len() as i32 {
        return true;
    }
    let x = x as u32;
    let x = x as usize; //X is safe to value check

    if w >= (((maze[z])[y])[x]).len() as i32 {
        return true;
    }
    let w = w as u32;
    let w = w as usize; //w is safe to value check

    if v >= (((maze[z])[y])[x])[w].len() as i32 {
        return true;
    }
    let v = v as u32;
    let v = v as usize;
    ((((maze[z])[y])[x])[w])[v]
}

//a mazewalker, spawn a player every time there is more than 1 path, players die if they can't go onward or meet player footsteps
#[derive(Debug, Clone)]
pub struct Mazewalker {
    position_vec: Vec<usize>, //player's current posisiton as xy, xyz or xyzt coordinates.
    move_history: Vec<usize>, // history of index values for each allowed move along a path so if allowed moves were vec![north, south, east, west] an player went North North West maze path index should be vec![0,0,3]
}
fn sketch(boolvec: &Vec<Vec<bool>>) {
    for yrow in boolvec {
        for x in yrow {
            if *x {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

///Maze2dpath - feed it a maze Vec<Vec<bool>>, x y axis movement rules (like nw, ne, south), entrance and exit coordinates and it should return  Some(one of the very fastest
///paths through the maze), None for no path, and a empty path vec for a entrance and exit that are the same.  Decode the steps taken on the path with the allowed move list.
///(If your move list is vec!(north, south, east, west, upside_down), a path of 0, 0, 2, 4 would be 'north, north, east, upside down")    
pub fn maze2dpath(
    in_maze: &Vec<Vec<bool>>,
    moverules: &AllowedMoves2D,
    entrancepointxy: &(usize, usize),
    exitpointxy: &(usize, usize),
) -> Option<Vec<usize>> {
    let mut maze = in_maze.clone();
    let mut position_vec: Vec<usize> = vec![entrancepointxy.0, entrancepointxy.1];
    let mut move_history: Vec<usize> = vec![];
    let mut players: Vec<Mazewalker> = vec![Mazewalker {
        position_vec,
        move_history,
    }];
    let mut newplayers = vec![];
    let (exitx, exity) = exitpointxy;

    loop {
        for player in players.iter() {
            let (px, py) = (player.position_vec[0], player.position_vec[1]);
            if (px == *exitx) && (py == *exity) {
                let out = player.move_history.clone();
                return Some(out);
            } //Found Exit, first one prints player path}

            (maze[py])[px] = true; // the player's leave footsteps

            for (i, dance) in moverules.rules.iter().enumerate() {
                //adjust player px py location to possible move loc}
                let (offsetx, offsety) = dance;
                let (tx, ty) = ((px as i32) + offsetx, (py as i32) + offsety);
                let iswall = mazestate2d(&maze, &(tx, ty));
                if iswall {
                }
                // nothing to do for walls
                else {
                    //create a new player for each step along the path
                    (maze[ty as usize])[tx as usize] = true; //new player leaves footprints
                    let mut move_history = player.move_history.clone();
                    move_history.push(i);
                    let position_vec = vec![tx as usize, ty as usize];
                    newplayers.push(Mazewalker {
                        position_vec,
                        move_history,
                    });
                } //endelse
            } //end for i dance
        } //end for players
        if newplayers.len() == 0 {
            return None; //no path through maze
        } else {
            players = newplayers;
            newplayers = vec![]; //empty vec & try again
        }
    } //loop
}

///Maze3dpath - feed it a maze Vec<Vec<Vec<bool>>>, x y z axis movement rules (like over under around and through), entrance and exit coordinates and it should return
/// Some(one of the very fastest paths through the maze), None for no path, or a empty path vec for a entrance and exit that are the same.
///Decode the steps taken on the path with the allowed move list.
///(If your move list is vec!(over, under, around, through), a path of 0, 0, 3 would be 'over over through")
pub fn maze3dpath(
    //Lee algorithm, otherwise known as only the fastest flood fill path survives
    in_maze: &Vec<Vec<Vec<bool>>>,
    moverules: &AllowedMoves3D,
    entrancepoint: &(usize, usize, usize),
    exitpoint: &(usize, usize, usize),
) -> Option<Vec<usize>> {
    let mut maze = in_maze.clone();

    //Case entrance is exit, solution involves no movement
    if *entrancepoint == *exitpoint {
        return Some(vec![]);
    }

    let mut position_vec: Vec<usize> = vec![entrancepoint.0, entrancepoint.1, entrancepoint.2];
    let mut move_history: Vec<usize> = vec![];
    let mut players: Vec<Mazewalker> = vec![Mazewalker {
        position_vec,
        move_history,
    }];

    let mut newplayers = vec![];
    let (entrancex, entrancey, entrancez) = *entrancepoint;
    {
        ((maze[entrancez])[entrancey])[entrancex] = true;
    }

    loop {
        for player in players.iter() {
            let spot = (
                player.position_vec[0],
                player.position_vec[1],
                player.position_vec[2],
            );
            //Found Exit, first one prints player path}

            for (i, step) in moverules.rules.iter().enumerate() {
                //adjust player px py location to possible move loc}
                let (sx, sy, sz) = step; //step offsets from matrix moverules
                let px = spot.0 as i32; //p for player
                let py = spot.1 as i32;
                let pz = spot.2 as i32;
                let (tx, ty, tz) = (px + sx, py + sy, pz + sz);

                let iswall = mazestate3d(&maze, &(tx, ty, tz));
                //any vector out of bounds become a wall in mazestate3d
                if iswall {
                }
                // nothing to do for walls
                else {
                    //create a new player for each step along the path
                    ((maze[tz as usize])[ty as usize])[tx as usize] = true; //new player leaves footprints
                    let mut move_history = player.move_history.clone();
                    move_history.push(i);

                    let position_vec = vec![tx as usize, ty as usize, tz as usize];
                    if (tx as usize, ty as usize, tz as usize) == *exitpoint {
                        return Some(move_history);
                    }
                    newplayers.push(Mazewalker {
                        position_vec,
                        move_history,
                    });
                } //endelse
            } //end for i step
        } //end for players
        if newplayers.len() == 0 {
            return None; //no path through maze found
        } else {
            players = newplayers;
            newplayers = vec![];
        }
    } //end while loop
    return None;
}

///Maze4dpath - feed it a maze Vec<Vec<Vec<Vec<bool>>>>, w x y z axis movement rules (like (0,0,1,1) , (0,0,1,-1) for northeast and northwest), entrance and exit coordinates
/// and it should return  Some(one of the very fastest paths through the maze), None for no path found, and a empty path vec for a entrance and exit that are the same.
/// Decode the steps taken on the path with the allowed move list.
///(If your move list was vec!(north, south, east, west, night, day, up, down), a path of 0, 0, 2, 4, 7 would be 'north, north, east, night, down")
pub fn maze4dpath(
    //Lee algorithm, otherwise known as only the fastest flood fill path survives... at least I
    //think so, the geometry of higher dimensional spaces isn't intuitive and it is possible that Lee wasn't thinking about solving mazes for time travelers.
    //One can think of a 4D maze as moving across lanes of moving trafic to catch an exit (one way move rules)
    in_maze: &Vec<Vec<Vec<Vec<bool>>>>,
    moverules: &AllowedMoves4D,
    entrancepoint: &(usize, usize, usize, usize),
    exitpoint: &(usize, usize, usize, usize),
) -> Option<Vec<usize>> {
    let mut maze = in_maze.clone();

    //Case entrance is exit, solution involves no movement
    if *entrancepoint == *exitpoint {
        return Some(vec![]);
    }

    let mut position_vec: Vec<usize> = vec![
        entrancepoint.0,
        entrancepoint.1,
        entrancepoint.2,
        entrancepoint.3,
    ];
    let mut move_history: Vec<usize> = vec![];
    let mut players: Vec<Mazewalker> = vec![Mazewalker {
        position_vec,
        move_history,
    }];

    let mut newplayers = vec![];

    let (w, x, y, z) = *entrancepoint;
    {
        (((maze[z])[y])[x])[w] = true;
    }

    loop {
        for player in players.iter() {
            let spot = (
                player.position_vec[0],
                player.position_vec[1],
                player.position_vec[2],
                player.position_vec[3],
            );
            //Found Exit, first one prints player path}

            for (i, step) in moverules.rules.iter().enumerate() {
                //adjust player px py location to possible move loc}
                let (sw, sx, sy, sz) = step; //step offsets from matrix moverules
                let pw = spot.0 as i32;
                let px = spot.1 as i32; //p for player
                let py = spot.2 as i32;
                let pz = spot.3 as i32;
                let (tw, tx, ty, tz) = (pw + sw, px + sx, py + sy, pz + sz);

                let iswall = mazestate4d(&maze, &(tw, tx, ty, tz));
                //any vector out of bounds become a wall in mazestate3d
                if iswall {
                }
                // nothing to do for walls
                else {
                    //create a new player for each step along the path
                    (((maze[tz as usize])[ty as usize])[tx as usize])[tw as usize] = true; //new player leaves footprints
                    let mut move_history = player.move_history.clone();
                    move_history.push(i);

                    let position_vec = vec![tw as usize, tx as usize, ty as usize, tz as usize];
                    if (tw as usize, tx as usize, ty as usize, tz as usize) == *exitpoint {
                        return Some(move_history);
                    }
                    newplayers.push(Mazewalker {
                        position_vec,
                        move_history,
                    });
                } //endelse
            } //end for i step
        } //end for players
        if newplayers.len() == 0 {
            return None; //no path through maze found
        } else {
            players = newplayers;
            newplayers = vec![];
        }
    } //end while loop
}

///Maze5dpath - feed it a maze Vec<Vec<Vec<Vec<Vec<bool>>>>>, v w x y z axis movement rules (like (0,0,0,0,-1) for "upside down?"), entrance and exit coordinates
/// and it should return  Some(one of the very fastest paths through the maze), None() for no path found, and Some(empty vec) for a entrance and exit that are the same.
/// Decode the steps taken on the path with the allowed move list.
/// If the moves list was vec!(north, south, east, west, night, day, up, down, action_lever), a path of 0, 2, 4, 8 would be 'north, east, nighttime, action_lever"
pub fn maze5dpath(
    //Lee algorithm, otherwise known as only the fastest flood fill path survives... at least I
    //think so, the geometry of five dimensional spaces isn't obvious
    // If one can think 5d maze as morning commute, (a maze of moving cars), repleat with a sprinkling of unpredictable crazy drivers -
    in_maze: &Vec<Vec<Vec<Vec<Vec<bool>>>>>,
    moverules: &AllowedMoves5D,
    entrancepoint: &(usize, usize, usize, usize, usize),
    exitpoint: &(usize, usize, usize, usize, usize),
) -> Option<Vec<usize>> {
    let mut maze = in_maze.clone();

    //Case entrance is exit, solution involves no movement
    if *entrancepoint == *exitpoint {
        return Some(vec![]);
    }

    let mut position_vec: Vec<usize> = vec![
        entrancepoint.0,
        entrancepoint.1,
        entrancepoint.2,
        entrancepoint.3,
        entrancepoint.4,
    ];
    let mut move_history: Vec<usize> = vec![];
    let mut players: Vec<Mazewalker> = vec![Mazewalker {
        position_vec,
        move_history,
    }];

    let mut newplayers = vec![];

    let (v, w, x, y, z) = *entrancepoint;
    {
        ((((maze[z])[y])[x])[w])[v] = true;
    }

    loop {
        for player in players.iter() {
            let spot = (
                player.position_vec[0],
                player.position_vec[1],
                player.position_vec[2],
                player.position_vec[3],
                player.position_vec[4],
            );
            //Found Exit, first one prints player path}

            for (i, step) in moverules.rules.iter().enumerate() {
                //adjust player px py location to possible move loc}
                let (sv, sw, sx, sy, sz) = step; //step offsets from matrix moverules
                let pv = spot.0 as i32;
                let pw = spot.1 as i32;
                let px = spot.2 as i32; //p for player
                let py = spot.3 as i32;
                let pz = spot.4 as i32;
                let (tv, tw, tx, ty, tz) = (pv + sv, pw + sw, px + sx, py + sy, pz + sz);

                let iswall = mazestate5d(&maze, &(tv, tw, tx, ty, tz));
                //any vector out of bounds become a wall in mazestate3d
                if iswall {
                }
                // nothing to do for walls
                // nothing to do for walls
                else {
                    //create a new player for each step along the path
                    ((((maze[tz as usize])[ty as usize])[tx as usize])[tw as usize])[tv as usize] =
                        true; //new player leaves footprints
                    let mut move_history = player.move_history.clone();
                    move_history.push(i);

                    let position_vec = vec![
                        tv as usize,
                        tw as usize,
                        tx as usize,
                        ty as usize,
                        tz as usize,
                    ];
                    if (
                        tv as usize,
                        tw as usize,
                        tx as usize,
                        ty as usize,
                        tz as usize,
                    ) == *exitpoint
                    {
                        return Some(move_history);
                    }
                    newplayers.push(Mazewalker {
                        position_vec,
                        move_history,
                    });
                } //endelse
            } //end for i step
        } //end for players
        if newplayers.len() == 0 {
            return None; //no path through maze found
        } else {
            players = newplayers;
            newplayers = vec![];
        }
    } //end while loop
}
