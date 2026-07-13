---
title: "One Must Imagine Sisyphus at the Slot Machine."
date: "2026-07-14"
author: "Mega Nugraha"
summary: "The daily ritual of pasting specs into an LLM, accepting generated code without understanding it, and slowly watching a once-competent engineer get replaced by a very fast typist. A meditation on what happens when the work feels less like pushing a boulder and more like pulling a lever."
tags: ["AI", "LLM", "Software Engineering", "Cognitive Debt", "Philosophy"]
published: true
---

It is 9:02 a.m. when I open a spec written by someone who has never opened a terminal, paste it into a model that has never shipped anything, and wait for the two of them to negotiate a contract neither party intends to honor. This is called "velocity."

I would like to tell you I once wanted to be an engineer. The kind of developer who draws a system on a whiteboard before writing a line of it, who fires up a perfectly tuned LazyVim environment and names a variable like he intends to see it again. I still remember the shape of that ambition. It is preserved the way a saint's finger bone is preserved: reverently, uselessly, in a box, for people to look at and feel something.

What I am instead is a man staring at a ticket that has been sitting in the TO-DO column since yesterday, feeding a machine that produces plausible Go faster than any compiler can be bothered to argue with it.

While the cursor blinks and the machine hums, I try not to think about what is actually happening underneath the chat window. There is no engineer in there. There is a statistical engine that has read every line of code ever pushed to a public remote, and its entire personality is a very expensive bet on what token comes next, over and over, thousands of times a second. It has no more understanding of `context.Context` than a slot machine has an understanding of cherries. It is not reasoning about my gRPC boundary. It is completing a pattern that resembles reasoning about my gRPC boundary. And then it starts folding its own last guess back into the next one, so that three prompts deep I am no longer reviewing a colleague's work—I am watching a coin land on the last coin's outcome and calling that a streak.

I know this, clearly, stone sober, before my coffee is even done. But I pull the lever anyway, because management has made it extremely clear that moving tickets to the DONE column is a personality trait worth having, and because some tiny, degraded part of me has already decided that this prompt is going to land on cherries.

**I press Enter.**

The code generates. It looks like code.

Before I can even read the first function, a Slack notification slides into the top right of my screen. It's the sprint board. The ticket has been "In Progress" for four days, which is, apparently, a geological era.

_Ping._ Another notification. It's the frontend guy, three desks over. He has his own chat window open, cooking his own confident nonsense, and he needs my JSON schema the way a man overboard needs a rope—immediately, without commentary on the rope's construction.

I don't have time to read the AI's logic. I am squeezed between the relentless mechanical heartbeat of the sprint and a blocked coworker. I glance at the generated output. The model is very confident. It is confident the way a man who has never played chess is confident he can beat a grandmaster, because confidence, unlike skill, is free to generate.

I click _Accept_.

And right then, underneath the noise of the office—and this is the one nobody puts on the org chart—a small, patient ghost who lives somewhere behind my sternum leans in. _You used to be able to do this yourself,_ he says, very calmly.

He wasn't some mythic savant. He wrote arrogant code, made bad architectural bets, and spent half his nights brute-forcing his way out of his own mistakes. But he was imperfect in a way I respected. When he broke the build, he bled for it himself. More importantly, he had a habit of walking toward whatever had just exposed the limits of his understanding.

He doesn't shout. He's not dramatic about it. He just keeps a ledger. Every accepted suggestion I couldn't defend if you woke me at 3 a.m. and made me—he adds it to the pile, in his own quiet handwriting, and says nothing further. He was the one, two years ago, who could perfectly map how a payload hits which part of the code, which part of the network layer, which part of the handler, armed only with a whiteboard and pure spite in front of a classroom full of people.

He has watched that man get replaced, one _Accept_ button at a time, by a very fast typist with excellent instincts for what looks broadly correct. He is not, technically, wrong about any of it. That's the part I can't forgive him for.

I hit the PR button.

At 10:00 a.m., the daily standup begins. This is the part of the morning where I am required to act as defense counsel for a statistical anomaly.

