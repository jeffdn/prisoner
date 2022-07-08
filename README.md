# Prisoner's riddle

## Description

This is a riddle that I learned about from the Veritasium channel on YouTube. I found
it very compelling, and so thought that it would be interesting to implement a proof of
the described method for astronomically improving one's odds.

See [here](https://www.youtube.com/watch?v=iSNsgj1OCLA) for the video.

## Riddle

There are 100 prisoners. They are given an opportunity to be released. The conditions of
this release are as follows:
 - each prisoner is assigned a unique number, and this number is written on a slip of
   paper
 - in a room, there are 100 boxes, and those slips are distributed amongst the boxes
   randomly
 - each prisoner must go into the room alone, and can open 50 boxes -- if they find the
   box with their number in it, they are a winner
 - if all 100 prisoners each find the box with their number in it, all of the prisoners
   are freed -- but if even one fails, they are all executed
 - the prisoners are allowed to coordinate a strategy before the game begins

## Solution

The premise of the solution is that each prisoner should start by opening the box with
their number on it. If that does not contain the slip with their number, they are to
open the box with the same number as that slip. They are to repeat the exercise until
they find their number. These chains of numbers will be called "loops" -- given that
each slip's number is unique and each box's number is unique, there will necessarily be
one way to start with a given box and end up with the box containing a slip with the
initial box's number. For example, given a setup with only five numbers:

       0   1   2   3   4   5   6   7   8   9
      +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+
      |4| |3| |9| |2| |7| |8| |6| |5| |0| |1|
      +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+

      In this case, let's start with prisoner #1. This prisoner will
      go to box #1 first. They will then go to box #3, then #2, #9,
      and then they will have found their number. There are three total
      loops in this set:

          0 -> 4 -> 7 -> 5 -> 8
          ^                   |
          +-------------------+

          1 -> 3 -> 2 -> 9
          ^              |
          +--------------+

          6 -+
          ^  |
          +--+

In just over 31% of cases, all prisoners will be able to find their number without
needing to open more than 50 boxes. In the other ~69% of cases more than half of the
prisoners will not be able to find their number.
