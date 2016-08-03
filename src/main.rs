use std::io;
//use std::io::Error;

extern crate rand;


enum RPS {
	Rock,
	Paper,
	Scissors,
}

//create GameStats
struct GameStats {
	gamesplayed: i32,
	win_total: i32, 
	win_pct: f32,
    tie_total: i32,
	tie_pct: f32,
    lose_total: i32, 
	lose_pct: f32,
    rock_total: i32,
    paper_total: i32,
    scissor_total: i32,
}

//function to determine choice based on input in:String out:choice
fn getplrchoice (input: i32) -> RPS {
	let choice: RPS;
	match input {
		1 => choice = RPS::Rock,
		2 => choice = RPS::Paper,
		3 => choice = RPS::Scissors,	
		_ => choice = RPS::Scissors,
	}
	return choice;
}

//checking for a win function
//return 1, 2, or 3 (win, loss, or tie)
fn checkforwin (plrchoice: RPS, oppchoice: RPS) -> i32 {
	let plr: i32;
	let opp: i32;
	let mut sum: i32;
	let mut sum2: i32;
	match plrchoice { 
		RPS::Rock => plr=1,
		RPS::Paper => plr=2,
		RPS::Scissors => plr=3,
	}
	match oppchoice {
		RPS::Rock => opp=1,
		RPS::Paper => opp=2,
		RPS::Scissors => opp=3,
	}	
	sum=opp-plr;
	sum2=plr-opp;
	match sum {
		1 => return 2, //plr loses
		2 => return 1, //plr wins
		0 => return 3, //plr ties
		_ => sum = 3,
	}
	match sum2 {
		1 => return 1, //plr wins
		2 => return 2, //plr loses
		0 => return 3, //plr ties
		_ => sum2 = 3,
	}
	return sum2-sum;
}

//calculate stats function 
fn stats(mut gs: &mut GameStats, winlosstie: i32, plrchoice: i32) {
	gs.gamesplayed+=1;
	match winlosstie {
		1 => gs.win_total+=1,
		2 => gs.lose_total+=1,
		3 => gs.tie_total+=1,
		_ => gs.tie_total+=1,
	}
	gs.win_pct = (gs.win_total as f32)/(gs.gamesplayed as f32);
	gs.lose_pct = (gs.lose_total as f32)/(gs.gamesplayed as f32);
	gs.tie_pct = (gs.tie_total as f32)/(gs.gamesplayed as f32);
	match plrchoice {
		1 => gs.rock_total+=1,
		2 => gs.paper_total+=1,
		3 => gs.scissor_total+=1,
		_ => gs.scissor_total+=1,
	}
}

//display stats function 
fn displaystats(gs: GameStats) {
	//display stats after quit
    println!("Player Stats:");
    println!("Wins: {} ({:.2}%)", gs.win_total, gs.win_pct);
    println!("Ties: {} ({:.2}%)", gs.tie_total, gs.tie_pct);
    println!("Losses: {} ({:.2}%)", gs.lose_total, gs.lose_pct);
    println!("Rocks: {}", gs.rock_total);
    println!("Papers: {}", gs.paper_total);
    println!("Scissors: {}", gs.scissor_total);
    println!("");
    println!("---------------");
    println!("");
}

fn resetstats(mut gs: &mut GameStats) {
	gs.gamesplayed=0;
	gs.win_total=0; 
	gs.win_pct=0.0;
    gs.tie_total=0;
	gs.tie_pct=0.0;
    gs.lose_total=0; 
	gs.lose_pct=0.0;
    gs.rock_total=0;
    gs.paper_total=0;
    gs.scissor_total=0;
}

fn main() {
	use rand::Rng;
	let mut gs = GameStats {gamesplayed: 0,
	win_total: 0, 
	win_pct: 0.0,
    tie_total: 0,
	tie_pct: 0.0,
    lose_total: 0, 
	lose_pct: 0.0,
    rock_total: 0,
    paper_total: 0,
    scissor_total: 0};
	let mut input = String::from("Hello, world!");
	let str_r = String::from("r");
//	let plrchoice: RPS;
//	let mut oppchoice: RPS;
//	let mut done: bool;
//	let mut quit: bool;
	let mut done = false; // mut done: bool
	let mut quit = false;
//	let mut winlosstie: i32;
	let mut randomnumber: i32;
	let mut plrchoicenum: i32 = 0;
	resetstats(&mut gs);
	//game loop
	while !quit {
		//input loop
		while !done {
			
			println!("Enter choice (r,p,s) or q quit >");
			io::stdin().read_line(&mut input).expect("FAIL"); //any errors should use
			if input==str_r {
				done = true;
				plrchoicenum=1;
			} else if input.starts_with("p") {
				done = true;
				plrchoicenum=2;
			} else if input.starts_with("s") {
				done = true;
				plrchoicenum=3;
			} else if input.starts_with("q") {
				done = true;
				quit = true;
				break;
			} else {
				println!("Invalid input. Please enter (r,p,s, or q to quit).",);
			}
		}
		if !quit {
			//function to get choice from input
			let plrchoice = getplrchoice(plrchoicenum);
			randomnumber = rand::thread_rng().gen_range(1, 3);
			let oppchoice = getplrchoice(randomnumber);
			match plrchoice {
				RPS::Rock => println!("Player chose: Rock"),
				RPS::Paper => println!("Player chose: Paper"),
				RPS::Scissors => println!("Player chose: Scissors"),
			}
			match oppchoice {
				RPS::Rock => println!("Opponent chose: Rock"),
				RPS::Paper => println!("Opponent chose: Paper"),
				RPS::Scissors => println!("Opponent chose: Scissors"),
			}
			//win or lose or tie
			let winlosstie = checkforwin(plrchoice, oppchoice);
			
			//display win loss or tie
			match winlosstie {
				1 => println!("You win!"),
				2 => println!("You lose!"),
				3 => println!("It's a tie!"),
				_ => println!("ERROR :-o"),
			}			
			stats(&mut gs, winlosstie, plrchoicenum);
		}
	}	
	//after quit
	displaystats(gs);
}
