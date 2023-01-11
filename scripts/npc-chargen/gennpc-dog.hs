{-# LANGUAGE OverloadedStrings #-}
import qualified Data.Text as T
import qualified Data.Text.IO as T
import Control.Monad
import System.Random

rooms :: [T.Text]
rooms = [
          "melbs_kingst_latrobest",
          "melbs_kingst_10",
          "melbs_kingst_20",
          "melbs_kingst_30",
          "melbs_kingst_lonsdalest",
          "melbs_kingst_40",
          "melbs_homelessshelter",
          "melbs_kingst_50",
          "melbs_kingst_60",
          "melbs_kingst_bourkest",
          "melbs_kingst_70",
          "melbs_kingst_80",
          "melbs_kingst_90",
          "melbs_kingst_collinsst",
          "melbs_kingst_100",
          "melbs_kingst_110",
          "melbs_kingst_120",
          "melbs_kingst_flinderst",
          "melbs_flindersst_210",
          "melbs_flindersst_200",
          "melbs_flindersst_190",
          "melbs_williamsst_flindersst",
          "melbs_flindersst_180",
          "melbs_flindersst_170",
          "melbs_flindersst_160",
          "melbs_queenst_flindersst",
          "melbs_flindersst_150",
          "melbs_flindersst_140",
          "melbs_flindersst_130",
          "melbs_elizabethst_flindersst",
          "melbs_flindersst_120",
          "melbs_flindersst_110",
          "melbs_flindersst_100",
          "melbs_swanstonst_flindersst",
          "melbs_swanstonst_latrobest",
          "melbs_swansonst_10",
          "melbs_swanstonst_20",
          "melbs_swanstonst_30",
          "melbs_swanstonst_lonsdalest",
          "melbs_swanstonst_40",
          "melbs_swanstonst_50",
          "melbs_swanstonst_60",
          "melbs_swanstonst_bourkest",
          "melbs_swanstonst_70",
          "melbs_swanstonst_80",
          "melbs_swanstonst_90",
          "melbs_swanstonst_collinsst",
          "melbs_swanstonst_100",
          "melbs_swanstonst_110",
          "melbs_swanstonst_120",
          "melbs_latrobest_210",
          "melbs_latrobesst_200",
          "melbs_latrobest_190",
          "melbs_williamstlatrobest",
          "melbs_latrobest_180",
          "melbs_latrobest_170",
          "melbs_latrobest_160",
          "melbs_queenst_latrobest",
          "melbs_latrobest_150",
          "melbs_latrobest_140",
          "melbs_latrobest_130",
          "melbs_elizabethst_latrobest",
          "melbs_latrobest_120",
          "melbs_latrobest_110",
          "melbs_latrobest_100",
          "melbs_lonsdalest_210",
          "melbs_lonsdalest_200",
          "melbs_lonsdalest_190",
          "melbs_williamstlonsdalest",
          "melbs_lonsdalest_180",
          "melbs_lonsdalest_170",
          "melbs_lonsdalest_160",
          "melbs_queenst_lonsdalest",
          "melbs_lonsdalest_150",
          "melbs_lonsdalest_140",
          "melbs_lonsdalest_130",
          "melbs_elizabethst_lonsdalest",
          "melbs_lonsdalest_120",
          "melbs_lonsdalest_110",
          "melbs_lonsdalest_100",
          "melbs_williamsst_10",
          "melbs_williamsst_20",
          "melbs_williamsst_30",
          "melbs_williamsst_40",
          "melbs_williamsst_50",
          "melbs_williamsst_60",
          "melbs_williamsst_bourkest",
          "melbs_williamsst_70",
          "melbs_williamsst_80",
          "melbs_williamsst_90",
          "melbs_williamsst_collinsst",
          "melbs_williamsst_100",
          "melbs_williamsst_110",
          "melbs_williamsst_120",
          "melbs_bourkest_210",
          "melbs_bourkest_200",
          "melbs_bourkest_190",
          "melbs_bourkest_180",
          "melbs_bourkest_170",
          "melbs_bourkest_160",
          "melbs_queenst_bourkest",
          "melbs_bourkest_150",
          "melbs_bourkest_140",
          "melbs_bourkest_130",
          "melbs_elizabethst_bourkest",
          "melbs_bourkest_120",
          "melbs_bourkest_110",
          "melbs_bourkest_100",
          "melbs_queenst_10",
          "melbs_queenst_20",
          "melbs_queenst_30",
          "melbs_queenst_40",
          "melbs_queenst_50",
          "melbs_queenst_60",
          "melbs_queenst_70",
          "melbs_queenst_80",
          "melbs_queenst_90",
          "melbs_queenst_collinsst",
          "melbs_queenst_100",
          "melbs_queenst_110",
          "melbs_queenst_120",
          "melbs_collinsst_210",
          "melbs_collinsst_200",
          "melbs_collinsst_190",
          "melbs_collinsst_180",
          "melbs_collinsst_170",
          "melbs_collinsst_160",
          "melbs_collinsst_150",
          "melbs_collinsst_140",
          "melbs_collinsst_130",
          "melbs_elizabethst_collinsst",
          "melbs_collinsst_120",
          "melbs_collinsst_110",
          "melbs_collinsst_100",
          "melbs_elizabethst_10",
          "melbs_elizabethst_20",
          "melbs_elizabethst_30",
          "melbs_elizabethst_40",
          "melbs_elizabethst_50",
          "melbs_elizabethst_60",
          "melbs_elizabethst_70",
          "melbs_elizabethst_80",
          "melbs_elizabethst_90",
          "melbs_elizabethst_100",
          "melbs_elizabethst_110",
          "melbs_elizabethst_120"
        ]

character :: Int -> T.Text -> T.Text -> T.Text
character id adjective room =
  "      NPC {\n\
  \          code: \"melbs_dog_" <> (T.pack $ show id) <> "\",\n\
  \          name: \"" <> adjective <> " dog\",\n\
  \          proper_noun: false,\n\
  \          description: \"A malnourished looking dog. Its skeleton is visible through its thin and patchy fur. It smells terrible, and certainly doesn't look tame.\",\n\
  \          spawn_location: \"room/" <> room <> "\",\n\
  \          ..Default::default()\n\
  \      },\n"

chooseFromList :: [a] -> IO a
chooseFromList l = (l!!) <$> randomRIO (0, length l - 1)
  
firstAdjectives :: [T.Text]
firstAdjectives = ["mangy", "smelly", "feral", "ferocious", "howling", "growling", "reeking"]

secondAdjectives :: [T.Text]
secondAdjectives = ["brown", "black", "white", "grey", "light brown"]
  
main :: IO ()
main = forM_ [1..60] $ \id -> do
  adjective1 <- chooseFromList firstAdjectives
  adjective2 <- chooseFromList secondAdjectives
  room <- chooseFromList rooms
  T.putStr $ character id (adjective1 <> " " <> adjective2) room
