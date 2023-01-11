module Main where
import Data.List
import System.Random
import Control.Monad

first :: [String]
first = [
  "James",
  "Mary",
  "Robert",
  "Patricia",
  "John",
  "Jennifer",
  "Michael",
  "Linda",
  "David",
  "Elizabeth",
  "William",
  "Barbara",
  "Richard",
  "Susan",
  "Joseph",
  "Jessica",
  "Thomas",
  "Sarah",
  "Charles",
  "Karen",
  "Christopher",
  "Lisa",
  "Daniel",
  "Nancy",
  "Matthew",
  "Betty",
  "Anthony",
  "Margaret",
  "Mark",
  "Sandra",
  "Donald",
  "Ashley",
  "Steven",
  "Kimberly",
  "Paul",
  "Emily",
  "Andrew",
  "Donna",
  "Joshua"
      ]
      
surn :: [String]
surn = [
           "Smith",
           "Johnson",
           "Williams",
           "Brown",
           "Jones",
           "Garcia",
           "Miller",
           "Davis",
           "Rodriguez",
           "Martinez",
           "Hernandez",
           "Lopez",
           "Gonzalez",
           "Wilson",
           "Anderson",
           "Thomas",
           "Taylor",
           "Moore",
           "Jackson",
           "Martin",
           "Lee",
           "Perez",
           "Thompson",
           "Harris",
           "Sanchez",
           "Clark",
           "Ramirez",
           "Lewis",
           "Robinson",
           "Walker",
           "Young",
           "Allen",
           "King",
           "Wright",
           "Scott",
           "Torres",
           "Nguyen",
           "Hill",
           "Flores",
           "Green",
           "Adams"
          ]

main :: IO ()
main = do
  forM_ [1..60] $ \i -> do
    first_name <- (first!!) <$> randomRIO (0, length first - 1)
    last_name <- (surn!!) <$> randomRIO (0, length surn - 1)
    putStrLn $ "        NPC {\n            code: \"melbs_citizen_" <> (show i) <> "\",\n            name: \"" <> first_name <> " " <> last_name <> "\n            description: \"A fairly ordinary looking citizen of Melbs, clearly weary from the harst reality of post-apocalyptic life\",\n            spawn_location: \"room/melbs_\"\n            message_handler: None,\n            says: vec!()\n        },\n"
