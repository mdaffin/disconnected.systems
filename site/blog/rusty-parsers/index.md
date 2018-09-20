Recently I started a project where I needed to parse some text in the form of a hardware discription language (a way of describing how logic gates are connected on a chip). Nothing very complex as far as parsers are conserned, quite short input and a quite simple structure - quite liekly something that I could do by hand but I wanted to take this as an oppotinity to learn some of rusts parsing libraries.

I first tried nom, being one of the more popular libraries that lots of people seemed to rave about. I hit quite a few issues, and decided to take a step back and look at pest, another popular library. Quite quickly I ran into some other issues. I relised that this was not going to be as simple as I first thourght and decided to learn a selection of different libraries out there to compare and contrast them. I went back to nom, taking a subset of the full language and started to learn nom in more details, push through the problems and get a complete solution to my partital problem. Then repeate this process for pest, and a selection of other popular or intresting parser libraries.

As part of this I am looking at erganomics over preformance, there are other resources out there that benchmark the various libraries (and all seem to come to different conslusions). But I want to know about the developers side, how well are they documented? How nice are they to use? What are the best practices of using them? How well do they handle errors? In this series I will be attempting to answer these questions as well as gain a better understanding of parsing in general and create some minimal - but complete - examples for each of the libraries (as I found a lot of partial example and very few that tie everything togeather that were not hugly complex).

The language I will be parsing is a subset of a HDL language that is used by the [nand2tetris](https://www.nand2tetris.org/) course. The full HDL looks like:

```
CHIP Xor {
    IN a, b;
    OUT out;
    PARTS:
    Not(in=a, out=nota);
    Not(in=b, out=notb);
    And(a=a, b=notb, out=w1);
    And(a=nota, b=b, out=w2);
    Or(a=w1, b=w2, out=out);
}
```

But for this exersise I am only going to be implementing a line from the PARTS subsection, such as `Not(in=a, out=nota);`. This takes the form `<chip_name> ( <internal_wire>=<outter_wire>, <internal_wire>=<outter_wire> );`. White space can exist between any of the tokens and there can be any number of inner/outter wire pairs. For this we are going to ignore comments.