The tech lead looks at the board, sees my ticket moved to 'Review', and casually asks why I chose a specific fan-out pattern for the data fetching.

I don't know. I have absolutely no idea. The machine chose it because it was probabilistically likely based on a tutorial written in 2019. But I cannot say that, because it violates the terms of our collective illusion.

So I unmute. I clear my throat. I deploy words like "extensibility" and "payload optimization." I construct a desperate, post-hoc rationalization for an architectural decision that was never actually made by a thinking mind. **I talk for two unbroken minutes.** I am sweating, tap-dancing through the technical explanation, trying with every ounce of my energy to look like a man who understands the code currently bearing his name. I am giving a masterclass book report on a novel I didn't read, terrified that someone is going to ask me about the themes in chapter four.

Nobody asks.

Underneath my sternum, the ghost doesn't even look up. He just keeps tallying.

Ten minutes after the call ends, the senior reviewer descends on the pull request. Let's say, for the sake of not naming him, he is a man who has forgotten more about N+1 queries than I currently know. He starts leaving brutal, surgically precise comments on architectural choices I didn't even make.

So I do what I am told is engineering: I paste the reviewer's diff comments back into the chat box, verbatim, like feeding a horse its own manure and calling it a balanced diet, and I ask the model to "fix this." It fixes this. It also, unbidden, refactors a helper function eleven layers away that was working fine, because somewhere in its training it absorbed the idea that unrequested change signals effort. I revert the helper function. I don't tell the AI I've done this. It would only apologize, and I've run out of room to store its apologies.

I ask it to check its work. It is delighted to report that its work is excellent. I ask again, phrased more suspiciously, and it discovers—with the theatrical surprise of a man "finding" a wallet he planted himself—three bugs it invented in the first draft specifically so it could fix them in the second.

Three rounds of this. Four. The senior reviewer starts leaving shorter and shorter comments, which is not a sign of approval. It is a sign of a man conserving his remaining will to live. Eventually, he writes LGTM, all caps, capitalized like a headstone.

I click 'Squash and Merge'. **A webhook fires.** The frontend gets his rope. The board turns the color of money.

I feel absolutely nothing. Not relief. Relief requires having been afraid of losing. I was never playing to win. I was playing not to be noticed—which is, I'm told, also how you're supposed to play a slot machine, if you want to keep your seat at it.

Camus's version of this ends well for the boulder guy. He gets down the mountain, watches the rock roll to the bottom, and Camus insists that we must imagine him happy, because the plain physical fact of pushing is enough to fill a human heart to the brim.

I don't think I'm Sisyphus. Sisyphus's punishment is honest. The rock is real, the mountain is real, and every time he fails it's his failure, earned by his own hands. There's a dignity in that I don't get to have. My rock isn't a rock. It's three symbols on a wheel, and the only labor involved is pulling the same lever and pretending the outcome is up to skill.

A gambler, at least, knows exactly what he's doing when he sits down. He's traded certainty for the shape of hope, and he'll tell you so, over a drink, without much shame. I'm the gambler who tells himself, every single time, that this next pull is engineering—that reading the diff before I merge it, that asking the model to double-check itself one more time, is a skill and not a superstition, a lucky number I mutter before I press the lever down.

My ghost knows better. He's the only one at this table who was ever any good at the actual game—the one where you trace the bug by hand, where you feel around in the dark until something hurts and you fix it in nine lines nobody had to review under duress. He remembers, because he's the one who did it. And as I sit here at the machine instead of at the workbench, he doesn't say anything cruel. He just makes a small mark in his ledger, in the column that isn't TO-DO or DONE, the one only he can see.

Nobody is punishing me. I am doing this to myself, on purpose, three or four times a day, keeping a much better craftsman locked behind my own sternum to watch me do it.

One must imagine Sisyphus happy. I don't get to. I get to imagine the ghost, keeping his ledger, watching the wheel spin, already knowing which three symbols it's going to land on.

I click on the next ticket. I drag it to 'In Progress', and I pull the lever for him anyway, one more time, because the board is still red and the coffee's still warm and some tiny, degraded part of me still thinks this is the morning I quit gambling and go back to work.
