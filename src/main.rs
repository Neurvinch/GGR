// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// pub struct Guess {
//     value: i32,
// }

//  impl Guess {

//      pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!(
//                 "The secret number will be between 1 and 100, got {}",
//                 value
//             );
//          }
//          Guess {value}
//         }

//         pub fn value(&self) -> i32 {
//             self.value
//          }    
        
        
//  }


//  fn main () {
//      println!(" Guess the number!");

//      let secret_number = rand::rng().random_range(1..101);

//      loop {
//          println!("Please input your guess. ");
//          let mut guess = String::new();
//          io::stdin()
//              .read_line(&mut guess)
//              .expect("Failed to read line");
        
//         let guess: i32 = match guess.trim().parse() {
//             Ok(num ) => num,
//             Err(_) => continue,
//         }  ;   
      

//          Guess::new(guess);
//          println!("You guessed: {}", guess);
//          match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//          }

//      }
//  }
  
  use axum::{
    extract::State,
    routing:: {get,post},
    Json,Router
  };


   use rand::Rng;
   use serde::{Deserialize, Serialize};

   use std::{net::SocketAddr, sync::{Arc, Mutex}};

   #[derive(Clone)] 
   struct AppState {
    secret_number : Arc<Mutex<i32>>,
   }  
   
   #[derive(Deserialize)] // json into struct
   struct GuessRequest {
    guess : i32,
   }

   #[derive(Serialize)] // struct into json 
   struct GuessResponse {
    result: String,
   }

   async fn get_secret(State(state) : State<AppState>) -> Json<i32> {

    let secret = *state.secret_number.lock().unwrap();
    Json(secret)
   }

   async fn post_guess (
                   
    State(state): State<AppState>,
    Json(payload): Json<GuessRequest>
   ) -> Json<GuessResponse> {
     
    let secret = *state.secret_number.lock().unwrap();

      let result = if payload.guess < secret {
          "Too small!".to_string()
      } else if payload.guess > secret {
          "Too big!".to_string()
      } else {
          "You win!".to_string()
      };

      Json(GuessResponse {result})
   }


   #[tokio::main]
   async fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number: {}", secret_number);

    let state = AppState {
       secret_number:Arc::new(mutex::new(secret_number))
    };
   
      let app = Router::new()
      .route("/guess", post(post_guess))
      .route("/secret", get(get_secret))
      .with_state(state);

      println!("Listening on {}", addr);


      axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap();          

   }


   


   


